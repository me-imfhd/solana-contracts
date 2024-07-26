use std::str::FromStr;

use anchor_lang::{
    prelude::*,
    solana_program::program_option::COption,
    system_program::{ transfer, Transfer },
};
use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::spl_token_2022::onchain::invoke_transfer_checked,
    token_interface::{ thaw_account, Mint, ThawAccount, Token2022, TokenAccount },
};
use crate::{
    calculate_royalties,
    error::MarketplaceError,
    state::{
        Creator,
        DistributionAccount,
        EnforcingAccount,
        ListedNftPda,
        DISTRIBUTION_NO_CREATORS_SPACE,
    },
};

use super::{ get_metadata, update_account_lamports_to_minimum_balance, CreatorWithShare };

pub fn buy_nft(ctx: Context<BuyNft>) -> Result<()> {
    let selling_price = ctx.accounts.listed_nft_pda.price;
    let mint = ctx.accounts.mint.to_account_info();
    let royalty_amount = calculate_royalties(&mint, selling_price)?;
    let payment_amount = selling_price
        .checked_sub(royalty_amount)
        .ok_or(MarketplaceError::ArithmeticError)?;
    ctx.accounts.transfer_royalty_to_distribution_account(royalty_amount)?; // Send sol to distribution account, which can be claimed after distributing royalties.
    ctx.accounts.distribute_royalty(royalty_amount)?; // Distribute royalties
    ctx.accounts.set_enforcement()?; // Ensures transfer_hook works by requiring a valid account slot, enforced and validated at hook CPI.
    ctx.accounts.pay_and_invoke_nft_transfer(payment_amount)?; // Send sol to seller, thaw and trasfer the nft, which invokes transfer hook.
    ctx.accounts.listed_nft_pda.close(ctx.accounts.seller.to_account_info())?; // Remove from the listing.
    Ok(())
}

#[derive(Accounts)]
pub struct BuyNft<'info> {
    #[account(mut)]
    pub buyer: Signer<'info>,
    /// CHECK: validated with constraints below
    #[account(mut)]
    pub seller: SystemAccount<'info>,
    #[account(
        mut,
        associated_token::token_program = token_program,
        associated_token::mint = mint,
        associated_token::authority = seller
    )]
    pub seller_token_account: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = buyer,
        associated_token::token_program = token_program,
        associated_token::mint = mint,
        associated_token::authority = buyer
    )]
    pub buyer_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        seeds = [b"listed_nft_pda", seller.key().as_ref(), mint.key().as_ref()],
        bump,
        constraint = mint.key() == listed_nft_pda.mint && 
        seller.key() == listed_nft_pda.seller && 
        seller_token_account.key() == listed_nft_pda.seller_ata
    )]
    pub listed_nft_pda: Account<'info, ListedNftPda>,
    #[account(
        mut,
        constraint = mint.mint_authority == COption::None &&
        mint.decimals == 0 &&
        mint.supply == 1
    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    /// CHECK: Checked inside Token extensions program
    transfer_hook_program: UncheckedAccount<'info>,
    /// CHECK: Its owner is another program so can't derive it
    extra_metas_account: UncheckedAccount<'info>,
    #[account(
        init_if_needed,
        seeds = [b"enforcing_account", mint.key().as_ref()],
        bump,
        payer = buyer,
        space = 8 + EnforcingAccount::INIT_SPACE
    )]
    pub enforcing_account: Account<'info, EnforcingAccount>,
    #[account(
        mut,
        seeds = [b"distribution_account", mint.key().as_ref()],
        bump,
    )]
    pub distribution_account: Account<'info, DistributionAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

impl<'info> BuyNft<'info> {
    pub fn set_enforcement(&mut self) -> Result<()> {
        let clock = Clock::get()?;
        self.enforcing_account.slot = clock.slot;
        Ok(())
    }
    pub fn thaw_account(&self) -> Result<()> {
        thaw_account(
            CpiContext::new(self.token_program.to_account_info(), ThawAccount {
                account: self.seller_token_account.to_account_info(),
                authority: self.seller.to_account_info(),
                mint: self.mint.to_account_info(),
            })
        )?;
        Ok(())
    }
    pub fn transfer_royalty_to_distribution_account(&self, royalty_amount: u64) -> Result<()> {
        transfer(
            CpiContext::new(self.system_program.to_account_info(), Transfer {
                from: self.buyer.to_account_info(),
                to: self.distribution_account.to_account_info(),
            }),
            royalty_amount
        )
    }
    pub fn distribute_royalty(&mut self, royalty_amount: u64) -> Result<()> {
        let metadata = get_metadata(self.mint.to_account_info().as_ref())?;
        if royalty_amount == 0 {
            return Ok(());
        }
        let initial_creators_len = self.distribution_account.creators_claims.len();
        let creators = metadata.additional_metadata
            .iter()
            .filter(|(key, _)| key != "royalty_basis_points")
            .filter_map(|(key, value)| {
                match Pubkey::from_str(key) {
                    Ok(pubkey) =>
                        Some(CreatorWithShare {
                            address: pubkey,
                            share: value.parse::<u8>().unwrap(),
                        }),
                    Err(_) => None,
                }
            })
            .collect::<Vec<CreatorWithShare>>();

        // update creator amounts in distribution account. add creator if not present, else update amount (amount * pct / 100)
        let creators_already_claimer = &mut self.distribution_account.creators_claims;
        for creator in creators {
            let is_creator_already_claimer = creators_already_claimer
                .iter()
                .position(|cc| creator.address == cc.address);
            match is_creator_already_claimer {
                Some(creator_index) => {
                    // Calculate and add more claim to previous claim amount
                    creators_already_claimer
                        .get_mut(creator_index)
                        .unwrap()
                        .claim_amount.checked_add(
                            royalty_amount
                                .checked_mul(creator.share as u64)
                                .ok_or(MarketplaceError::ArithmeticError)?
                                .checked_div(100)
                                .ok_or(MarketplaceError::ArithmeticError)?
                        )
                        .ok_or(MarketplaceError::ArithmeticError)?;
                }
                None => {
                    // Add a new claimer
                    creators_already_claimer.push(Creator {
                        address: creator.address,
                        claim_amount: royalty_amount
                            .checked_mul(creator.share as u64)
                            .ok_or(MarketplaceError::ArithmeticError)?
                            .checked_div(100)
                            .ok_or(MarketplaceError::ArithmeticError)?,
                    });
                }
            }
        }
        if creators_already_claimer.len() <= initial_creators_len {
            // No need to reallocate space
            return Ok(());
        }
        msg!("Reallocating space and rent for new creators");
        let realloc_size =
            DISTRIBUTION_NO_CREATORS_SPACE + creators_already_claimer.len() * Creator::INIT_SPACE;
        if creators_already_claimer.len() == 0 {
            realloc_size.checked_add(Creator::INIT_SPACE).ok_or(MarketplaceError::ArithmeticError)?;
        }
        self.distribution_account.to_account_info().realloc(realloc_size, false)?;
        update_account_lamports_to_minimum_balance(
            self.distribution_account.to_account_info(),
            self.buyer.to_account_info(),
            self.system_program.to_account_info()
        )?;
        Ok(())
    }
    pub fn pay_and_invoke_nft_transfer(&self, payment_amount: u64) -> Result<()> {
        // transfer payment_amount to seller
        transfer(
            CpiContext::new(self.system_program.to_account_info(), Transfer {
                from: self.buyer.to_account_info(),
                to: self.seller.to_account_info(),
            }),
            payment_amount
        )?;
        // Unfreeze and allow for transfer
        self.thaw_account()?;
        let listed_nft_pda = &self.listed_nft_pda;
        let signer_seeds: &[&[&[u8]]] = &[
            &[
                b"listed_nft_pda".as_ref(),
                listed_nft_pda.seller.as_ref(),
                listed_nft_pda.mint.as_ref(),
                &[listed_nft_pda.bump],
            ],
        ];
        let additional_accounts = &mut vec![
            self.transfer_hook_program.to_account_info(),
            self.extra_metas_account.to_account_info()
        ];
        invoke_transfer_checked(
            &Token2022::id(),
            self.seller_token_account.to_account_info(),
            self.mint.to_account_info(),
            self.buyer_token_account.to_account_info(),
            self.listed_nft_pda.to_account_info(),
            additional_accounts,
            1,
            0,
            signer_seeds
        )?;
        Ok(())
    }
}

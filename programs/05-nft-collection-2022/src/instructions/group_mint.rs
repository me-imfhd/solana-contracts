use anchor_lang::{ prelude::*, solana_program::entrypoint::ProgramResult };

use anchor_spl::{
    associated_token::AssociatedToken,
    token_2022::spl_token_2022::instruction::AuthorityType,
    token_interface::{
        mint_to,
        set_authority,
        token_metadata_initialize,
        Mint,
        MintTo,
        SetAuthority,
        Token2022,
        TokenAccount,
        TokenMetadataInitialize,
    },
};

use crate::state::token_group::TokenGroup;

use super::*;

pub fn create_group_mint(ctx: Context<CreateGroupMint>, args: CreateGroupArgs) -> Result<()> {
    ctx.accounts.initialize_metadata(args.name, args.symbol, args.uri)?; // metadata stored inside the mint
    ctx.accounts.initialize_group_configuration(args.max_size)?; // stored outside the mint, since not supported yet
    ctx.accounts.mint_to_receiver()?; // creation of collection_nft
    ctx.accounts.close_minting()?; // make sure it will be an nft
    // Needs to have enough funds to allocated space for pointers and accounts with the mint
    update_account_lamports_to_minimum_balance(
        ctx.accounts.group_mint.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.system_program.to_account_info()
    )?;
    Ok(())
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct CreateGroupArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
    pub max_size: u32,
}
#[derive(Accounts)]
#[instruction()]
pub struct CreateGroupMint<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account()]
    /// CHECK: can be any account
    pub mint_to: UncheckedAccount<'info>,
    #[account(
        init,
        signer,
        payer = payer,
        mint::token_program = token_program,
        mint::decimals = 0,
        mint::authority = payer,
        extensions::metadata_pointer::authority = payer,
        extensions::metadata_pointer::metadata_address = group_mint,
        extensions::group_pointer::authority = payer,
        extensions::group_pointer::group_address = group_data_account
    )]
    pub group_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init,
        payer = payer,
        associated_token::token_program = token_program,
        associated_token::mint = group_mint,
        associated_token::authority = mint_to
    )]
    pub collection_nft: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init,
        seeds = [b"group_configuration_account", group_mint.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + TokenGroup::INIT_SPACE
    )]
    pub group_data_account: Account<'info, TokenGroup>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
}

impl CreateGroupMint<'_> {
    fn initialize_metadata(&self, name: String, symbol: String, uri: String) -> ProgramResult {
        let cpi_accounts = TokenMetadataInitialize {
            token_program_id: self.token_program.to_account_info(),
            mint: self.group_mint.to_account_info(),
            metadata: self.group_mint.to_account_info(), // metadata account is the mint, since data is stored in mint
            mint_authority: self.payer.to_account_info(),
            update_authority: self.payer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        token_metadata_initialize(cpi_ctx, name, symbol, uri)?;
        Ok(())
    }
    fn initialize_group_configuration(&mut self, max_size: u32) -> ProgramResult {
        // using a custom group account until token22 implements group account
        let group = &mut self.group_data_account;
        group.max_size = max_size;
        group.update_authority = self.payer.key();
        group.mint = self.group_mint.key();
        group.size = 0;
        Ok(())
    }

    fn mint_to_receiver(&self) -> Result<()> {
        let cpi_ctx = MintTo {
            mint: self.group_mint.to_account_info(),
            to: self.collection_nft.to_account_info(),
            authority: self.payer.to_account_info(),
        };
        let cpi_accounts = CpiContext::new(self.token_program.to_account_info(), cpi_ctx);
        mint_to(cpi_accounts, 1)?;
        Ok(())
    }
    fn close_minting(&self) -> Result<()> {
        let cpi_accounts = SetAuthority {
            current_authority: self.payer.to_account_info(),
            account_or_mint: self.group_mint.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        set_authority(cpi_ctx, AuthorityType::MintTokens, None)?;
        Ok(())
    }
}

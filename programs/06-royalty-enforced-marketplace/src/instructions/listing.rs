use anchor_lang::prelude::*;
use anchor_spl::token_interface::{
    Mint,
    Token2022,
    TokenAccount,
    freeze_account,
    FreezeAccount,
    approve,
    Approve,
    thaw_account,
    ThawAccount,
    revoke,
    Revoke,
};

use crate::state::ListedNftPda;

pub fn list_nft(ctx: Context<ListNft>, args: ListNftArgs) -> Result<()> {
    // gives approval/delegate for contract to be able to transfer the nft
    approve(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), Approve {
            authority: ctx.accounts.seller.to_account_info(),
            delegate: ctx.accounts.listed_nft_pda.to_account_info(),
            to: ctx.accounts.seller_token_account.to_account_info(),
        }),
        1
    )?;
    // Freeze so user can't relist them again anywhere to transfer it
    freeze_account(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), FreezeAccount {
            account: ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
        })
    )?;
    // List the NFT
    let list_nft = &mut ctx.accounts.listed_nft_pda;
    list_nft.set_inner(ListedNftPda {
        bump: ctx.bumps.listed_nft_pda,
        mint: ctx.accounts.mint.key(),
        price: args.price,
        seller: ctx.accounts.seller.key(),
        seller_ata: ctx.accounts.seller_token_account.key(),
    });
    Ok(())
}

pub fn unlist_nft(ctx: Context<ListNft>) -> Result<()> {
    // revoke delegation from the contract, will set delegate to null
    revoke(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), Revoke {
            authority: ctx.accounts.seller.to_account_info(),
            source: ctx.accounts.seller_token_account.to_account_info(),
        })
    )?;
    // UnFreeze (thaw) so user can
    thaw_account(
        CpiContext::new(ctx.accounts.token_program.to_account_info(), ThawAccount {
            account: ctx.accounts.seller_token_account.to_account_info(),
            authority: ctx.accounts.seller.to_account_info(),
            mint: ctx.accounts.mint.to_account_info(),
        })
    )?;
    Ok(())
}

#[derive(Accounts)]
pub struct ListNft<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
        init,
        payer = seller,
        space = 8 + ListedNftPda::INIT_SPACE,
        seeds = [b"listed_nft_pda", seller.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub listed_nft_pda: Account<'info, ListedNftPda>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::token_program = token_program,
        associated_token::mint = mint,
        associated_token::authority = seller
    )]
    pub seller_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UnListNft<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(
        mut,
        close = seller, // unlists the nft
        seeds = [b"listed_nft_pda", seller.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub listed_nft_pda: Account<'info, ListedNftPda>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        associated_token::token_program = token_program,
        associated_token::mint = mint,
        associated_token::authority = seller
    )]
    pub seller_token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Program<'info, Token2022>,
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct ListNftArgs {
    price: u64,
}

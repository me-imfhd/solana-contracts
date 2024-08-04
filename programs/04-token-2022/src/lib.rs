#![allow(ambiguous_glob_reexports)]
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::{
    self,
    Mint,
    MintTo,
    TokenAccount,
    TokenInterface,
    TransferChecked,
};
pub mod instructions;
pub use instructions::*;
declare_id!("Btqh6o2XcewQvF51sbMdXUwFaMK5waC988F3NbmTQNZo");

#[program]
pub mod token_2022_program {
    // use super::*;
    use super::*;
    pub fn create_token(ctx: Context<CreateToken>, _token_name: String) -> Result<()> {
        msg!("Created Token {:?}", ctx.accounts.mint);
        Ok(())
    }
    pub fn create_token_with_metadata_pointer(
        ctx: Context<CreateTokenWithMetadataPointer>,
        token_metadata: TokenMetadataArgs,
        decimal: u8
    ) -> Result<()> {
        instructions::metadata_pointer::create_token_with_metadata_pointer(
            ctx,
            token_metadata,
            decimal
        )?;
        Ok(())
    }
    pub fn create_token_account(_ctx: Context<CreateTokenAccount>) -> Result<()> {
        msg!("Create Token Account");
        Ok(())
    }
    pub fn create_associated_token_account(
        _ctx: Context<CreateAssociatedTokenAccount>
    ) -> Result<()> {
        msg!("Create Associated Token Account");
        Ok(())
    }
    pub fn transfer_token(ctx: Context<TransferToken>, amount: u64) -> Result<()> {
        token_interface::transfer_checked(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), TransferChecked {
                from: ctx.accounts.from.to_account_info().clone(),
                mint: ctx.accounts.mint.to_account_info().clone(),
                to: ctx.accounts.to_ata.to_account_info().clone(),
                authority: ctx.accounts.signer.to_account_info(),
            }),
            amount,
            ctx.accounts.mint.decimals
        )?;
        msg!("Transfer Token");
        Ok(())
    }
    pub fn mint_token(ctx: Context<MintToken>, amount: u64) -> Result<()> {
        token_interface::mint_to(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), MintTo {
                mint: ctx.accounts.mint.to_account_info().clone(),
                to: ctx.accounts.receiver.to_account_info().clone(),
                authority: ctx.accounts.signer.to_account_info(),
            }),
            amount
        )?;
        msg!("Mint Token");
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(token_name: String)]
pub struct CreateToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(
        init,
        payer = signer,
        mint::decimals = 6,
        mint::authority = signer.key(),
        seeds = [b"token-2022-token", signer.key().as_ref(), token_name.as_bytes()],
        bump
    )]
    pub mint: InterfaceAccount<'info, Mint>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}
#[derive(Accounts)]
pub struct CreateTokenAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        token::mint = mint,
        token::authority = signer,
        payer = signer,
        seeds = [b"token-2022-token-account", signer.key().as_ref(), mint.key().as_ref()],
        bump
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
}

#[derive(Accounts)]
pub struct CreateAssociatedTokenAccount<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(
        init,
        associated_token::token_program = token_program,
        associated_token::mint = mint,
        payer = signer,
        associated_token::authority = signer
    )]
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct TransferToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub from: InterfaceAccount<'info, TokenAccount>,
    pub to: SystemAccount<'info>,
    #[account(
        init,
        associated_token::mint = mint,
        payer = signer,
        associated_token::authority = to
    )]
    pub to_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

#[derive(Accounts)]
pub struct MintToken<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    #[account(mut)]
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut)]
    pub receiver: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
}

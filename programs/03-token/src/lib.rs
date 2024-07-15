use anchor_lang::prelude::*;
use anchor_spl::token::{ Mint, Token };

declare_id!("8R9g9PTeWNt8v7Uf5YLeorV9vYcZJGLtx9LuTA4uwtsR");

// Highly recommended to read:
// https://solana.com/docs/core/tokens,
// https://docs.rs/anchor-lang/0.30.1/anchor_lang/derive.Accounts.html#spl-constraints

#[program]
mod token {
    use super::*;
    pub fn create_mint(ctx: Context<CreateMint>, _decimal: u8) -> Result<()> {
        msg!("Created a new token mint, {:?}", ctx.accounts.mint);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(decimals: u8)]
pub struct CreateMint<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(
        init, // create new mint / token type
        payer = signer, // deduct lamport for rent from this guy
        mint::decimals = decimals, // decimal places
        mint::authority = signer, // signer can increase the supply of the token
        mint::freeze_authority = signer // signer can freeze total supply
    )]
    mint: Account<'info, Mint>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>,
}

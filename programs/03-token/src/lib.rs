use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{ Mint, Token, TokenAccount },
};

declare_id!("8C21j7EYA4ztftyvpJ66cpdKd58ncF4fZGBwPPWxDQyC");

// Highly recommended to read:
// https://solana.com/docs/core/tokens,
// https://docs.rs/anchor-lang/0.30.1/anchor_lang/derive.Accounts.html#spl-constraints

#[program]
mod token {
    use anchor_spl::{
        token::{ mint_to, transfer, MintTo, Transfer },
    };

    use super::*;
    pub fn create_mint(ctx: Context<CreateMint>, _decimal: u8) -> Result<()> {
        msg!("Created a new token mint, {:?}", ctx.accounts.mint);
        Ok(())
    }
    pub fn create_user_token_account(ctx: Context<CreateUserTokenAccount>) -> Result<()> {
        msg!("Created a new associated token account {:?}", ctx.accounts.associated_token_account);
        Ok(())
    }
    // updates the supply of mint_account
    pub fn mint_tokens(ctx: Context<MintTokens>, amount: u16) -> Result<()> {
        mint_to(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), MintTo {
                authority: ctx.accounts.signer.to_account_info(), // mint_authority
                mint: ctx.accounts.mint.to_account_info(),
                to: ctx.accounts.associated_token_account.to_account_info(), // reciever
            }),
            amount.into()
        )?;
        msg!("Minted {} tokens ", amount);
        Ok(())
    }
    // supply is same, but token is transfered to some other ATA
    pub fn transfer_token(
        ctx: Context<TransferTokens>,
        amount: u16
    ) -> Result<()> {
        transfer(
            CpiContext::new(ctx.accounts.token_program.to_account_info(), Transfer {
                authority: ctx.accounts.signer.to_account_info(),
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.reciever_token_account.to_account_info(),
            }),
            amount.into()
        )?;
        msg!("Transferred {} tokens ", amount);
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

#[derive(Accounts)]
pub struct CreateUserTokenAccount<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    mint: Account<'info, Mint>,
    #[account(
        init,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    associated_token_account: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct MintTokens<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(mut, constraint = signer.key() == mint.mint_authority.unwrap() )]
    mint: Account<'info, Mint>,
    #[account(
        init_if_needed,
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    associated_token_account: Account<'info, TokenAccount>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    mint: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = signer
    )]
    sender_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed, // creates the ata if not already exists
        payer = signer,
        associated_token::mint = mint,
        associated_token::authority = reciever_account
    )] // anchor will automatically derive the token account from the reciver_account
    reciever_token_account: Account<'info, TokenAccount>,
    /// CHECK: Token Reciever
    reciever_account: AccountInfo<'info>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
    system_program: Program<'info, System>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct MetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

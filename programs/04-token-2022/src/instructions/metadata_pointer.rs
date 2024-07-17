use anchor_lang::prelude::*;
use anchor_lang::system_program::{ self, transfer };
use anchor_spl::token_2022::{ self, Token2022 };
use anchor_spl::token_interface::{
    metadata_pointer_initialize,
    token_metadata_initialize,
    MetadataPointerInitialize,
    TokenMetadataInitialize,
};
use super::Space;
#[derive(Clone, Debug, AnchorSerialize, AnchorDeserialize)]
pub struct TokenMetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}
pub fn create_token_with_metadata_pointer(
    ctx: Context<CreateTokenWithMetadataPointer>,
    token_metadata: TokenMetadataArgs,
    decimal: u8
) -> Result<()> {
    let mint = &ctx.accounts.mint;
    let metadata_pointer = &ctx.accounts.mint; // because it will be pointing to itself
    let token_program = &ctx.accounts.token_program;
    let authority = &ctx.accounts.signer;

    // Initialize the metadata pointer (Need to do this before initializing the mint)
    metadata_pointer_initialize(
        CpiContext::new(token_program.to_account_info(), MetadataPointerInitialize {
            mint: mint.to_account_info(),
            token_program_id: token_program.to_account_info(),
        }),
        Some(ctx.accounts.signer.key()),
        Some(ctx.accounts.mint.key())
    )?;

    token_2022::initialize_mint2(
        CpiContext::new(token_program.to_account_info(), token_2022::InitializeMint2 {
            mint: mint.to_account_info(),
        }),
        decimal,
        &ctx.accounts.signer.key(),
        None
    )?;
    // we need to send more lamports to pay the rent for addtional metadata space
    let space = std::mem::size_of_val(&token_metadata);
    let lamports = Rent::get()?.minimum_balance(space);
    transfer(
        CpiContext::new(ctx.accounts.system_program.to_account_info(), system_program::Transfer {
            from: authority.to_account_info(),
            to: mint.to_account_info(),
        }),
        lamports
    )?;
    token_metadata_initialize(
        CpiContext::new(token_program.to_account_info(), TokenMetadataInitialize {
            mint: mint.to_account_info(),
            mint_authority: authority.to_account_info(),
            token_program_id: token_program.to_account_info(),
            update_authority: authority.to_account_info(),
            metadata: metadata_pointer.to_account_info(),
        }),
        token_metadata.name,
        token_metadata.symbol,
        token_metadata.uri
    )?;
    msg!("Created Token with metadata and metadata pointer pointing to itself {:?}", mint);
    Ok(())
}
#[derive(Accounts)]
#[instruction(token_metadata: TokenMetadataArgs)]
pub struct CreateTokenWithMetadataPointer<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    /// CHECK: We need to initialize the metadata pointer before initializing mint
    #[account(
        init,
        space = Space::mint_size_with_metadata_pointer()?,
        payer = signer,
        seeds = [b"token-2022-token", signer.key().as_ref(), token_metadata.name.as_bytes()],
        bump,
        owner = token_program.key()
    )]
    pub mint: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
    pub token_program: Program<'info, Token2022>,
}

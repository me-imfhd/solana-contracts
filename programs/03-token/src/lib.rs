use anchor_lang::prelude::*;
use anchor_spl::{ metadata::Metadata, token::{ Mint, Token } };

declare_id!("CAcz3Z9p5bH56mX2pzEyEfzBCqE5uUiuNuYWngJe2p8V");

// Highly recommended to read:
// https://solana.com/docs/core/tokens,
// https://docs.rs/anchor-lang/0.30.1/anchor_lang/derive.Accounts.html#spl-constraints

#[program]
mod token {
    use anchor_spl::metadata::{
        create_metadata_accounts_v3,
        mpl_token_metadata::types::DataV2,
        CreateMetadataAccountsV3,
    };

    use super::*;
    pub fn create_mint(ctx: Context<CreateMint>, _decimal: u8) -> Result<()> {
        msg!("Created a new token mint, {:?}", ctx.accounts.mint);
        Ok(())
    }
    pub fn attach_metadata(
        ctx: Context<AttachMetadata>,
        metadata_args: MetadataArgs
    ) -> Result<()> {
        msg!("Creating metadata account...");
        msg!("Metadata account address: {}", &ctx.accounts.metadata_account.key());
        create_metadata_accounts_v3(
            CpiContext::new(
                ctx.accounts.metadata_program.to_account_info(),
                CreateMetadataAccountsV3 {
                    metadata: ctx.accounts.metadata_account.to_account_info(),
                    mint: ctx.accounts.mint.to_account_info(),
                    mint_authority: ctx.accounts.signer.to_account_info(),
                    payer: ctx.accounts.signer.to_account_info(),
                    rent: ctx.accounts.rent.to_account_info(),
                    system_program: ctx.accounts.system_program.to_account_info(),
                    update_authority: ctx.accounts.signer.to_account_info(),
                }
            ),
            DataV2 {
                collection: None,
                creators: None,
                name: metadata_args.name,
                symbol: metadata_args.symbol,
                uri: metadata_args.uri,
                seller_fee_basis_points: 0,
                uses: None,
            },
            true,
            true,
            None
        )?;

        msg!("Attached metadata successfully.");

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
pub struct AttachMetadata<'info> {
    #[account(mut)]
    signer: Signer<'info>,
    #[account(constraint = signer.key() == mint.mint_authority.unwrap())]
    mint: Account<'info, Mint>,
    /// CHECK: verify the address
    #[account(mut, 
        seeds = [b"metadata", metadata_program.key().as_ref(), mint.key().as_ref()],
        bump,
        seeds::program = metadata_program.key(),
    )]
    metadata_account: UncheckedAccount<'info>, // we will create metadata account in function with cpi
    metadata_program: Program<'info, Metadata>,
    system_program: Program<'info, System>,
    rent: Sysvar<'info, Rent>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Debug, Clone)]
pub struct MetadataArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

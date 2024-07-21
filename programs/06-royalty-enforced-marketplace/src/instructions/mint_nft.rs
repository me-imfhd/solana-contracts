use anchor_lang::prelude::*;
use anchor_lang::solana_program::entrypoint::ProgramResult;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token_interface::*;
use anchor_spl::token_interface::{
    token_metadata_initialize,
    Mint,
    TokenAccount,
    TokenMetadataInitialize,
    spl_token_2022::instruction::AuthorityType,
    spl_pod::solana_program::program_option::COption,
};

use crate::state::token_group::TokenGroup;
use crate::state::token_group_member::TokenGroupMember;
use super::update_account_lamports_to_minimum_balance;
pub fn mint_nft(ctx: Context<MintNft>, args: MintNftArgs) -> Result<()> {
    ctx.accounts.initialize_token_metadata(args.name, args.symbol, args.uri)?; // metadata stored inside the mint
    ctx.accounts.initialize_member_configuration()?; // member_data is stored outside the mint, since not supported yet
    ctx.accounts.mint_to_receiver()?; // creation of collection_nft
    ctx.accounts.close_minting()?; // make sure it will be an nft
    // Needs to have enough funds to allocated space for pointers and accounts with the mint
    update_account_lamports_to_minimum_balance(
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.system_program.to_account_info()
    )?;
    Ok(())
}

#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct MintNftArgs {
    pub name: String,
    pub symbol: String,
    pub uri: String,
}

#[derive(Accounts)]
#[instruction(args: MintNftArgs)]
pub struct MintNft<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    /// CHECK: nft_owner
    pub mint_to: AccountInfo<'info>,
    #[account(
        constraint = group_mint.mint_authority == COption::None &&
        group_mint.decimals == 0 &&
        group_mint.supply == 1
    )]
    pub group_mint: InterfaceAccount<'info, Mint>,
    #[account(
        mut,
        seeds = [b"group_configuration_account", group_mint.key().to_bytes().as_ref()],
        bump,
        constraint = group_data_account.mint == group_mint.key() &&
        group_data_account.update_authority == payer.key()
    )]
    pub group_data_account: Account<'info, TokenGroup>,
    #[account(
        init,
        signer,
        payer = payer,
        mint::token_program = token_program,
        mint::decimals = 0,
        mint::authority = payer,
        extensions::metadata_pointer::authority = payer,
        extensions::metadata_pointer::metadata_address = mint,
        extensions::group_member_pointer::authority = payer,
        extensions::group_member_pointer::member_address = mint,
        extensions::transfer_hook::authority = payer,
        extensions::transfer_hook::program_id = crate::ID
    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
        init,
        payer = payer,
        associated_token::token_program = token_program,
        associated_token::mint = mint,
        associated_token::authority = mint_to
    )]
    pub nft: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init,
        seeds = [b"member_configuration_account", mint.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + TokenGroupMember::INIT_SPACE
    )]
    pub member_data_account: Account<'info, TokenGroupMember>,
    pub system_program: Program<'info, System>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Program<'info, Token2022>,
}

impl<'info> MintNft<'info> {
    fn initialize_token_metadata(
        &self,
        name: String,
        symbol: String,
        uri: String
    ) -> ProgramResult {
        let cpi_accounts = TokenMetadataInitialize {
            token_program_id: self.token_program.to_account_info(),
            mint: self.mint.to_account_info(),
            metadata: self.mint.to_account_info(),
            mint_authority: self.payer.to_account_info(),
            update_authority: self.payer.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        token_metadata_initialize(cpi_ctx, name, symbol, uri)?;
        Ok(())
    }
    fn initialize_member_configuration(&mut self) -> ProgramResult {
        let group = &mut self.group_data_account;
        let member_number = group.increment_size()?;

        // using a custom member account until token22 implements member account
        let member = &mut self.member_data_account;
        member.group = self.group_data_account.key();
        member.mint = self.mint.key();
        member.member_number = member_number;
        Ok(())
    }

    fn mint_to_receiver(&self) -> Result<()> {
        let cpi_ctx = MintTo {
            mint: self.mint.to_account_info(),
            to: self.nft.to_account_info(),
            authority: self.mint_to.to_account_info(),
        };
        let cpi_accounts = CpiContext::new(self.token_program.to_account_info(), cpi_ctx);
        mint_to(cpi_accounts, 1)?;
        Ok(())
    }

    fn close_minting(&self) -> Result<()> {
        let cpi_accounts = SetAuthority {
            current_authority: self.payer.to_account_info(),
            account_or_mint: self.mint.to_account_info(),
        };
        let cpi_ctx = CpiContext::new(self.token_program.to_account_info(), cpi_accounts);
        set_authority(cpi_ctx, AuthorityType::MintTokens, None)?; // no more token will be minted now
        Ok(())
    }
}
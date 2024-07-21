use anchor_lang::{ prelude::*, solana_program::entrypoint::ProgramResult };
use anchor_spl::token_interface::{
    spl_token_metadata_interface::state::Field,
    token_metadata_update_field,
    Mint,
    Token2022,
    TokenMetadataUpdateField,
};
use spl_tlv_account_resolution::{ account::ExtraAccountMeta, state::ExtraAccountMetaList };
use spl_transfer_hook_interface::instruction::ExecuteInstruction;

use crate::{ error::MetadataErrors, update_account_lamports_to_minimum_balance };

use super::UpdateRoyaltiesArgs;
/// adds royalty attributes to nft's/mint's metadata, initailizes extra_meta_accounts for transfer hook
pub fn add_royalties(ctx: Context<AddRoyalties>, args: UpdateRoyaltiesArgs) -> Result<()> {
    // validate that the fee_basis_point is less than 10000 (100%)
    require!(args.royalty_basis_points <= 10000, MetadataErrors::RoyaltyBasisPointsInvalid);
    ctx.accounts.update_token_metadata(
        Field::Key("royalty_basis_points".to_owned()),
        args.royalty_basis_points.to_string()
    )?;
    let mut total_share: u8 = 0;
    // add creators and their respective shares to metadata
    for creator in args.creators {
        total_share = total_share
            .checked_add(creator.share)
            .ok_or(MetadataErrors::CreatorShareInvalid)?;
        ctx.accounts.update_token_metadata(
            Field::Key(creator.address.to_string()),
            creator.share.to_string()
        )?;
    }
    if total_share != 100 {
        return Err(MetadataErrors::CreatorShareInvalid.into());
    }
    ctx.accounts.init_account_metas()?;
    update_account_lamports_to_minimum_balance(
        ctx.accounts.mint.to_account_info(),
        ctx.accounts.payer.to_account_info(),
        ctx.accounts.system_program.to_account_info()
    )?;
    Ok(())
}
#[derive(Accounts)]
pub struct AddRoyalties<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        mut,
        mint::token_program = token_program,
    )]
    pub mint: Box<InterfaceAccount<'info, Mint>>,
    /// CHECK: This account's data is a buffer of TLV data
    #[account(
        init,
        space = AddRoyalties::extra_account_metas_size()?,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        payer = payer
    )]
    pub extra_metas_account: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

impl<'info> AddRoyalties<'info> {
    pub fn account_metas() -> Result<Vec<ExtraAccountMeta>> {
        let meta_accounts: Vec<ExtraAccountMeta> = vec![];
        Ok(meta_accounts)
    }
    pub fn extra_account_metas_size() -> Result<usize> {
        let meta_accounts: Vec<ExtraAccountMeta> = AddRoyalties::account_metas()?;
        let size = ExtraAccountMetaList::size_of(meta_accounts.len())?;
        Ok(size)
    }
    pub fn init_account_metas(&mut self) -> ProgramResult {
        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut self.extra_metas_account.try_borrow_mut_data()?,
            AddRoyalties::account_metas()?.as_ref()
        )?;
        Ok(())
    }
    pub fn update_token_metadata(&self, field: Field, value: String) -> Result<()> {
        token_metadata_update_field(
            CpiContext::new(self.token_program.to_account_info(), TokenMetadataUpdateField {
                metadata: self.mint.to_account_info(),
                token_program_id: self.token_program.to_account_info(),
                update_authority: self.payer.to_account_info(),
            }),
            field,
            value
        )?;
        Ok(())
    }
}

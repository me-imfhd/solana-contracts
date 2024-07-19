use anchor_lang::prelude::*;
use anchor_spl::token_interface::{ Token2022, Mint };
use spl_tlv_account_resolution::{ account::ExtraAccountMeta, state::ExtraAccountMetaList };

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
        space = 8,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        payer = payer
    )]
    pub extra_metas_account: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token2022>,
}

// Define extra account metas to store on extra_account_meta_list account
// In this example there are none
impl<'info> AddRoyalties<'info> {
    pub fn extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        Ok(vec![])
    }
}

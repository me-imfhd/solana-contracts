#![allow(ambiguous_glob_reexports)]
use anchor_lang::prelude::*;

declare_id!("DZaduqinAmkYy15YzdBr48FKgP8MDevdK4dmgrWxvvyH");
pub mod instructions;
pub mod error;
use anchor_spl::{ token_2022::Token2022, token_interface::Mint };
pub use instructions::*;
use spl_tlv_account_resolution::{ account::ExtraAccountMeta, state::ExtraAccountMetaList };

#[program]
pub mod enforced_transfer_hook {
    use spl_transfer_hook_interface::instruction::ExecuteInstruction;

    use super::*;
    // #[interface(spl_transfer_hook_interface::initialize_extra_account_meta_list)]
    pub fn initialize_extra_account_meta_list(
        ctx: Context<InitializeExtraAccountMetaList>
    ) -> Result<()> {
        let extra_account_metas = InitializeExtraAccountMetaList::extra_account_metas()?;
        // initialize ExtraAccountMetaList account with extra accounts
        ExtraAccountMetaList::init::<ExecuteInstruction>(
            &mut ctx.accounts.extra_account_meta_list.try_borrow_mut_data()?,
            &extra_account_metas
        )?;
        Ok(())
    }
    #[interface(spl_transfer_hook_interface::execute)]
    pub fn execute_hook(ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
        instructions::execute_hook(ctx)
    }
}
#[derive(Accounts)]
pub struct InitializeExtraAccountMetaList<'info> {
    #[account(mut)]
    payer: Signer<'info>,
    /// CHECK: ExtraAccountMetaList Account, must use these seeds
    #[account(
        init,
        seeds = [b"extra-account-metas", mint.key().as_ref()],
        bump,
        space = ExtraAccountMetaList::size_of(
            InitializeExtraAccountMetaList::extra_account_metas()?.len()
        )?,
        payer = payer
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}
impl<'info> InitializeExtraAccountMetaList<'info> {
    pub fn extra_account_metas() -> Result<Vec<ExtraAccountMeta>> {
        Ok(vec![])
    }
}

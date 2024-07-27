#![allow(ambiguous_glob_reexports)]
use anchor_lang::{ prelude::*, solana_program::sysvar };

declare_id!("DZaduqinAmkYy15YzdBr48FKgP8MDevdK4dmgrWxvvyH");
pub mod instructions;
pub mod error;
use anchor_spl::{ token_2022::Token2022, token_interface::Mint };
pub use instructions::*;
use spl_tlv_account_resolution::{ account::ExtraAccountMeta, state::ExtraAccountMetaList };

#[program]
pub mod enforced_transfer_hook {
    use error::MarketplaceError;
    use spl_transfer_hook_interface::instruction::ExecuteInstruction;

    use super::*;
    // #[interface(spl_transfer_hook_interface::initialize_extra_account_meta_list)]
    pub fn initialize_extra_account_meta_list(
        ctx: Context<InitializeExtraAccountMetaList>
    ) -> Result<()> {
        let extra_account_metas = InitializeExtraAccountMetaList::extra_account_metas(
            ctx.accounts.enforcing_account.key()
        )?;
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
    pub fn set_enforcement(ctx: Context<SetEnforcement>) -> Result<()> {
        let ix = anchor_lang::solana_program::sysvar::instructions::get_instruction_relative(
            0,
            &ctx.accounts.instruction
        )?;
        if ix.program_id != pubkey!("HuLA2mESUMReZJbQvx2nbFFrQNJw6jWLvKF1akf4PUqq") {
            return Err(MarketplaceError::RequireMarketplaceProgram.into());
        }
        ctx.accounts.enforcing_account.slot = Clock::get()?.slot;
        msg!("Slot is set to : {:?}", ctx.accounts.enforcing_account.slot);
        Ok(())
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
            InitializeExtraAccountMetaList::extra_account_metas(enforcing_account.key())?.len()
        )?,
        payer = payer
    )]
    pub extra_account_meta_list: UncheckedAccount<'info>,
    #[account(
        init,
        seeds = [b"enforcing_account", mint.key().as_ref()],
        bump,
        payer = payer,
        space = 8 + EnforcingAccount::INIT_SPACE
    )]
    pub enforcing_account: Account<'info, EnforcingAccount>,
    pub mint: InterfaceAccount<'info, Mint>,
    pub token_program: Program<'info, Token2022>,
    pub system_program: Program<'info, System>,
}
impl<'info> InitializeExtraAccountMetaList<'info> {
    pub fn extra_account_metas(enforcing_account: Pubkey) -> Result<Vec<ExtraAccountMeta>> {
        Ok(vec![ExtraAccountMeta::new_with_pubkey(&enforcing_account, false, true)?])
    }
}

#[derive(Accounts)]
pub struct SetEnforcement<'info> {
    pub mint: InterfaceAccount<'info, Mint>,
    #[account(mut, seeds = [b"enforcing_account", mint.key().as_ref()], bump)]
    pub enforcing_account: Account<'info, EnforcingAccount>,
    /// CHECK: sysvar
    #[account(address = sysvar::instructions::id())]
    pub instruction: UncheckedAccount<'info>,
}
#[account]
#[derive(InitSpace, Debug)]
pub struct EnforcingAccount {
    pub slot: u64,
}

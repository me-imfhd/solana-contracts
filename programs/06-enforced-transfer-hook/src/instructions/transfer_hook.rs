use std::cell::RefMut;

use anchor_lang::prelude::*;
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{
            transfer_hook::TransferHookAccount,
            BaseStateWithExtensionsMut,
            PodStateWithExtensionsMut,
        },
        pod::PodAccount,
    },
    token_interface::{ Mint, TokenAccount },
};

use crate::error::MetadataErrors;

pub fn execute_hook(ctx: Context<TransferHook>) -> Result<()> {
    msg!("Executing transfer hook");
    ctx.accounts.check_is_transferring()?;
    msg!("OK");
    Ok(())
    // if ctx.remaining_accounts.is_empty() {
    //     return Err(MetadataErrors::MissingApproveAccount.into());
    // }
    // let mut enforcing_account: EnforcingAccount = AnchorDeserialize::deserialize(
    //     &mut &ctx.remaining_accounts[0].try_borrow_mut_data()?[8..]
    // )?;
    // if enforcing_account.slot == Clock::get()?.slot {
    //     // mark approve account as used by setting slot to 0
    //     enforcing_account.slot = 0;
    //     AnchorSerialize::serialize(
    //         &enforcing_account,
    //         &mut &mut ctx.remaining_accounts[0].try_borrow_mut_data()?[8..]
    //     )?;
    //     return Ok(());
    // } else {
    //     return Err(MetadataErrors::ExpiredApproveAccount.into());
    // }
}
// order really matters here
#[derive(Accounts)]
#[instruction(amount: u64)]
pub struct TransferHook<'info> {
    #[account(
        token::mint = mint,
        token::token_program = anchor_spl::token_interface::spl_token_2022::id()
    )]
    source_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(token::token_program = anchor_spl::token_interface::spl_token_2022::id())]
    mint: InterfaceAccount<'info, Mint>,
    #[account(
        token::mint = mint,
        token::token_program = anchor_spl::token_interface::spl_token_2022::id()
    )]
    destination_token_account: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: verified with constraints above
    owner: UncheckedAccount<'info>,
    /// CHECK: meta list accounts
    #[account(seeds = [b"extra-account-metas", mint.key().as_ref()], bump)]
    extra_metas_account: UncheckedAccount<'info>,
}

impl<'info> TransferHook<'info> {
    fn check_is_transferring(&self) -> Result<()> {
        let source_token_info = self.source_token_account.to_account_info();
        let mut account_data_ref: RefMut<&mut [u8]> = source_token_info.try_borrow_mut_data()?;
        let mut account = PodStateWithExtensionsMut::<PodAccount>::unpack(*account_data_ref)?;
        let account_extension = account.get_extension_mut::<TransferHookAccount>()?;
        if !bool::from(account_extension.transferring) {
            return err!(MetadataErrors::IsNotCurrentlyTransferring);
        }
        Ok(())
    }
}

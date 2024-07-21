use anchor_lang::prelude::*;
use anchor_spl::token_interface::{ Mint, TokenAccount };

pub fn royalties_enforcement_hook(ctx: Context<TransferHook>) -> Result<()> {
    Ok(())
}
// order really matters here
#[derive(Accounts)]
pub struct TransferHook<'info> {
    #[account(token::mint = mint, token::authority = owner)]
    source_token_account: InterfaceAccount<'info, TokenAccount>,
    mint: InterfaceAccount<'info, Mint>,
    #[account(token::mint = mint)]
    destination_token_account: InterfaceAccount<'info, TokenAccount>,
    /// CHECK: verified with constraints above
    owner: UncheckedAccount<'info>,
    /// CHECK: meta list accounts
    #[account(seeds = [b"extra-accounts-meta", mint.key().as_ref()], bump)]
    extra_metas_account: UncheckedAccount<'info>,
}

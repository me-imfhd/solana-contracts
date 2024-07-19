pub mod mint_nft;
pub mod group_mint;
pub mod royalties;

pub use mint_nft::*;
pub use group_mint::*;
pub use royalties::*;

use anchor_lang::prelude::*;
pub fn update_account_lamports_to_minimum_balance<'info>(
    account: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>
) -> Result<()> {
    let extra_lamports_needed = Rent::get()?
        .minimum_balance(account.data_len())
        .saturating_sub(account.get_lamports());
    msg!("ex:{}", extra_lamports_needed);
    if extra_lamports_needed > 0 {
        anchor_lang::system_program::transfer(
            CpiContext::new(system_program, anchor_lang::system_program::Transfer {
                from: payer,
                to: account,
            }),
            extra_lamports_needed
        )?;
    }
    Ok(())
}

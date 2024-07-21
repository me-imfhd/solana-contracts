pub mod mint_nft;
pub mod group_mint;
pub mod royalties;
pub mod royalty_transfer_hook;
use anchor_spl::{
    token_2022::spl_token_2022::{
        extension::{ BaseStateWithExtensions, StateWithExtensions },
        state,
    },
    token_interface::spl_token_metadata_interface::state::TokenMetadata,
};
pub use mint_nft::*;
pub use group_mint::*;
pub use royalties::*;
pub use royalty_transfer_hook::*;
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

pub fn calculate_royalties(mint: &AccountInfo, amount: u64) -> Result<u64> {
    let mint_account_data = mint.try_borrow_data()?;
    let mint_data = StateWithExtensions::<state::Mint>::unpack(&mint_account_data)?;
    let metadata = mint_data.get_variable_len_extension::<TokenMetadata>()?;
    let royalty_basis_points = metadata.additional_metadata
        .iter()
        .find(|(key, _)| key == "royalty_basis_points")
        .map(|(_, value)| value.parse::<u64>().unwrap())
        .unwrap_or(0);

    Ok((amount * royalty_basis_points) / 10000)
}
#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct CreatorWithShare {
    pub address: Pubkey,
    pub share: u8,
}

#[derive(AnchorDeserialize, AnchorSerialize, Clone)]
pub struct UpdateRoyaltiesArgs {
    pub royalty_basis_points: u16,
    pub creators: Vec<CreatorWithShare>,
}

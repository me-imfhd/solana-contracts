pub mod mint_nft;
pub mod group_mint;
pub mod royalties;
pub mod transfer_hook;
pub mod listing;
pub mod buy_nft;
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
pub use transfer_hook::*;
pub use listing::*;
pub use buy_nft::*;
use anchor_lang::prelude::*;
pub fn update_account_lamports_to_minimum_balance<'info>(
    account: AccountInfo<'info>,
    payer: AccountInfo<'info>,
    system_program: AccountInfo<'info>
) -> Result<()> {
    let extra_lamports_needed = Rent::get()?
        .minimum_balance(account.data_len())
        .saturating_sub(account.get_lamports());
    if extra_lamports_needed > 0 {
        anchor_lang::system_program::transfer(
            CpiContext::new(system_program, anchor_lang::system_program::Transfer {
                from: payer,
                to: account,
            }),
            extra_lamports_needed
        ).unwrap();
    }
    Ok(())
}
pub fn get_metadata(mint: &AccountInfo) -> std::result::Result<TokenMetadata, ProgramError> {
    let mint_account_data = mint.try_borrow_data()?;
    let mint_data = StateWithExtensions::<state::Mint>::unpack(&mint_account_data)?;
    mint_data.get_variable_len_extension::<TokenMetadata>()
}
pub fn calculate_royalties(mint: &AccountInfo, amount: u64) -> Result<u64> {
    let metadata = get_metadata(mint)?;
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

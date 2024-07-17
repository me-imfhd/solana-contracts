use anchor_lang::Result;
use anchor_spl::token_2022::spl_token_2022::{ extension::ExtensionType, state };

pub mod metadata_pointer;

pub struct Space;
impl Space {
    pub fn mint_size_with_metadata_pointer() -> Result<usize> {
        Ok(
            ExtensionType::try_calculate_account_len::<state::Mint>(
                &[ExtensionType::MetadataPointer]
            )?
        )
    }
}

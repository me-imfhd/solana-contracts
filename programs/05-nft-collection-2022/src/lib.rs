#![allow(ambiguous_glob_reexports)]
use anchor_lang::prelude::*;

declare_id!("7CysybXTfFLKS9XrRkDKUwufyuVgAaQ6stbuefHvHZyo");
pub mod instructions;
pub mod state;
pub mod error;
pub use instructions::*;
#[program]
pub mod nft_collection_2022 {
    use super::*;
    pub fn collection_mint(ctx: Context<CreateGroupMint>, args: CreateGroupArgs) -> Result<()> {
        instructions::group_mint::create_group_mint(ctx, args)
    }
    pub fn mint_nft(ctx: Context<MintNft>, args: MintNftArgs) -> Result<()> {
        instructions::mint_nft::mint_nft(ctx, args)
    }
}

#![allow(ambiguous_glob_reexports)]
use anchor_lang::prelude::*;

declare_id!("9iSkAhQ1jMLsk9mDRztTnDALmFpZk8W19fT8BCJCVn8g");
pub mod instructions;
pub mod state;
pub mod error;
pub use instructions::*;
#[program]
pub mod royalty_transfer_hook {
    use super::*;
    pub fn collection_mint(ctx: Context<CreateGroupMint>, args: CreateGroupArgs) -> Result<()> {
        instructions::group_mint::create_group_mint(ctx, args)
    }
    pub fn mint_nft(ctx: Context<MintNft>, args: MintNftArgs) -> Result<()> {
        instructions::mint_nft::mint_nft(ctx, args)
    }
}

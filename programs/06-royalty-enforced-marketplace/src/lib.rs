#![allow(ambiguous_glob_reexports)]
use anchor_lang::prelude::*;

declare_id!("HuLA2mESUMReZJbQvx2nbFFrQNJw6jWLvKF1akf4PUqq");
pub mod instructions;
pub mod state;
pub mod error;
pub use instructions::*;

#[program]
pub mod royalty_enforced_marketplace {
    use super::*;
    pub fn collection_mint(ctx: Context<CreateGroupMint>, args: CreateGroupArgs) -> Result<()> {
        instructions::group_mint::create_group_mint(ctx, args)
    }
    pub fn mint_nft(ctx: Context<MintNft>, args: MintNftArgs) -> Result<()> {
        instructions::mint_nft::mint_nft(ctx, args)
    }
    pub fn add_royalties(ctx: Context<AddRoyalties>, args: UpdateRoyaltiesArgs) -> Result<()> {
        instructions::add_royalties(ctx, args)
    }
    pub fn list_nft(ctx: Context<ListNft>, args: ListNftArgs) -> Result<()> {
        instructions::list_nft(ctx, args)
    }
    pub fn unlist_nft(ctx: Context<ListNft>) -> Result<()> {
        instructions::unlist_nft(ctx)
    }
    pub fn buy_nft(ctx: Context<BuyNft>) -> Result<()> {
        instructions::buy_nft(ctx)
    }
    // #[interface(spl_transfer_hook_interface::execute)]
    // pub fn execute_hook(ctx: Context<TransferHook>, _amount: u64) -> Result<()> {
    //     instructions::execute_hook(ctx)
    // }
}

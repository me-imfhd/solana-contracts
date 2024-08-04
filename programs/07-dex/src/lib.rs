#![allow(ambiguous_glob_reexports)]
use anchor_lang::prelude::*;
mod state;
mod instructions;
use state::*;
use instructions::*;
declare_id!("2QwSjSH5awaCodfXSuS4c4SpwiwyXfQL8LhW5owyP3mZ");
#[program]
pub mod dex {
    use super::*;

    /// Initialize the program by creating the liquidity pool
    pub fn create_pool(ctx: Context<CreatePool>) -> Result<()> {
        instructions::create_pool(ctx)
    }

    /// Provide liquidity to the pool by funding it with some asset
    pub fn fund_pool(ctx: Context<FundPool>, amount: u64) -> Result<()> {
        instructions::fund_pool(ctx, amount)
    }

    // /// Swap assets using the DEX
    // pub fn swap(ctx: Context<Swap>, amount_to_swap: u64) -> Result<()> {
    //     instructions::swap(ctx, amount_to_swap)
    // }
}

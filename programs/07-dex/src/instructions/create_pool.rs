use anchor_lang::prelude::*;

use crate::LiquidityPool;

pub fn create_pool(ctx: Context<CreatePool>) -> Result<()> {
    ctx.accounts.pool.set_inner(LiquidityPool::new(ctx.bumps.pool));
    Ok(())
}
#[derive(Accounts)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = LiquidityPool::SPACE,
        seeds = [LiquidityPool::SEED_PREFIX.as_bytes()],
        bump
    )]
    pub pool: Account<'info, LiquidityPool>,
    pub system_program: Program<'info, System>,
}

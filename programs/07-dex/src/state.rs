use anchor_lang::prelude::*;
/// The `LiquidityPool` state - the inner data of the program-derived address
/// that will be our Liquidity Pool
#[account]
#[derive(InitSpace)]
pub struct LiquidityPool {
    #[max_len(0)]
    pub assets: Vec<Pubkey>,
    pub bump: u8,
}

impl LiquidityPool {
    /// The Liquidity Pool's seed prefix, and in this case the only seed used to
    /// derive it's program-derived address
    pub const SEED_PREFIX: &'static str = "liquidity_pool";

    /// Anchor discriminator + Vec (empty) + u8
    pub const SPACE: usize = 8 + LiquidityPool::INIT_SPACE;

    /// Creates a new `LiquidityPool1 state
    pub fn new(bump: u8) -> Self {
        Self {
            assets: vec![],
            bump,
        }
    }
}
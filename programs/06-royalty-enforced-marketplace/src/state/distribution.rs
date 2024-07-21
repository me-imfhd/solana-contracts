use anchor_lang::prelude::*;

#[derive(Clone, AnchorSerialize, AnchorDeserialize, InitSpace)]
pub struct Creator {
    /// creator address
    pub address: Pubkey,
    /// token amount that creator can claim
    pub claim_amount: u64,
}

#[account()]
#[derive(InitSpace)]
pub struct DistributionAccount {
    /// group to which the distribution account belongs to
    pub mint: Pubkey,
    #[max_len(1)] // initial length
    pub creators_claims: Vec<Creator>,
}

pub const DISTRIBUTION_NO_CREATORS_SPACE: usize = 8 + DistributionAccount::INIT_SPACE - Creator::INIT_SPACE;
pub const DISTRIBUTION_ACCOUNT_MIN_LEN: usize = DistributionAccount::INIT_SPACE + 8;

impl DistributionAccount {
    pub fn initialize_account_data(&mut self, mint: Pubkey) {
        self.mint = mint;
        self.creators_claims = vec![];
    }
}

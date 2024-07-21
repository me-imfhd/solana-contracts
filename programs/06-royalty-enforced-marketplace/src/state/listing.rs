use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct ListedNftPda {
    pub bump: u8,
    pub price: u64,
    pub seller: Pubkey,
    pub seller_ata: Pubkey,
    pub mint: Pubkey,
}

#[account]
#[derive(InitSpace)]
pub struct EnforcingAccount {
    pub slot: u64,
}

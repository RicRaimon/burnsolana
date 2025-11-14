use anchor_lang::prelude::*;

#[account]
pub struct UserStats {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub total_amount: u64,
    pub burn_count: u64,
    pub bump: u8,
}

impl UserStats {
    pub const MAX_SIZE: usize = 32 + 32 + 8 + 8 + 1;
}

#[account]
pub struct FeeVault {} 

#[event]
pub struct BurnEvent {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub message: String,
    pub timestamp: i64,
}
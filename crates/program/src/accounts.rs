use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

use crate::events::Side;

#[derive(Debug, BorshDeserialize)]
pub struct PredictionEvent {
    pub id: Pubkey,
    pub creator: Pubkey,
    pub bump: u8,
    pub start_date: u64,
    pub end_date: u64,
    pub left_pool: u64,
    pub right_pool: u64,
    pub left_mint: Option<Pubkey>,
    pub right_mint: Option<Pubkey>,
    pub result: Option<Side>,
    pub burning: bool,
}

#[derive(Debug, BorshDeserialize)]
pub struct Ticket {
    pub creator: Pubkey,
    pub amount: u64,
    pub selection: Side,
}

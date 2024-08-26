use borsh::BorshDeserialize;
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, BorshDeserialize)]
pub struct DeployEvtEvent {
    pub key: Pubkey,
    pub id: Pubkey,
    pub bump: u8,
    pub title: String,
    pub description: String,
    pub left_description: String,
    pub right_description: String,
    pub creator: Pubkey,
    pub end_date: u64,
    pub start_date: u64,
    pub burning: bool,
    pub left_mint: Option<Pubkey>,
    pub right_mint: Option<Pubkey>,
}

#[derive(Debug, BorshDeserialize)]
pub struct FinishEvtEvent {
    pub event_id: Pubkey,
    pub result: Side,
}

#[derive(Debug, BorshDeserialize)]
pub struct VoteEvtEvent {
    pub key: Pubkey,
    pub event_id: Pubkey,
    pub creator: Pubkey,
    pub selection: Side,
    pub amount: u64,
}

#[derive(BorshDeserialize, Copy, Clone, PartialEq, Eq, Debug)]
pub enum Side {
    Left,
    Right,
}

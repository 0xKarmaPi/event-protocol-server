use base64::{prelude::BASE64_STANDARD, Engine};
use borsh::BorshDeserialize;
use std::fmt::Debug;

use crate::events::{
    ClaimRewardsEvent, CloseEvtEvent, DeployEvtEvent, FinishEvtEvent, VoteEvtEvent, WithdrawEvent,
};

const DEPLOY_EVENT: &str = "Program log: Instruction: DeployEvent";
const VOTE_EVENT: &str = "Program log: Instruction: VoteEvent";
const FINISH_EVENT: &str = "Program log: Instruction: FinishEvent";
const CLOSE_EVENT: &str = "Program log: Instruction: CloseEvent";
const CLAIM_REWARDS: &str = "Program log: Instruction: ClaimRewards";
const WITHDRAW: &str = "Program log: Instruction: WithdrawDeposited";

const PRGOGRAM_DATA: &str = "Program data: ";

#[derive(Debug)]
pub enum Event {
    VoteEvent(VoteEvtEvent),
    DeployEvent(DeployEvtEvent),
    FinishEvent(FinishEvtEvent),
    CloseEvent(CloseEvtEvent),
    ClaimRewards(ClaimRewardsEvent),
    Withdraw(WithdrawEvent),
}

/// https://book.anchor-lang.com/anchor_bts/discriminator.html
pub fn parse_logs(logs: Vec<String>) -> Option<Event> {
    let instruction = logs.iter().find(|log| filter(log))?.as_str();

    let data = logs
        .iter()
        .find(|log| log.starts_with(PRGOGRAM_DATA))
        .and_then(|log| log.strip_prefix(PRGOGRAM_DATA))?;

    let borsh_bytes = &BASE64_STANDARD.decode(data).ok()?[8..];

    match instruction {
        DEPLOY_EVENT => from_bytes(borsh_bytes).map(Event::DeployEvent),
        VOTE_EVENT => from_bytes(borsh_bytes).map(Event::VoteEvent),
        FINISH_EVENT => from_bytes(borsh_bytes).map(Event::FinishEvent),
        CLOSE_EVENT => from_bytes(borsh_bytes).map(Event::CloseEvent),
        CLAIM_REWARDS => from_bytes(borsh_bytes).map(Event::ClaimRewards),
        WITHDRAW => from_bytes(borsh_bytes).map(Event::Withdraw),
        _ => None,
    }
}

fn from_bytes<E: BorshDeserialize>(buffer: &[u8]) -> Option<E> {
    E::try_from_slice(buffer).ok()
}

fn filter(log: &str) -> bool {
    log == DEPLOY_EVENT
        || log == VOTE_EVENT
        || log == FINISH_EVENT
        || log == CLOSE_EVENT
        || log == CLAIM_REWARDS
        || log == WITHDRAW
}

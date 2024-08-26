use base64::{prelude::BASE64_STANDARD, Engine};
use borsh::BorshDeserialize;
use std::fmt::Debug;

use crate::events::{DeployEvtEvent, FinishEvtEvent, VoteEvtEvent};

const DEPLOY_EVENT_LOG: &str = "Program log: Instruction: DeployEvent";
const VOTE_EVENT_LOG: &str = "Program log: Instruction: VoteEvent";
const FINISH_EVENT_LOG: &str = "Program log: Instruction: FinishEvent";
const PRGOGRAM_DATA: &str = "Program data: ";

#[derive(Debug)]
pub enum Event {
    VoteEvent(VoteEvtEvent),
    DeployEvent(DeployEvtEvent),
    FinishEvent(FinishEvtEvent),
}

/// https://book.anchor-lang.com/anchor_bts/discriminator.html
pub fn parse_logs(logs: Vec<String>) -> Option<Event> {
    let instruction = logs.iter().find(|log| filter(*log))?.as_str();

    let data = logs
        .iter()
        .find(|log| log.starts_with(PRGOGRAM_DATA))
        .and_then(|log| log.strip_prefix(PRGOGRAM_DATA))?;

    let borsh_bytes = &BASE64_STANDARD.decode(data).ok()?[8..];

    match instruction {
        DEPLOY_EVENT_LOG => from_bytes(borsh_bytes).map(Event::DeployEvent),
        VOTE_EVENT_LOG => from_bytes(borsh_bytes).map(Event::VoteEvent),
        FINISH_EVENT_LOG => from_bytes(borsh_bytes).map(Event::FinishEvent),
        _ => None,
    }
}

fn from_bytes<E: BorshDeserialize>(buffer: &[u8]) -> Option<E> {
    E::try_from_slice(buffer).ok()
}

fn filter(log: &str) -> bool {
    log == DEPLOY_EVENT_LOG || log == VOTE_EVENT_LOG || log == FINISH_EVENT_LOG
}

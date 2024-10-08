use database::{
    native_enums::Network,
    repositories::{prediction_event, ticket},
    sea_orm::DatabaseConnection,
};
use program::{
    accounts::PredictionEvent,
    error::ProgramError,
    events::{
        ClaimRewardsEvent, CloseEvtEvent, DeployEvtEvent, FinishEvtEvent, VoteEvtEvent,
        WithdrawEvent,
    },
};
use solana_client::nonblocking::rpc_client::RpcClient;

use crate::error::ScannerError;

pub async fn process_deploy_event(
    db: &DatabaseConnection,
    network: Network,
    client: &RpcClient,
    event: DeployEvtEvent,
    block_time: i64,
) -> Result<(), ScannerError> {
    match program::deserialize_account::<PredictionEvent>(client, &event.key).await {
        Err(error) => {
            if let ProgramError::AccountNotFound(_) = error {
                Ok(())
            } else {
                Err(error.into())
            }
        }
        Ok(prediction_event) => {
            prediction_event::create_from_account(db, network, event, prediction_event, block_time)
                .await?;
            Ok(())
        }
    }
}

pub async fn process_vote_event(
    db: &DatabaseConnection,
    network: Network,
    client: &RpcClient,
    event: VoteEvtEvent,
    block_time: i64,
) -> Result<(), ScannerError> {
    let account = program::deserialize_account(client, &event.ticket_key).await?;
    ticket::create_or_update_amount_from_account(db, network, event, account, block_time).await?;

    Ok(())
}

pub async fn process_finish_event(
    db: &DatabaseConnection,
    network: Network,
    event: FinishEvtEvent,
) -> Result<(), ScannerError> {
    prediction_event::set_result(db, network, event).await?;
    Ok(())
}

pub async fn process_close_event(
    db: &DatabaseConnection,
    network: Network,
    event: CloseEvtEvent,
) -> Result<(), ScannerError> {
    prediction_event::delete(db, network, &event.key.to_string()).await?;
    Ok(())
}

pub async fn process_claim_reward(
    db: &DatabaseConnection,
    network: Network,
    event: ClaimRewardsEvent,
) -> Result<(), ScannerError> {
    ticket::set_claimed_by_pubkey(db, network, &event.ticket_key.to_string()).await?;
    Ok(())
}

pub async fn process_withdraw(
    db: &DatabaseConnection,
    network: Network,
    event: WithdrawEvent,
) -> Result<(), ScannerError> {
    ticket::set_withdrawn_by_pubkey(db, network, &event.ticket_key.to_string()).await?;
    Ok(())
}

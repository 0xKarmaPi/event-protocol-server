use database::sea_orm::DatabaseConnection;
use program::{
    accounts::PredictionEvent,
    events::{
        ClaimRewardsEvent, CloseEvtEvent, DeployEvtEvent, FinishEvtEvent, VoteEvtEvent,
        WithdrawEvent,
    },
};
use solana_client::nonblocking::rpc_client::RpcClient;

use crate::error::ScannerError;

// let account = program::deserialize_account::<PredictionEvent>(
//     &client,
//     &Pubkey::from_str("2Fm2TWAnX1JDfJ1kLbozjJsg58aCGJhKYXtRNBSwCYs6")?,
// )
// .await
// .unwrap();

pub async fn process_deploy_event(
    db: &DatabaseConnection,
    client: &RpcClient,
    event: &DeployEvtEvent,
    signature: &str,
) -> Result<(), ScannerError> {
    let prediction_event: PredictionEvent =
        program::deserialize_account(client, &event.key).await?;
    Ok(())
}

pub async fn process_vote_event(
    db: &DatabaseConnection,
    event: &VoteEvtEvent,
    signature: &str,
) -> Result<(), ScannerError> {
    Ok(())
}

pub async fn process_finish_event(
    db: &DatabaseConnection,
    event: &FinishEvtEvent,
    signature: &str,
) -> Result<(), ScannerError> {
    Ok(())
}

pub async fn process_close_event(
    db: &DatabaseConnection,
    event: &CloseEvtEvent,
    signature: &str,
) -> Result<(), ScannerError> {
    Ok(())
}

pub async fn process_claim_reward(
    db: &DatabaseConnection,
    event: &ClaimRewardsEvent,
    signature: &str,
) -> Result<(), ScannerError> {
    Ok(())
}

pub async fn process_withdraw(
    db: &DatabaseConnection,
    event: &WithdrawEvent,
    signature: &str,
) -> Result<(), ScannerError> {
    Ok(())
}

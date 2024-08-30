use crate::error::StreamError;
use database::{
    repositories::{prediction_event, ticket},
    sea_orm::DatabaseConnection,
};
use program::events::{
    ClaimRewardsEvent, CloseEvtEvent, DeployEvtEvent, FinishEvtEvent, VoteEvtEvent, WithdrawEvent,
};

pub async fn _on_deploy_event(
    db: &DatabaseConnection,
    event: DeployEvtEvent,
) -> Result<(), StreamError> {
    tracing::info!("deploy event: {:#?}", event);

    prediction_event::create(db, event).await?;
    Ok(())
}

pub async fn _on_vote_event(
    db: &DatabaseConnection,
    event: VoteEvtEvent,
) -> Result<(), StreamError> {
    tracing::info!("vote event: {:#?}", event);

    ticket::create_or_update_amount(db, event).await?;
    Ok(())
}

pub async fn _on_finish_event(
    db: &DatabaseConnection,
    event: FinishEvtEvent,
) -> Result<(), StreamError> {
    tracing::info!("finish event: {:#?}", event);

    prediction_event::set_result(db, event).await?;
    Ok(())
}

pub async fn _on_close_event(
    db: &DatabaseConnection,
    event: CloseEvtEvent,
) -> Result<(), StreamError> {
    tracing::info!("close event: {:#?}", event);

    prediction_event::delete(db, &event.key.to_string()).await?;
    Ok(())
}

pub async fn _on_claim_rewards(
    db: &DatabaseConnection,
    event: ClaimRewardsEvent,
) -> Result<(), StreamError> {
    tracing::info!("claims event: {:#?}", event);

    ticket::set_claimed_by_pubkey(db, &event.ticket_key.to_string()).await?;
    Ok(())
}

pub async fn _on_withdraw(
    db: &DatabaseConnection,
    event: WithdrawEvent,
) -> Result<(), StreamError> {
    tracing::info!("withdraw event: {:#?}", event);

    ticket::set_withdrawn_by_pubkey(db, &event.ticket_key.to_string()).await?;
    Ok(())
}

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
    prediction_event::create(db, event).await?;
    Ok(())
}

pub async fn _on_vote_event(
    db: &DatabaseConnection,
    event: VoteEvtEvent,
) -> Result<(), StreamError> {
    ticket::create_or_update_amount(db, event).await?;
    Ok(())
}

pub async fn _on_finish_event(
    db: &DatabaseConnection,
    event: FinishEvtEvent,
) -> Result<(), StreamError> {
    prediction_event::set_result(db, event).await?;
    Ok(())
}

pub async fn _on_close_event(
    db: &DatabaseConnection,
    event: CloseEvtEvent,
) -> Result<(), StreamError> {
    prediction_event::close(db, &event.key.to_string()).await?;
    Ok(())
}

pub async fn _on_claim_rewards(
    db: &DatabaseConnection,
    event: ClaimRewardsEvent,
) -> Result<(), StreamError> {
    ticket::set_claimed_by_pubkey(db, &event.ticket_key.to_string()).await?;
    Ok(())
}

pub async fn _on_withdraw(
    db: &DatabaseConnection,
    event: WithdrawEvent,
) -> Result<(), StreamError> {
    ticket::set_withdrawn_by_pubkey(db, &event.ticket_key.to_string()).await?;
    Ok(())
}

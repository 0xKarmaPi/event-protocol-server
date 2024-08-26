use crate::error::StreamError;
use database::{
    native_enums::{Context, Event},
    repositories::{prediction_event, signature_snapshot, ticket},
    sea_orm::DatabaseConnection,
};
use program::events::{DeployEvtEvent, FinishEvtEvent, VoteEvtEvent};

pub async fn _on_deploy_event(
    db: &DatabaseConnection,
    event: DeployEvtEvent,
    signature: &str,
) -> Result<(), StreamError> {
    prediction_event::create(db, event).await?;
    signature_snapshot::create(
        db,
        signature.to_string(),
        Event::DeployEvent,
        Context::Stream,
    )
    .await?;

    println!("deploy_event done > {}", signature);
    Ok(())
}

pub async fn _on_vote_event(
    db: &DatabaseConnection,
    event: VoteEvtEvent,
    signature: &str,
) -> Result<(), StreamError> {
    ticket::create_or_update_amount(db, event).await?;
    signature_snapshot::create(db, signature.to_string(), Event::VoteEvent, Context::Stream)
        .await?;

    println!("vote_event done > {}", signature);
    Ok(())
}

pub async fn _on_finish_event(
    db: &DatabaseConnection,
    event: FinishEvtEvent,
    signature: &str,
) -> Result<(), StreamError> {
    prediction_event::set_result(db, event).await?;
    signature_snapshot::create(
        db,
        signature.to_string(),
        Event::FinishEvent,
        Context::Stream,
    )
    .await?;

    println!("finish_event done > {}", signature);
    Ok(())
}

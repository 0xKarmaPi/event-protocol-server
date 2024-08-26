use crate::error::StreamError;
use database::{
    native_enums::{Context, Event},
    repositories::{prediction_event, signature_snapshot, ticket},
    sea_orm::DatabaseConnection,
};
use program::events::{
    ClaimRewardsEvent, CloseEvtEvent, DeployEvtEvent, FinishEvtEvent, VoteEvtEvent, WithdrawEvent,
};

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

pub async fn _on_close_event(
    db: &DatabaseConnection,
    event: CloseEvtEvent,
    signature: &str,
) -> Result<(), StreamError> {
    prediction_event::close(db, &event.key.to_string()).await?;
    signature_snapshot::create(
        db,
        signature.to_string(),
        Event::FinishEvent,
        Context::Stream,
    )
    .await?;

    println!("close_event done > {}", signature);

    Ok(())
}

pub async fn _on_claim_rewards(
    db: &DatabaseConnection,
    event: ClaimRewardsEvent,
    signature: &str,
) -> Result<(), StreamError> {
    ticket::set_claimed_by_pubkey(db, &event.ticket_key.to_string()).await?;
    signature_snapshot::create(
        db,
        signature.to_string(),
        Event::FinishEvent,
        Context::Stream,
    )
    .await?;

    println!("claim_rewards done > {}", signature);

    Ok(())
}

pub async fn _on_withdraw(
    db: &DatabaseConnection,
    event: WithdrawEvent,
    signature: &str,
) -> Result<(), StreamError> {
    ticket::set_withdrawn_by_pubkey(db, &event.ticket_key.to_string()).await?;
    signature_snapshot::create(
        db,
        signature.to_string(),
        Event::FinishEvent,
        Context::Stream,
    )
    .await?;

    println!("withdraw done > {}", signature);

    Ok(())
}

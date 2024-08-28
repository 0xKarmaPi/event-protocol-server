use program::{accounts::Ticket, events::VoteEvtEvent};
use sea_orm::{
    prelude::DateTimeUtc, sea_query::Expr, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    IntoActiveModel, QueryFilter, Set,
};

use crate::entities::ticket;

pub async fn create_or_update_amount(
    db: &DatabaseConnection,
    event: VoteEvtEvent,
) -> Result<(), DbErr> {
    let ticket = ticket::Entity::find_by_id(event.ticket_key.to_string())
        .one(db)
        .await?;

    if let Some(ticket) = ticket {
        let mut model = ticket.into_active_model();

        model.amount = Set(event.current_amount.into());

        ticket::Entity::update(model).exec(db).await?;
    } else {
        let model = ticket::ActiveModel {
            creator: Set(event.creator.to_string()),
            pubkey: Set(event.ticket_key.to_string()),
            amount: Set(event.current_amount.into()),
            selection: Set(event.selection.into()),
            event_pubkey: Set(event.event_key.to_string()),
            claimed: Set(false),
            withdrawn: Set(false),
            created_date: Default::default(),
        };

        ticket::Entity::insert(model).exec(db).await?;
    }

    Ok(())
}

pub async fn set_claimed_by_pubkey(db: &DatabaseConnection, pubkey: &str) -> Result<(), DbErr> {
    ticket::Entity::update_many()
        .col_expr(ticket::Column::Claimed, Expr::value(true))
        .filter(ticket::Column::Pubkey.eq(pubkey))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn set_withdrawn_by_pubkey(db: &DatabaseConnection, pubkey: &str) -> Result<(), DbErr> {
    ticket::Entity::update_many()
        .col_expr(ticket::Column::Withdrawn, Expr::value(true))
        .filter(ticket::Column::Pubkey.eq(pubkey))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn create_or_update_amount_from_account(
    db: &DatabaseConnection,
    event: VoteEvtEvent,
    account: Ticket,
    block_time: i64,
) -> Result<(), DbErr> {
    let ticket = ticket::Entity::find_by_id(event.ticket_key.to_string())
        .one(db)
        .await?;

    if let Some(ticket) = ticket {
        let mut model = ticket.into_active_model();

        model.amount = Set(account.amount.into());

        ticket::Entity::update(model).exec(db).await?;
    } else {
        let created_date = DateTimeUtc::from_timestamp(block_time, 0)
            .ok_or(DbErr::Custom("invalid date block_time".to_string()))?;

        let model = ticket::ActiveModel {
            pubkey: Set(event.ticket_key.to_string()),
            event_pubkey: Set(event.event_key.to_string()),

            creator: Set(account.creator.to_string()),
            amount: Set(account.amount.into()),
            selection: Set(account.selection.into()),
            claimed: Set(account.claimed),
            withdrawn: Set(account.withdrawn),
            created_date: Set(created_date.into()),
        };

        ticket::Entity::insert(model).exec(db).await?;
    }

    Ok(())
}

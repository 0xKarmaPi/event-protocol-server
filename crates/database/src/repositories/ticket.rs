use program::events::VoteEvtEvent;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, Set};

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
            created_date: Default::default(),
        };

        ticket::Entity::insert(model).exec(db).await?;
    }

    Ok(())
}

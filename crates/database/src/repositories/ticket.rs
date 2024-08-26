use program::events::VoteEvtEvent;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, IntoActiveModel, Set};

use crate::entities::ticket;

pub async fn create_or_update_amount(
    db: &DatabaseConnection,
    event: VoteEvtEvent,
) -> Result<(), DbErr> {
    let ticket = ticket::Entity::find_by_id(event.key.to_string())
        .one(db)
        .await?;

    if let Some(ticket) = ticket {
        let mut model = ticket.into_active_model();

        let amount = model.amount.as_ref();

        let new_amount = amount
            .checked_add(event.amount.into())
            .ok_or(DbErr::Custom(
                "fail to add amount on updating ticket".to_string(),
            ))?;

        model.amount = Set(new_amount);

        ticket::Entity::update(model).exec(db).await?;
    } else {
        let model = ticket::ActiveModel {
            creator: Set(event.creator.to_string()),
            pubkey: Set(event.key.to_string()),
            amount: Set(event.amount.into()),
            selection: Set(event.selection.into()),
        };

        ticket::Entity::insert(model).exec(db).await?;
    }

    Ok(())
}

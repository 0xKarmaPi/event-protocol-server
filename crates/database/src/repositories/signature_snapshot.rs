use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Set};

use crate::{
    entities::signature_snapshot,
    native_enums::{Context, Event},
};

pub async fn create(
    db: &DatabaseConnection,
    signature: String,
    event: Event,
    context: Context,
) -> Result<(), DbErr> {
    signature_snapshot::Entity::insert(signature_snapshot::ActiveModel {
        signature: Set(signature),
        event: Set(event),
        context: Set(context),
        created_date: Default::default(),
    })
    .exec(db)
    .await?;

    Ok(())
}

pub async fn find_by_signature(
    db: &DatabaseConnection,
    signature: &str,
) -> Result<Option<signature_snapshot::Model>, DbErr> {
    signature_snapshot::Entity::find_by_id(signature)
        .one(db)
        .await
}

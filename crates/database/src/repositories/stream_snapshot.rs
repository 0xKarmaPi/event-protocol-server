use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Set};

use crate::{entities::stream_snapshot, native_enums::Event};

pub async fn create(db: &DatabaseConnection, signature: String, event: Event) -> Result<(), DbErr> {
    stream_snapshot::Entity::insert(stream_snapshot::ActiveModel {
        signature: Set(signature),
        event: Set(event),
        date: Set(Default::default()),
    })
    .exec(db)
    .await?;

    Ok(())
}

pub async fn find_by_signature(
    db: &DatabaseConnection,
    signature: &str,
) -> Result<Option<stream_snapshot::Model>, DbErr> {
    stream_snapshot::Entity::find_by_id(signature).one(db).await
}

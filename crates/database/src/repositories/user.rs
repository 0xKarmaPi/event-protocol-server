use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Set};

use crate::entities::user;

pub async fn create_if_not_exist(db: &DatabaseConnection, pubkey: String) -> Result<(), DbErr> {
    let user = user::Entity::find_by_id(&pubkey).one(db).await?;

    if user.is_none() {
        user::Entity::insert(user::ActiveModel {
            pubkey: Set(pubkey),
        })
        .exec(db)
        .await?;
    }

    Ok(())
}

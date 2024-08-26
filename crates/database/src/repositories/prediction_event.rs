use program::events::{DeployEvtEvent, FinishEvtEvent};
use sea_orm::{
    prelude::DateTimeUtc, ActiveEnum, ColumnTrait, DatabaseConnection, DbErr, EntityTrait,
    QueryFilter, QueryOrder, QuerySelect, QueryTrait, Set,
};

use crate::{entities::prediction_event, models::Count, native_enums::Side};

pub async fn create(db: &DatabaseConnection, event: DeployEvtEvent) -> Result<(), DbErr> {
    let start_date = DateTimeUtc::from_timestamp(event.start_date as i64, 0)
        .ok_or(DbErr::Custom("invlaid date start_date".to_string()))?;

    let end_date = DateTimeUtc::from_timestamp(event.end_date as i64, 0)
        .ok_or(DbErr::Custom("invlaid date end_date".to_string()))?;

    let model = prediction_event::ActiveModel {
        id: Set(event.id.to_string()),
        pubkey: Set(event.key.to_string()),
        title: Set(event.title),
        description: Set(event.description),
        left_description: Set(event.left_description),
        right_description: Set(event.right_description),
        left_mint: Set(event.left_mint.map(|mint| mint.to_string())),
        right_mint: Set(event.right_mint.map(|mint| mint.to_string())),
        start_date: Set(start_date.into()),
        end_date: Set(end_date.into()),
        creator: Set(event.creator.to_string()),
        burning: Set(event.burning),
        result: Set(None),
        created_date: Set(Default::default()),
    };

    prediction_event::Entity::insert(model).exec(db).await?;

    Ok(())
}

pub async fn set_result(db: &DatabaseConnection, event: FinishEvtEvent) -> Result<(), DbErr> {
    let result: Side = event.result.into();

    prediction_event::Entity::update_many()
        .col_expr(prediction_event::Column::Result, result.as_enum())
        .filter(prediction_event::Column::Id.eq(event.event_id.to_string()))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn find(
    db: &DatabaseConnection,
    page: u64,
    limit: u64,
    creator: Option<String>,
) -> Result<(Vec<prediction_event::Model>, i64), DbErr> {
    let events = prediction_event::Entity::find()
        .apply_if(creator.as_ref(), |query, creator| {
            query.filter(prediction_event::Column::Creator.eq(creator))
        })
        .limit(limit)
        .offset((page - 1) * limit)
        .order_by_desc(prediction_event::Column::CreatedDate)
        .all(db)
        .await?;

    let total = prediction_event::Entity::find()
        .select_only()
        .column_as(prediction_event::Column::Id.count(), "count")
        .apply_if(creator, |query, creator| {
            query.filter(prediction_event::Column::Creator.eq(creator))
        })
        .into_model::<Count>()
        .one(db)
        .await?
        .unwrap_or_default()
        .count;

    Ok((events, total))
}

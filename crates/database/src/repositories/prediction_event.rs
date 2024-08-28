use program::{
    accounts::PredictionEvent,
    events::{DeployEvtEvent, FinishEvtEvent},
};
use sea_orm::{
    prelude::DateTimeUtc, sea_query::Expr, ActiveEnum, ColumnTrait, DatabaseConnection, DbErr,
    EntityTrait, QueryFilter, QueryOrder, QuerySelect, QueryTrait, Set,
};

use crate::{entities::prediction_event, models::Count, native_enums::Side};

pub async fn create(db: &DatabaseConnection, event: DeployEvtEvent) -> Result<(), DbErr> {
    let start_date = DateTimeUtc::from_timestamp(event.start_date as i64, 0)
        .ok_or(DbErr::Custom("invalid date start_date".to_string()))?;

    let end_date = DateTimeUtc::from_timestamp(event.end_date as i64, 0)
        .ok_or(DbErr::Custom("invalid date end_date".to_string()))?;

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
        created_date: Default::default(),
        deleted: Default::default(),
    };

    prediction_event::Entity::insert(model).exec(db).await?;

    Ok(())
}

pub async fn set_result(db: &DatabaseConnection, event: FinishEvtEvent) -> Result<(), DbErr> {
    let result: Side = event.result.into();

    prediction_event::Entity::update_many()
        .col_expr(prediction_event::Column::Result, result.as_enum())
        .filter(prediction_event::Column::Pubkey.eq(event.key.to_string()))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn close(db: &DatabaseConnection, pubkey: &str) -> Result<(), DbErr> {
    prediction_event::Entity::update_many()
        .col_expr(prediction_event::Column::Deleted, Expr::value(true))
        .filter(prediction_event::Column::Pubkey.eq(pubkey))
        .exec(db)
        .await?;

    Ok(())
}

pub async fn create_from_account(
    db: &DatabaseConnection,
    event: DeployEvtEvent,
    account: PredictionEvent,
    block_time: i64,
) -> Result<(), DbErr> {
    let record = prediction_event::Entity::find_by_id(account.id.to_string())
        .one(db)
        .await?;

    let start_date = DateTimeUtc::from_timestamp(account.start_date as i64, 0)
        .ok_or(DbErr::Custom("invalid date start_date".to_string()))?;

    let end_date = DateTimeUtc::from_timestamp(account.end_date as i64, 0)
        .ok_or(DbErr::Custom("invalid date end_date".to_string()))?;

    let created_date = DateTimeUtc::from_timestamp(block_time, 0)
        .ok_or(DbErr::Custom("invalid date block_time".to_string()))?;

    if record.is_none() {
        let model = prediction_event::ActiveModel {
            pubkey: Set(event.key.to_string()),
            title: Set(event.title.to_string()),
            description: Set(event.description.to_string()),
            left_description: Set(event.left_description.to_string()),
            right_description: Set(event.right_description.to_string()),

            id: Set(account.id.to_string()),
            left_mint: Set(account.left_mint.map(|mint| mint.to_string())),
            right_mint: Set(account.right_mint.map(|mint| mint.to_string())),
            start_date: Set(start_date.into()),
            end_date: Set(end_date.into()),

            creator: Set(account.creator.to_string()),
            burning: Set(account.burning),
            result: Set(account.result.map(Into::into)),
            created_date: Set(created_date.into()),
            deleted: Default::default(),
        };

        prediction_event::Entity::insert(model).exec(db).await?;
    }

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

pub async fn find_by_id(
    db: &DatabaseConnection,
    id: &str,
) -> Result<Option<prediction_event::Model>, DbErr> {
    prediction_event::Entity::find_by_id(id).one(db).await
}

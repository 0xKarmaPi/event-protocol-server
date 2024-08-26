use crate::entities::prediction_event;

pub use prediction_event::Model as PredictionEvent;
use sea_orm::FromQueryResult;

#[derive(FromQueryResult, Default)]
pub struct Count {
    pub count: i64,
}

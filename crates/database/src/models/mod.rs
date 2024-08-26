use crate::entities::prediction_event;
use sea_orm::FromQueryResult;

pub type PredictionEvent = prediction_event::Model;

#[derive(FromQueryResult, Default)]
pub struct Count {
    pub count: i64,
}

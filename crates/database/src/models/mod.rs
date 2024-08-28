use crate::entities::{prediction_event, ticket};
use sea_orm::FromQueryResult;

pub type PredictionEvent = prediction_event::Model;
pub type Ticket = ticket::Model;

#[derive(FromQueryResult, Default)]
pub struct Count {
    pub count: i64,
}

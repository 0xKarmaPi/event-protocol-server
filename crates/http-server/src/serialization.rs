use database::models::PredictionEvent;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[aliases(PaginatedEvents = PaginatedData<PredictionEvent>)]
pub struct PaginatedData<T> {
    pub page: u64,
    pub total: i64,
    pub nodes: Vec<T>,
}

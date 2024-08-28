use crate::handlers::PredictionEventWithTickets;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[aliases(PaginatedEventsWithTickets = PaginatedData<PredictionEventWithTickets>)]
pub struct PaginatedData<T> {
    pub page: u64,
    pub total: i64,
    pub nodes: Vec<T>,
}

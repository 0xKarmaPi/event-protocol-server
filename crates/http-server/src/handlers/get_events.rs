use crate::{
    error::HttpException,
    extractors::{state::Db, validation::ValidatedQuery},
    serialization::PaginatedData,
};
use axum::Json;
use database::{models::PredictionEvent, repositories::prediction_event};
use serde::Deserialize;
use utoipa::IntoParams;
use validator::Validate;

#[derive(Deserialize, IntoParams, Validate)]
#[into_params(parameter_in = Query)]
pub struct GetEventsParams {
    #[validate(range(min = 1))]
    page: u64,

    #[validate(range(min = 1, max = 60))]
    limit: u64,

    #[validate(length(min = 1))]
    creator: Option<String>,
}

#[utoipa::path(
  get,
  params(
    GetEventsParams
  ),
  path = "/api/events",
  tag = "Event",
  responses(
      (status = 200, description = "return prediction event list", body = PaginatedEvents)
  )
)]
pub async fn get_events(
    Db(db): Db,
    ValidatedQuery(GetEventsParams {
        page,
        limit,
        creator,
    }): ValidatedQuery<GetEventsParams>,
) -> Result<Json<PaginatedData<PredictionEvent>>, HttpException> {
    let (events, total) = prediction_event::find(&db, page, limit, creator).await?;

    let response = PaginatedData {
        page,
        total,
        nodes: events,
    };

    Ok(Json(response))
}

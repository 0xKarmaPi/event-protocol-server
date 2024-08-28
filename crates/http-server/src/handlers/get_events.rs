use crate::{
    error::HttpException,
    extractors::{state::Db, validation::ValidatedQuery},
    serialization::PaginatedData,
};
use axum::Json;
use database::{
    models::{PredictionEvent, Ticket},
    repositories::prediction_event,
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
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

    #[validate(length(min = 1))]
    predictor: Option<String>,
}

#[utoipa::path(
  get,
  params(
    GetEventsParams
  ),
  path = "/api/events",
  tag = "Event",
  responses(
      (status = 200, description = "return prediction event list", body = PaginatedEventsWithTickets)
  )
)]
pub async fn get_events(
    Db(db): Db,
    ValidatedQuery(GetEventsParams {
        page,
        limit,
        creator,
        predictor,
    }): ValidatedQuery<GetEventsParams>,
) -> Result<Json<PaginatedData<PredictionEventWithTickets>>, HttpException> {
    let (events, total, tickets) =
        prediction_event::find(&db, creator, predictor, page, limit).await?;

    let events = if let Some(tickes) = tickets {
        events
            .into_iter()
            .zip(tickes)
            .map(|(event, tickets)| PredictionEventWithTickets { event, tickets })
            .collect()
    } else {
        events
            .into_iter()
            .map(|event| PredictionEventWithTickets {
                event,
                tickets: vec![],
            })
            .collect()
    };

    let response = PaginatedData {
        page,
        total,
        nodes: events,
    };

    Ok(Json(response))
}

#[derive(Serialize, ToSchema)]
pub struct PredictionEventWithTickets {
    #[serde(flatten)]
    pub event: PredictionEvent,
    pub tickets: Vec<Ticket>,
}

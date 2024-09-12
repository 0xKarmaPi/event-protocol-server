use crate::{
    error::HttpException,
    extractors::{state::Db, validation::ValidatedPath},
};
use axum::Json;
use database::{models::PredictionEvent, native_enums::Network, repositories::prediction_event};
use serde::Deserialize;
use utoipa::IntoParams;
use validator::Validate;

#[derive(Deserialize, Validate, IntoParams)]
#[into_params(parameter_in = Path)]
pub struct GetEventPathParam {
    #[validate(length(min = 1))]
    id: String,

    network: Network,
}

#[utoipa::path(
  get,
  path = "/api/events/{id}",
  params(GetEventPathParam),
  tag = "Event",
  responses(
      (status = 200, description = "return the prediction event by id", body = PredictionEvent)
  )
)]
pub async fn get_event(
    Db(db): Db,
    ValidatedPath(GetEventPathParam { id, network }): ValidatedPath<GetEventPathParam>,
) -> Result<Json<PredictionEvent>, HttpException> {
    prediction_event::find_by_id(&db, network, id.clone())
        .await?
        .ok_or(HttpException::BadRequest(format!(
            "event not found with id: {}",
            id
        )))
        .map(Json)
}

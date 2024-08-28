use crate::{
    error::HttpException,
    extractors::{security::Auth, state::Db},
};
use axum::Json;
use database::repositories::{prediction_event, ticket};
use serde::Serialize;
use utoipa::ToSchema;

#[utoipa::path(
  get,
  path = "/api/statistics",
  tag = "User",
  security(
    ("BearerAuth" = []),
  ),
  responses(
      (status = 200, description = "return the user's statistics", body = UserStatistics)
  )
)]
pub async fn get_user_statistics(
    Db(ref db): Db,
    Auth(claims): Auth,
) -> Result<Json<UserStatistics>, HttpException> {
    let pubkey = &claims.pubkey;

    let (total_created, total_participated, (total_win, total_lose)) = tokio::try_join!(
        prediction_event::count_total_created_by_pubkey(db, pubkey),
        ticket::count_total_created_by_pubkey(db, pubkey),
        ticket::count_total_win_and_lose_by_pubkey(db, pubkey)
    )?;

    let statistics = UserStatistics {
        total_created,
        total_lose,
        total_participated,
        total_win,
    };

    Ok(Json(statistics))
}

#[derive(Serialize, ToSchema)]
pub struct UserStatistics {
    total_created: i64,
    total_participated: i64,
    total_lose: i64,
    total_win: i64,
}

use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use database::sea_orm::{ConnectOptions, Database, DatabaseConnection};

use crate::error::HttpException;

// pub struct Redis(pub RedisConnection);
pub struct Db(pub DatabaseConnection);

#[derive(Clone)]
pub struct AppState {
    pub db: DatabaseConnection,
}

#[async_trait]
impl<S> FromRequestParts<S> for Db
where
    S: Send + Sync,
    DatabaseConnection: FromRef<S>,
{
    type Rejection = HttpException;

    async fn from_request_parts(_parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let connection = DatabaseConnection::from_ref(state);

        Ok(Self(connection))
    }
}

impl FromRef<AppState> for DatabaseConnection {
    fn from_ref(app_state: &AppState) -> Self {
        app_state.db.clone()
    }
}

impl AppState {
    pub async fn new(db_url: String) -> Result<Self, HttpException> {
        let opt = ConnectOptions::new(db_url);

        let database_connection = Database::connect(opt).await?;

        Ok(Self {
            db: database_connection,
        })
    }
}

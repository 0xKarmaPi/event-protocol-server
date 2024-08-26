#[derive(thiserror::Error, Debug)]
pub enum StreamError {
    #[error(transparent)]
    PubsubClientError(#[from] solana_client::nonblocking::pubsub_client::PubsubClientError),

    #[error(transparent)]
    DbError(#[from] database::sea_orm::DbErr),
}

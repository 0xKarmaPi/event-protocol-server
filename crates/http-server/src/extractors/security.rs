use axum::{async_trait, extract::FromRequestParts, http::request::Parts, RequestPartsExt};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{errors::ErrorKind, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::error::HttpException;

#[derive(Deserialize, Serialize, Debug)]
pub struct Claims {
    pub pubkey: String,
    pub exp: u64,
}

pub struct Auth(pub Claims);

#[async_trait]
impl<S> FromRequestParts<S> for Auth
where
    S: Send + Sync,
{
    type Rejection = HttpException;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let bearer = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| HttpException::Unauthorized("Missing Authorization".to_owned()))?;

        decode_token(bearer.token()).map(Self)
    }
}

fn decode_token(token: &str) -> Result<Claims, HttpException> {
    let access_secret = std::env::var("JWT_SECRET")?;

    jsonwebtoken::decode::<Claims>(
        token,
        &DecodingKey::from_secret(access_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|err| match err.kind() {
        ErrorKind::ExpiredSignature => HttpException::Unauthorized("Expired token".to_owned()),
        _ => HttpException::Unauthorized("Invalid token".to_owned()),
    })
    .map(|token_data| token_data.claims)
}

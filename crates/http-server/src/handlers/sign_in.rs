use std::str::FromStr;

use axum::Json;
use chrono::{Duration, Utc};
use database::repositories::user;
use jsonwebtoken::{Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;
use utoipa::ToSchema;

use crate::{
    error::HttpException,
    extractors::{security::Claims, state::Db},
};

#[derive(Deserialize, ToSchema)]
pub struct SignInPayload {
    message: String,
    signed_message: String,
    address: String,
}

#[utoipa::path(
  post,
  path = "/api/sign-in",
  request_body = SignInPayload,
  tag = "Auth",
  responses(
      (status = 200, body = Token)
  )
)]
pub async fn sign_in(
    Db(db): Db,
    Json(SignInPayload {
        address,
        message,
        signed_message,
    }): Json<SignInPayload>,
) -> Result<Json<Token>, HttpException> {
    let sig = hex::decode(signed_message)
        .map_err(|_| HttpException::Unauthorized("invalid signed message".to_string()))?;

    let pk = Pubkey::from_str(&address).map_err(|e| HttpException::Unauthorized(e.to_string()))?;

    let token = nacl::sign::verify(sig.as_slice(), message.as_bytes(), &pk.to_bytes())
        .map_err(|e| HttpException::Unauthorized(e.message))?
        .then(|| sign(pk.to_string()))
        .ok_or(HttpException::Unauthorized("invalid signature".to_string()))??;

    user::create_if_not_exist(&db, pk.to_string()).await?;

    Ok(Json(token))
}

fn sign(pubkey: String) -> Result<Token, HttpException> {
    let secret = std::env::var("JWT_SECRET")?;

    let header = Header::new(Algorithm::HS256);

    let secret_key = EncodingKey::from_secret(secret.as_bytes());

    let access_exp = Utc::now()
        .checked_add_signed(Duration::days(3))
        .ok_or(HttpException::InternalError(
            "get none from checked_add_signed()".to_string(),
        ))?
        .timestamp() as u32;

    let claims = Claims {
        exp: access_exp as u64,
        pubkey,
    };

    let access_token = jsonwebtoken::encode(&header, &claims, &secret_key)
        .map_err(|e| HttpException::Unauthorized(e.to_string()))?;

    Ok(Token { access_token })
}

#[derive(Serialize, ToSchema)]
pub struct Token {
    access_token: String,
}

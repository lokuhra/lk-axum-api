use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::{
    async_trait,
    extract::FromRequestParts,
    headers::{authorization::Bearer, Authorization},
    http::request::Parts,
    TypedHeader,
};
use jsonwebtoken::{decode, Validation};

use crate::{error::AppError, models::auth::Claims, KEYS};

// get 8 hours timestamp for jwt expiry
pub fn get_timestamp_8_hours_from_now() -> u64 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let eighthoursfromnow = since_the_epoch + Duration::from_secs(28800);
    eighthoursfromnow.as_secs()
}

// verify token and extract data from it (a kind of middleware), whenever you try to extract claims in the handle it will first run this code
#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(req: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer_token)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(req, state)
                .await
                .map_err(|_| AppError::MissingCredential)?;

        let header = decode_header(bearer_token.token())?;

        let kid = match header.kid {
            Some(k) => k,
            None => return Err(AppError::InvalidToken),
        };

        let jwks = get_jwks().await?;

        let decoded = match jwks.find(&kid) {
            Some(j) => match j.algorithm {
                AlgorithmParameters::RSA(ref rsa) => {
                    let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap();
                    let validation = Validation::new(j.common.algorithm.unwrap());

                    decode::<Claims>(bearer_token.token(), &decoding_key, &validation)
                        .map_err(AppError::MissingCredential)
                }
                _ => Err(AppError::InvalidToken),
            },
            None => Err(AppError::InvalidToken),
        }?;

        Ok(decoded.claims)
    }
}

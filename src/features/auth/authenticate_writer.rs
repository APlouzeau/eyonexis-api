use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_macros::FromRef;
use sha2::{Digest, Sha256};
use sqlx::PgPool;

use crate::{
    error::AppError,
    features::auth::{repository::PostgresAuthRepository, service::AuthService},
};

pub struct ExtractAuthToken;

impl<S> FromRequestParts<S> for ExtractAuthToken
where
    AuthService<PostgresAuthRepository>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get("authorization")
            .ok_or_else(|| AppError::Unauthorized("No Header".to_string()))?
            .to_str()
            .map_err(|_| AppError::Unauthorized("Data Type no supported".to_string()))?
            .strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("No Bearer".to_string()))?;
        println!("Token reçu (brut) : {}", token);

        AuthService::from_ref(state)
            .verify_token(token.to_string())
            .await?;
        Ok(ExtractAuthToken)
    }
}

#[sqlx::test]
async fn from_request_parts_test(pool: PgPool) -> sqlx::Result<()> {
    let device = "device_test";
    let mut hasher = Sha256::new();
    hasher.update("monbeautoken");
    let compare_hashed = hex::encode(hasher.finalize());
    sqlx::query!(
        r#"
        INSERT INTO device_tokens
        (device, token_hash)
        VALUES 
        ($1, $2)"#,
        device,
        compare_hashed
    )
    .execute(&pool)
    .await?;

    let request = axum::http::Request::builder()
        .header("authorization", format!("Bearer {}", compare_hashed))
        .body(())
        .unwrap();
    let (mut parts, _body) = request.into_parts();

    #[derive(Clone, FromRef)]
    pub struct AppState {
        pub test_service: AuthService<PostgresAuthRepository>,
    }

    let state = AppState {
        test_service: AuthService {
            repository: PostgresAuthRepository { pool: pool.clone() },
        },
    };

    let result =
        <ExtractAuthToken as FromRequestParts<AppState>>::from_request_parts(&mut parts, &state)
            .await;

    assert!(result.is_ok());
    Ok(())
}

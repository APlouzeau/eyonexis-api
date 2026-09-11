use sqlx::PgPool;

use crate::features::auth::model::AuthRequest;

#[derive(Clone)]
pub struct PostgresAuthRepository {
    pub pool: PgPool,
}

impl AuthRepository for PostgresAuthRepository {
    async fn verify_token_hashed(
        &self,
        token_received: String,
    ) -> Result<Option<AuthRequest>, sqlx::Error> {
        println!("Token reçu (brut) repo : {}", token_received);
        let token_registered = sqlx::query_as!(
            AuthRequest,
            r#"
                UPDATE device_tokens
                SET last_connected_at = now()
                WHERE token_hash = $1
                RETURNING
                token_hash
                "#,
            token_received
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(token_registered)
    }
}

#[allow(async_fn_in_trait)]
pub trait AuthRepository: Send + Sync {
    async fn verify_token_hashed(
        &self,
        token_received: String,
    ) -> Result<Option<AuthRequest>, sqlx::Error>;
}

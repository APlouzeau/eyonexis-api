use crate::{error::AppError, features::auth::repository::AuthRepository};

#[derive(Clone)]
pub struct AuthService<R: AuthRepository> {
    pub repository: R,
}

impl<R: AuthRepository> AuthService<R> {
    pub async fn verify_token(&self, token_received: String) -> Result<(), AppError> {
        println!("Token reçu (brut) service : {}", token_received);
        self.repository
            .verify_token_hashed(token_received)
            .await?
            .ok_or_else(|| AppError::Unauthorized("Non autorisé".to_string()))?;
        Ok(())
    }
}

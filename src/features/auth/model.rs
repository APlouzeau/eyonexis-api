use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct _Device {
    id_device: i32,
    device: String,
    token_hash: String,
    created_at: DateTime<Utc>,
    last_connected_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub struct AuthRequest {
    pub token_hash: String,
}

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::features::note::model::NoteBlock;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct _NoteResponse {
    pub id_note: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: String,
    pub id_folder: Uuid,
    pub blocks: Vec<NoteBlock>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct _NoteBlock {
    pub id_note_block: Uuid,
    pub id_note: Uuid,
    pub block_type: _BlockType,
    pub content: String,
    pub order_index: u32,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct _NewNoteResponse {
    pub id_note: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: String,
    pub id_folder: Uuid,
    pub blocks: Vec<NoteBlock>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, Copy)]
#[sqlx(type_name = "block", rename_all = "lowercase")]
pub enum _BlockType {
    Text,
    Code,
    Heading,
    Note,
    List,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct _NoteToListResponse {
    pub id: Uuid,
    pub note_title: String,
    pub note_folder_id: Uuid,
    pub note_subtitle: Option<String>,
}

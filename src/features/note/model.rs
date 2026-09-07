use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id_note: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub slug: String,
    pub id_folder: Uuid,
    pub blocks: Vec<NoteBlock>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NewNote {
    pub id_note: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteNote {
    pub id_note: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteBlock {
    pub id_note_block: Uuid,
    pub id_note: Uuid,
    pub block_type: BlockType,
    pub content: String,
    pub order_index: i32,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NoteToList {
    pub id: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NoteToShow {
    pub id_note: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub folder: String,
    pub slug: String,
    pub blocks: Vec<NoteBlock>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct NoteSummary {
    pub id_note: Uuid,
    pub title: String,
    pub subtitle: Option<String>,
    pub folder: String,
    pub slug: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type, Copy, PartialEq)]
#[sqlx(type_name = "block", rename_all = "lowercase")]
pub enum BlockType {
    Text,
    Code,
    Heading,
    Note,
    List,
}

#[derive(Debug, Deserialize)]
pub struct CreateNoteBlockPayload {
    pub block_type: BlockType,
    pub content: String,
    pub order_index: i32,
    pub metadata: Option<serde_json::Value>,
}
#[derive(Debug, Deserialize)]
pub struct CreateNotePayload {
    pub title: String,
    pub subtitle: Option<String>,
    pub id_folder: Uuid,
    pub slug: String,
    pub blocks: Vec<CreateNoteBlockPayload>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateInitNotePayload {
    pub title: String,
    pub id_folder: Uuid,
}

#[derive(Debug, Deserialize)]
pub struct CreateInitNoteData {
    pub id_note: Uuid,
    pub title: String,
    pub id_folder: Uuid,
    pub slug: String,
}

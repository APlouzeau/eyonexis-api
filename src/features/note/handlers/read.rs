use crate::features::note::model::{BlockType::Heading, NoteBlock, NoteToList, NoteToShow};
use axum::{
    extract::{Path, State},
    Json,
};
use axum_macros::debug_handler;
use chrono::{DateTime, Local, Utc};
use std::vec;

use uuid::Uuid;

use crate::error::AppError;
use crate::AppState;

#[debug_handler]
pub async fn get_notes_by_folder_id(
    State(state): State<AppState>,
    Path(id_folder): Path<Uuid>,
) -> Result<Json<Vec<NoteToList>>, AppError> {
    let notes = state.note_service.list_by_folder(id_folder).await?;
    Ok(Json(notes))
}

pub async fn get_note_by_id(
    State(state): State<AppState>,
    Path(id_note): Path<Uuid>,
) -> Result<Json<NoteToShow>, AppError> {
    let note = state.note_service.get_note_by_id(id_note).await?;
    Ok(Json(note))
}

pub async fn get_note_by_path(
    State(state): State<AppState>,
    Path(path): Path<String>,
) -> Result<Json<NoteToShow>, AppError> {
    println!("{}", path);
    let split_path: Vec<&str> = path.split('/').collect();
    println!("{:?}", split_path);

    let note = state.note_service.get_note_by_path(split_path).await?;

    let id_note = Uuid::new_v4();
    let id_block = Uuid::new_v4();
    let block = NoteBlock {
        id_note_block: id_block,
        block_type: Heading,
        content: "coucou".to_string(),
        id_note: id_note,
        order_index: 1,
        metadata: None,
    };
    let mut vec_block = Vec::new();
    vec_block.push(block);
    let note = NoteToShow {
        id_note: id_note,
        title: "Ma belle note".to_string(),
        subtitle: None,
        slug: "ma-belle-note".to_string(),
        blocks: vec_block,
        folder: "un dossier".to_string(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    Ok(Json(note))
}

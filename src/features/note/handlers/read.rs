use crate::features::note::model::{NoteToList, NoteToShow};
use axum::{
    extract::{Path, State},
    Json,
};
use axum_macros::debug_handler;

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
    println!("id folder : {}", id_note);
    let note = state.note_service.get_note_by_id(id_note).await?;
    Ok(Json(note))
}

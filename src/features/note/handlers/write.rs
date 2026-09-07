use axum::extract::State;
use axum::Json;
use uuid::Uuid;

use crate::features::auth::authenticate_writer::ExtractAuthToken;
use crate::features::note::model::{CreateInitNotePayload, CreateNotePayload};

use crate::error::AppError;
use crate::AppState;

pub struct NoteCreated;

pub async fn create(
    State(state): State<AppState>,
    _extract_auth_token: ExtractAuthToken,
    Json(create_note_payload): Json<CreateInitNotePayload>,
) -> Result<Json<Uuid>, AppError> {
    let id_new_note = state.note_service.create(&create_note_payload).await?;
    println!("id new note : {}", id_new_note);
    Ok(Json(id_new_note))
}

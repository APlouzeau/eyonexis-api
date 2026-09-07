use crate::AppState;
use axum::{
    routing::{get, post},
    Router,
};

use super::handlers::{read, write};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/get-folder-content/{id_folder}",
            get(read::get_notes_by_folder_id),
        )
        .route("/get-note/{id_note}", get(read::get_note_by_id))
        .route("/create", post(write::create))
}

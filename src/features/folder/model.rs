use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::features::folder::NoteToList;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FolderNode {
    pub id_folder: Uuid,
    pub folder_name: String,
    pub folder_slug: String,
    pub children: Vec<FolderNode>,
    pub notes: Vec<NoteToList>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderBranch {
    pub id_folder: Uuid,
    pub folder_name: String,
    pub parent_id: Option<Uuid>,
    pub folder_slug: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct CreateFolderPayload {
    pub folder_name: String,
    pub parent_id: Option<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

pub struct FolderContent {
    pub id_folder: Uuid,
    pub folder_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFolderData {
    pub id_folder: Uuid,
    pub folder_name: String,
    pub folder_slug: String,
    pub parent_id: Option<Uuid>,
}

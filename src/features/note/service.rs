use crate::{
    error::AppError,
    features::{
        folder::repository::{FolderRepository, PostgresFolderRepository},
        note::{
            model::{CreateInitNoteData, CreateInitNotePayload, NewNote, NoteToList, NoteToShow},
            FolderBranch,
        },
    },
};
use slug::slugify;
use std::collections::HashMap;
use uuid::Uuid;

use super::repository::NoteRepository;

#[derive(Clone)]
pub struct NoteService<R: NoteRepository> {
    pub note_repository: R,
    pub folder_repository: PostgresFolderRepository,
}

impl<R: NoteRepository> NoteService<R> {
    pub async fn list_by_folder(&self, id_folder: Uuid) -> Result<Vec<NoteToList>, sqlx::Error> {
        let notes = self.note_repository.list_by_folder(id_folder).await?;
        Ok(notes.into_iter().collect())
    }

    pub async fn get_note_by_id(&self, id_note: Uuid) -> Result<NoteToShow, sqlx::Error> {
        let note = self.note_repository.get_note_by_id(id_note).await?;
        Ok(note)
    }

    pub async fn create(&self, new_note: &CreateInitNotePayload) -> Result<Uuid, sqlx::Error> {
        let id_new_note = NewNote {
            id_note: Uuid::new_v4(),
        };
        let title_slug = slugify(&new_note.title);
        let new_note_data = CreateInitNoteData {
            id_note: id_new_note.id_note,
            title: new_note.title.clone(),
            id_folder: new_note.id_folder,
            slug: title_slug,
        };
        self.note_repository
            .create_note(&id_new_note, &new_note_data)
            .await?;

        Ok(id_new_note.id_note)
    }

    pub async fn get_note_by_path(&self, mut path: Vec<&str>) -> Result<NoteToShow, AppError> {
        let slug = path
            .last()
            .copied()
            .ok_or_else(|| AppError::NotFound("Non autorisé".to_string()))?;
        path.remove(path.len() - 1);
        let candidates = self.note_repository.find_notes_by_slug(slug).await?;
        let folders = self.folder_repository.get_folder_tree().await?;
        println!("candidates : {:?}", candidates);
        let mut folders_by_id: HashMap<Uuid, FolderBranch> = HashMap::new();

        for folder in folders {
            folders_by_id.insert(
                folder.id_folder,
                FolderBranch {
                    id_folder: folder.id_folder,
                    folder_name: folder.folder_name,
                    parent_id: folder.parent_id,
                    folder_slug: folder.folder_slug,
                },
            );
        }

        let mut notes: Vec<NoteToShow> = Vec::new();

        for candidat in candidates {
            let mut rebuild_path: Vec<String> = Vec::new();
            let mut current_id_folder: Option<Uuid> = Some(candidat.id_folder);
            while let Some(id) = current_id_folder {
                let folder = folders_by_id
                    .get(&id)
                    .ok_or_else(|| AppError::NotFound("Dossier introuvable".to_string()))?;
                rebuild_path.push(folder.folder_slug.clone());
                current_id_folder = folder.parent_id;
            }
            rebuild_path.reverse();
            if rebuild_path == path {
                let note = self
                    .note_repository
                    .get_note_by_id(candidat.id_note)
                    .await?;
                notes.push(note);
            }
        }
        println!("note trouvée : {:?}", notes);
        match notes.len() {
            0 => Err(AppError::NotFound("Aucun résultat trouve".to_string())),
            1 => Ok(notes.remove(0)),
            _ => Err(AppError::Unauthorized(
                "Plusieurs résultats trouvés".to_string(),
            )),
        }
    }

    /*     pub async fn delete(&self, id: DeleteNote) -> Result<Vec<NoteResponse>, sqlx::Error> {
        self.repository.delete(id).await?;
        self.get_all().await
    } */
}

#[cfg(test)]
mod tests {
    use std::iter::zip;

    use axum_macros::FromRef;
    use sqlx::PgPool;

    use super::*;
    use crate::features::note::{
        model::{
            BlockType::{Heading, Text},
            CreateNoteBlockPayload, _CreateNotePayload,
        },
        repository::PostgresNoteRepository,
    };

    #[sqlx::test]
    async fn create_test(pool: PgPool) -> sqlx::Result<()> {
        let id_new_folder = Uuid::new_v4();
        let mut note_blocks = vec![CreateNoteBlockPayload {
            block_type: Heading,
            content: "Titre de la section".to_string(),
            order_index: 1,
            metadata: None,
        }];
        note_blocks.push(CreateNoteBlockPayload {
            block_type: Text,
            content: "Contenu de la section".to_string(),
            order_index: 2,
            metadata: None,
        });

        let new_note_data = CreateInitNotePayload {
            title: "Test de note".to_string(),
            id_folder: id_new_folder,
        };
        let new_note = _CreateNotePayload {
            title: "Test de note".to_string(),
            subtitle: None,
            id_folder: id_new_folder,
            slug: "test-de-note".to_string(),
            blocks: note_blocks,
        };
        #[derive(Clone, FromRef)]
        pub struct AppState {
            pub test_service: NoteService<PostgresNoteRepository>,
        }

        let state = AppState {
            test_service: NoteService {
                note_repository: PostgresNoteRepository { pool: pool.clone() },
                folder_repository: PostgresFolderRepository { pool: pool.clone() },
            },
        };

        sqlx::query!(
            r#"
        INSERT INTO folders
        (id_folder, folder_name, folder_slug)
        VALUES 
        ($1, $2, $3)"#,
            id_new_folder,
            "Dossier de test",
            "dossier-de-test"
        )
        .execute(&pool)
        .await?;

        let id_note = state.test_service.create(&new_note_data).await?;
        let get_note = state.test_service.get_note_by_id(id_note).await?;

        assert_eq!(get_note.title, new_note.title);
        assert_eq!(get_note.subtitle, new_note.subtitle);
        assert_eq!(get_note.slug, new_note.slug);
        for (a, b) in zip(get_note.blocks, new_note.blocks) {
            assert_eq!(a.content, b.content);
            assert_eq!(a.order_index, b.order_index);
            assert_eq!(a.metadata, b.metadata);
            assert_eq!(a.block_type, b.block_type);
        }

        Ok(())
    }
}

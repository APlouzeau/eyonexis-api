use slug::slugify;
use uuid::Uuid;

use crate::{
    error::AppError,
    features::note::model::{
        CreateInitNoteData, CreateInitNotePayload, CreateNotePayload, NewNote, NoteToList,
        NoteToShow,
    },
};

use super::repository::NoteRepository;

#[derive(Clone)]
pub struct NoteService<R: NoteRepository> {
    pub repository: R,
}

impl<R: NoteRepository> NoteService<R> {
    pub async fn list_by_folder(&self, id_folder: Uuid) -> Result<Vec<NoteToList>, sqlx::Error> {
        let notes = self.repository.list_by_folder(id_folder).await?;
        Ok(notes.into_iter().map(NoteToList::from).collect())
    }

    pub async fn get_note_by_id(&self, id_note: Uuid) -> Result<NoteToShow, sqlx::Error> {
        let note = self.repository.get_note_by_id(id_note).await?;
        Ok(note)
    }

    pub async fn create(&self, new_note: &CreateInitNotePayload) -> Result<Uuid, sqlx::Error> {
        let id_new_note = NewNote {
            id_note: Uuid::new_v4(),
        };
        let slug = slugify(&new_note.title);
        let new_note_data = CreateInitNoteData {
            id_note: id_new_note.id_note,
            title: new_note.title.clone(),
            id_folder: new_note.id_folder,
            slug: slug,
        };
        self.repository
            .create_note(&id_new_note, &new_note_data)
            .await?;

        Ok(id_new_note.id_note)
    }

    pub async fn get_note_by_path(&self, path: Vec<&str>) -> Result<NoteToShow, AppError> {
        let slug = path
            .last()
            .copied()
            .ok_or_else(|| AppError::NotFound("Non autorisé".to_string()))?;
        let candidates = self.repository.find_notes_by_slug(slug).await?;
        let folders 
        Ok()
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
            CreateNoteBlockPayload,
        },
        repository::PostgresNoteRepository,
    };

    #[sqlx::test]
    async fn create_test(pool: PgPool) -> sqlx::Result<()> {
        let id_folder = Uuid::new_v4();
        let mut note_blocks = Vec::new();
        note_blocks.push(CreateNoteBlockPayload {
            block_type: Heading,
            content: "Titre de la section".to_string(),
            order_index: 1,
            metadata: None,
        });
        note_blocks.push(CreateNoteBlockPayload {
            block_type: Text,
            content: "Contenu de la section".to_string(),
            order_index: 2,
            metadata: None,
        });

        let new_note_data = CreateInitNotePayload {
            title: "Test de note".to_string(),
            id_folder: id_folder,
        };
        let new_note = CreateNotePayload {
            title: "Test de note".to_string(),
            subtitle: Some("Ceci est un test".to_string()),
            id_folder: id_folder,
            slug: "test-de-note".to_string(),
            blocks: note_blocks,
        };
        #[derive(Clone, FromRef)]
        pub struct AppState {
            pub test_service: NoteService<PostgresNoteRepository>,
        }

        let state = AppState {
            test_service: NoteService {
                repository: PostgresNoteRepository { pool: pool.clone() },
            },
        };

        sqlx::query!(
            r#"
        INSERT INTO folders
        (id_folder, folder_name, folder_slug)
        VALUES 
        ($1, $2, $3)"#,
            id_folder,
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

use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::features::note::model::{BlockType, CreateInitNoteData,  CreateNoteBlockPayload, NewNote, NoteBlock, NoteFromSlug, NoteSummary, NoteToList, NoteToShow};

#[derive(Clone)]
pub struct PostgresNoteRepository {
    pub pool: PgPool,
}

impl NoteRepository for PostgresNoteRepository {
    async fn list_by_folder(
        &self,
        id_folder: Uuid,
    ) -> Result<Vec<NoteToList>, sqlx::Error> {
            let notes = sqlx::query_as!(
                NoteToList,
                r#"
            SELECT id_note AS "id: uuid::Uuid", title, subtitle, slug
            FROM notes
            WHERE id_folder = $1
            "#,
                id_folder
            )
            .fetch_all(&self.pool)
            .await?;
            Ok(notes)
    }

    async fn get_note_by_id(
        &self,
        id_note: Uuid,
    ) -> Result<NoteToShow, sqlx::Error>{
            let note = sqlx::query_as!(
                NoteSummary,
                r#"
            SELECT n.id_note AS "id_note: uuid::Uuid", n.title, n.subtitle, n.created_at, n.updated_at, f.folder_name as folder, n.slug
            FROM notes n
            INNER JOIN folders f ON n.id_folder = f.id_folder
            WHERE n.id_note = $1
            
            "#,
                id_note
            )            
            .fetch_optional(&self.pool)
            .await?
            .ok_or(sqlx::Error::RowNotFound)?;

            let note_blocks = sqlx::query_as!(
            NoteBlock,
            r#"
            SELECT id_note_block AS "id_note_block: uuid::Uuid", id_note AS "id_note: uuid::Uuid", block_type AS "block_type: BlockType", content, order_index, metadata AS "metadata: serde_json::Value"
            FROM notes_blocks
            WHERE id_note = $1
            ORDER BY order_index ASC
            "#,
            id_note
        )
        .fetch_all(&self.pool)
        .await?;

            Ok(NoteToShow {
                id_note: note.id_note,
                title: note.title,
                subtitle: note.subtitle,
                folder: note.folder,
                slug : note.slug,
                created_at: note.created_at,
                updated_at: note.updated_at,
                blocks: note_blocks,
            })
        }

    async fn create_note(
        &self,
        id_new_note: &NewNote,
        new_note: &CreateInitNoteData,
    ) -> Result<(), sqlx::Error> {
            sqlx::query!(
                r#"
            INSERT INTO notes (id_note, title, slug, id_folder)
            VALUES ($1, $2, $3, $4)
            "#,
                id_new_note.id_note,
                new_note.title,
                new_note.slug,
                new_note.id_folder
            )
            .execute(&self.pool)
            .await?;

            Ok(())
    }

    
    async fn insert_note_block(
        conn: &mut PgConnection,
        id_note: &NewNote,
        note_block: &CreateNoteBlockPayload,
    ) -> Result<(), sqlx::Error> {
        let id_note_block = uuid::Uuid::new_v4();
        sqlx::query!(
            r#"
            INSERT INTO notes_blocks (id_note_block, id_note, block_type, content, order_index, metadata)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            id_note_block,
            id_note.id_note,
            note_block.block_type as BlockType, // Assure-toi que le type de block est correctement converti pour la DB
            note_block.content,
            note_block.order_index,
            note_block.metadata.as_ref().map(|m| m) // Convertit le JSON en String pour la DB
        )
        .execute(conn)
        .await?;
        Ok(())
    } 

    async fn find_notes_by_slug(
        &self,
        slug: &str
    ) -> Result<Vec<NoteFromSlug>, sqlx::Error>{
            let candidates = sqlx::query_as!(
                NoteFromSlug,
                r#"
                SELECT id_note AS "id_note: uuid::Uuid", id_folder, slug 
                FROM notes
                WHERE slug = $1
                "#,
                slug
            ).fetch_all(&self.pool).await?;

            Ok(candidates)
    }
}

#[allow(async_fn_in_trait)]
pub trait NoteRepository {
    async fn list_by_folder(
        &self,
        id_folder: Uuid,
    ) -> Result<Vec<NoteToList>, sqlx::Error>;
    async fn get_note_by_id(
        &self,
        id_note: Uuid,
    ) -> Result<NoteToShow, sqlx::Error>;
    async fn create_note(
        &self,
        id_new_note : &NewNote,
        new_note: &CreateInitNoteData,
    ) -> Result<(), sqlx::Error>;
    async fn insert_note_block(
        conn: &mut PgConnection,
        id_note: &NewNote,
        note_block: &CreateNoteBlockPayload,
    ) -> Result<(), sqlx::Error>;
    async fn find_notes_by_slug(
        &self,
        slug: &str
    ) -> Result<Vec<NoteFromSlug>, sqlx::Error>;
} 

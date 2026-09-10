use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

use crate::features::note::model::{BlockType, CreateInitNoteData, CreateInitNotePayload, CreateNoteBlockPayload, CreateNotePayload, NewNote, NoteBlock, NoteFromSlug, NoteSummary, NoteToList, NoteToShow};

#[derive(Clone)]
pub struct PostgresNoteRepository {
    pub pool: PgPool,
}

impl NoteRepository for PostgresNoteRepository {
    fn list_by_folder(
        &self,
        id_folder: Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<NoteToList>, sqlx::Error>> + Send {
        async move {
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
    }

    fn get_note_by_id(
        &self,
        id_note: Uuid,
    ) -> impl std::future::Future<Output = Result<NoteToShow, sqlx::Error>> + Send {
        async move {
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

            let blocks = sqlx::query_as!(
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
                blocks: blocks,
            })
        }
    }

/* fn create_full_note(
    &self,
    id_new_note: &NewNote,
    new_note: &CreateNotePayload,
) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send {
        async move {
            let mut tx = self.pool.begin().await?;
            Self::create_note( &mut *tx, &id_new_note, new_note ).await?;
            for note_block in &new_note.blocks {
                Self::insert_note_block(&mut  *tx, &id_new_note, &note_block).await?;
            }
            tx.commit().await?;
            Ok(())
        }
    } */

    fn create_note(
        &self,
        id_new_note: &NewNote,
        new_note: &CreateInitNoteData,
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send {
        async move {
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
    }

    
    fn insert_note_block(
        conn: &mut PgConnection,
        id_note: &NewNote,
        note_block: &CreateNoteBlockPayload,
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send {
        async move {
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
    } 

    fn find_notes_by_slug(
        &self,
        slug: &str
    ) -> impl std::future::Future<Output = Result<Vec<NoteFromSlug>, sqlx::Error>> + Send {
        async move {
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
}

pub trait NoteRepository {
    fn list_by_folder(
        &self,
        id_folder: Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<NoteToList>, sqlx::Error>> + Send;
    fn get_note_by_id(
        &self,
        id_note: Uuid,
    ) -> impl std::future::Future<Output = Result<NoteToShow, sqlx::Error>> + Send;
    fn create_note(
        &self,
        id_new_note : &NewNote,
        new_note: &CreateInitNoteData,
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send;
    fn insert_note_block(
        conn: &mut PgConnection,
        id_note: &NewNote,
        note_block: &CreateNoteBlockPayload,
    ) -> impl std::future::Future<Output = Result<(), sqlx::Error>> + Send;
    fn find_notes_by_slug(
        &self,
        slug: &str
    ) -> impl std::future::Future<Output = Result<Vec<NoteFromSlug>, sqlx::Error>> + Send;
} 

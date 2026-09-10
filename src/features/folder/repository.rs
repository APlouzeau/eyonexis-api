use sqlx::PgPool;
use uuid::Uuid;

use crate::features::folder::model::{
    CreateFolderData, CreateFolderPayload, FolderContent, FolderNode,
};

use super::model::FolderBranch;

#[derive(Clone)]
pub struct PostgresFolderRepository {
    pub pool: PgPool,
}

impl FolderRepository for PostgresFolderRepository {
    fn get_folder_tree(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<FolderBranch>, sqlx::Error>> + Send {
        async move {
            let result = sqlx::query_as!(
            FolderBranch,
            r#"
            SELECT f.id_folder as "id_folder: uuid::Uuid", f.folder_name, f.parent_id as "parent_id: uuid::Uuid", f.folder_slug
            FROM folders f
            "#
        )
        .fetch_all(&self.pool)
        .await?;

            Ok(result)
        }
    }

    fn create(
        &self,
        new_folder: CreateFolderData,
    ) -> impl std::future::Future<Output = Result<FolderBranch, sqlx::Error>> + Send {
        async move {
            sqlx::query_as!(
                CreateFolderData,
                r#"
            INSERT INTO folders
            (id_folder, folder_name, folder_slug, parent_id)
            VALUES ($1, $2, $3, $4)"#,
                new_folder.id_folder,
                new_folder.folder_name,
                new_folder.folder_slug,
                new_folder.parent_id,
            )
            .execute(&self.pool)
            .await?;

            let result = FolderBranch {
                id_folder: new_folder.id_folder,
                folder_name: new_folder.folder_name,
                parent_id: new_folder.parent_id,
                folder_slug: new_folder.folder_slug,
            };

            Ok(result)
        }
    }

    fn get_folder_content(
        &self,
        parent_id: &Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<FolderContent>, sqlx::Error>> + Send {
        async move {
            let result = sqlx::query_as!(
                FolderContent,
                r#"
            SELECT 
            f.id_folder as "id_folder: uuid::Uuid", f.folder_name
            FROM folders f
            WHERE parent_id = $1"#,
                parent_id
            )
            .fetch_all(&self.pool)
            .await?;

            Ok(result)
        }
    }
}

pub trait FolderRepository {
    fn get_folder_tree(
        &self,
    ) -> impl std::future::Future<Output = Result<Vec<FolderBranch>, sqlx::Error>> + Send;
    fn get_folder_content(
        &self,
        parent_id: &Uuid,
    ) -> impl std::future::Future<Output = Result<Vec<FolderContent>, sqlx::Error>> + Send;
    fn create(
        &self,
        new_folder: CreateFolderData,
    ) -> impl std::future::Future<Output = Result<FolderBranch, sqlx::Error>> + Send;
}

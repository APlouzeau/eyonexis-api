use capitalize::Capitalize;
use slug::slugify;
use std::collections::HashMap;
use uuid::Uuid;

use super::model::FolderNode;
use super::repository::FolderRepository;
use crate::features::folder::model::{
    CreateFolderData, CreateFolderPayload, FolderBranch, FolderContent,
};
use crate::features::note::model::NoteToList;

#[derive(Clone)]
pub struct FolderService<R: FolderRepository> {
    pub repository: R,
}

impl<R: FolderRepository> FolderService<R> {
    pub async fn get_folder_tree(&self) -> Result<Vec<FolderNode>, sqlx::Error> {
        let folders = self.repository.get_folder_tree().await?;
        let mut child_per_parent: HashMap<Option<Uuid>, Vec<FolderBranch>> = HashMap::new();
        let mut result: Vec<FolderNode> = Vec::new();

        for folder in &folders {
            child_per_parent
                .entry(folder.parent_id)
                .or_insert_with(Vec::new)
                .push(folder.clone())
        }

        if let Some(roots) = child_per_parent.get(&None) {
            for root in roots {
                result.push(build_node(root.clone(), &child_per_parent));
            }
        }

        Ok(result)
    }

    pub async fn create(
        &self,
        new_folder: &CreateFolderPayload,
    ) -> Result<FolderBranch, sqlx::Error> {
        let slug = slugify(&new_folder.folder_name);
        let new_folder_data = CreateFolderData {
            id_folder: Uuid::new_v4(),
            folder_name: new_folder.folder_name.capitalize().to_string(),
            folder_slug: slug,
            parent_id: new_folder.parent_id,
        };
        let response = self.repository.create(new_folder_data).await?;
        Ok(response)
    }

    pub async fn get_folder_content(
        &self,
        parent_id: &Uuid,
    ) -> Result<Vec<FolderContent>, sqlx::Error> {
        let folder_content = self.repository.get_folder_content(&parent_id).await?;
        Ok(folder_content)
    }

    /*pub async fn delete(&self, id: DeleteFolder) -> Result<Vec<FolderResponse>, sqlx::Error> {
        self.repository.delete(id).await?;
        self.get_all().await
    } */
}

fn build_node(
    folder: FolderBranch,
    child_per_parent: &HashMap<Option<Uuid>, Vec<FolderBranch>>,
) -> FolderNode {
    let childs = child_per_parent.get(&Some(folder.id_folder));
    let mut children: Vec<FolderNode> = Vec::new();

    if let Some(childs) = childs {
        for child in childs {
            children.push(build_node(child.clone(), child_per_parent))
        }
    }

    let notes: Vec<NoteToList> = Vec::new();

    FolderNode {
        id_folder: folder.id_folder,
        folder_name: folder.folder_name,
        children,
        notes,
    }
}

#[cfg(test)]
mod tests {
    use axum_macros::FromRef;
    use sqlx::PgPool;

    use crate::features::folder::{
        model::CreateFolderPayload, repository::PostgresFolderRepository, service::FolderService,
    };

    #[sqlx::test]
    async fn create_test(pool: PgPool) -> sqlx::Result<()> {
        let folder_to_test_data = CreateFolderPayload {
            folder_name: "Un beau dossier de test".to_string(),
            parent_id: None,
        };
        #[derive(Clone, FromRef)]
        pub struct AppState {
            pub test_service: FolderService<PostgresFolderRepository>,
        }

        let state = AppState {
            test_service: FolderService {
                repository: PostgresFolderRepository { pool: pool.clone() },
            },
        };

        let folder_to_test = state.test_service.create(&folder_to_test_data).await?;

        assert_eq!(folder_to_test_data.folder_name, folder_to_test.folder_name);
        assert_eq!(folder_to_test_data.parent_id, folder_to_test.parent_id);

        Ok(())
    }
}

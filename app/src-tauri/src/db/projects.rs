use anyhow::Result;
use super::Database;
use crate::models::entities::Project;

impl Database {
    pub async fn create_project(&self, project: Project) -> Result<Project> {
        let created: Project = self.client
            .create(("project", project.id.clone()))
            .content(project)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Failed to create project"))?;
        Ok(created)
    }

    pub async fn list_projects(&self) -> Result<Vec<Project>> {
        let projects: Vec<Project> = self.client.select("project").await?;
        Ok(projects)
    }

    pub async fn get_project(&self, id: &str) -> Result<Option<Project>> {
        let project: Option<Project> = self.client.select(("project", id)).await?;
        Ok(project)
    }

    pub async fn delete_project(&self, id: &str) -> Result<()> {
        let _: Option<Project> = self.client.delete(("project", id)).await?;
        Ok(())
    }
}

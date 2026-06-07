use anyhow::Result;
use super::Database;
use crate::models::entities::Project;

impl Database {
    pub async fn create_project(&self, project: Project) -> Result<Project> {
        let created: Project = self.client
            .create(("project", project.id.id.to_raw()))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::entities::Project;
    use chrono::Utc;
    use surrealdb::sql::Thing;

    fn sample_project(id: &str) -> Project {
        Project {
            id: Thing::from(("project", id)),
            name: format!("Project {}", id),
            description: Some("Test project".into()),
            icon: None,
            color: Some("#4F46E5".into()),
            persona_template: "default".into(),
            harness_template: "standard".into(),
            orchestration_template: "balanced".into(),
            model_small: Some("qwen2.5-3b".into()),
            model_heavy: None,
            max_iterations: 5,
            token_budget: 8192,
            security_level: 1,
            is_active: true,
            created_at: Utc::now().into(),
            updated_at: Utc::now().into(),
        }
    }

    async fn setup_db() -> Database {
        Database::init("memory").await.expect("In-memory DB should init")
    }

    #[tokio::test]
    async fn test_create_project() {
        let db = setup_db().await;
        let proj = sample_project("proj-1");
        let created = db.create_project(proj).await.unwrap();
        assert_eq!(created.id.id.to_raw(), "proj-1");
        assert_eq!(created.name, "Project proj-1");
    }

    #[tokio::test]
    async fn test_list_projects_initially_empty() {
        let db = setup_db().await;
        let projects = db.list_projects().await.unwrap();
        assert!(projects.is_empty());
    }

    #[tokio::test]
    async fn test_list_projects_after_creation() {
        let db = setup_db().await;
        db.create_project(sample_project("p1")).await.unwrap();
        db.create_project(sample_project("p2")).await.unwrap();
        let projects = db.list_projects().await.unwrap();
        assert_eq!(projects.len(), 2);
    }

    #[tokio::test]
    async fn test_get_project_found() {
        let db = setup_db().await;
        db.create_project(sample_project("p-get")).await.unwrap();
        let proj = db.get_project("p-get").await.unwrap();
        assert!(proj.is_some());
        assert_eq!(proj.unwrap().id.id.to_raw(), "p-get");
    }

    #[tokio::test]
    async fn test_get_project_not_found() {
        let db = setup_db().await;
        let proj = db.get_project("nonexistent").await.unwrap();
        assert!(proj.is_none());
    }

    #[tokio::test]
    async fn test_delete_project() {
        let db = setup_db().await;
        db.create_project(sample_project("p-del")).await.unwrap();
        db.delete_project("p-del").await.unwrap();
        let proj = db.get_project("p-del").await.unwrap();
        assert!(proj.is_none());
    }

    #[tokio::test]
    async fn test_project_fields_preserved() {
        let db = setup_db().await;
        let proj = sample_project("p-fields");
        let created = db.create_project(proj).await.unwrap();
        assert_eq!(created.security_level, 1);
        assert_eq!(created.max_iterations, 5);
        assert_eq!(created.token_budget, 8192);
        assert!(created.is_active);
        assert_eq!(created.harness_template, "standard");
    }
}

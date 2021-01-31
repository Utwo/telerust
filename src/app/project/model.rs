use chrono;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use serde::Serialize;
use slug::slugify;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

// this struct will be used to represent database record
#[derive(Serialize, FromRow)]
pub struct Project {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    #[allow(non_snake_case)]
    pub createdAt: chrono::DateTime<chrono::Utc>,
    #[allow(non_snake_case)]
    pub updatedAt: chrono::DateTime<chrono::Utc>,
}

// Implementation for Project struct, functions for read/write/update and delete from database
impl Project {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Project>, sqlx::Error> {
        let projects = sqlx::query_as!(
            Project,
            r#"SELECT id, name, slug, "createdAt", "updatedAt" FROM projects;"#,
        )
        .fetch_all(pool)
        .await?;

        Ok(projects)
    }

    pub async fn add(pool: &PgPool, name: &String) -> Result<Project, sqlx::Error> {
        let mut slug_count: i64 = 1;
        let mut slug = "".to_string();
        while slug_count != 0 {
            slug = format!("{}-{}", slugify(&name), generate_random_string(5));
            let row = sqlx::query_as::<_, (i64,)>(
                "SELECT COUNT(*) as count FROM projects WHERE slug = $1;",
            )
            .bind(&slug)
            .fetch_one(pool)
            .await?;
            slug_count = row.0;
        }

        let project = sqlx::query_as::<_, Project>(
            "INSERT INTO projects (name, slug) VALUES ($1 , $2) RETURNING *;",
        )
        .bind(&name)
        .bind(&slug)
        .fetch_one(pool)
        .await?;

        Ok(project)
    }
}

fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}

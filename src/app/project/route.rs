use super::model::Project;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct ProjectRequest {
    pub name: String,
}

#[get("/all")]
pub async fn find_all(db_pool: State<'_, PgPool>) -> Json<Vec<Project>> {
    let result = Project::find_all(&db_pool.inner()).await;
    match result {
        Ok(projects) => Json(projects),
        _ => panic!("oohhh"),
    }
}

#[post("/", data = "<body>")]
pub async fn add(db_pool: State<'_, PgPool>, body: Json<ProjectRequest>) -> Json<Project> {
    let result = Project::add(&db_pool.inner(), &body.name).await;
    match result {
        Ok(project) => Json(project),
        Err(error) => panic!(error),
    }
}

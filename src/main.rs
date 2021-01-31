#[macro_use]
extern crate rocket;

mod app;

use app::project;
use dotenv::dotenv;
use rocket_cors::CorsOptions;

use sqlx::postgres::PgPoolOptions;
use std::env;

#[launch]
async fn rocket() -> rocket::Rocket {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    let cors = CorsOptions::default().to_cors().unwrap();
    rocket::ignite().manage(db_pool.clone()).attach(cors).mount(
        "/project",
        routes![project::route::find_all, project::route::add],
    )
}

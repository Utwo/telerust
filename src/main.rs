#[macro_use]
extern crate rocket;
mod app;
mod middlewares;

use app::account;
use app::project;
use dotenv::dotenv;
use firestore_db_and_auth::Credentials;
use rocket_cors::CorsOptions;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[launch]
#[tokio::main]
async fn rocket() -> rocket::Rocket {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();
    let cors = CorsOptions::default().to_cors().unwrap();
    // let credentials =
    //     Credentials::from_file("/home/utwo/.config/gcloud/application_default_credentials.json")
    //         .expect("Read credentials file");
    rocket::ignite()
        .manage(db_pool.clone())
        // .manage(credentials)
        .attach(cors)
        .mount(
            "/project",
            routes![project::route::find_all, project::route::add],
        )
        .mount(
            "/account",
            routes![account::route::get_authenticated, account::route::update],
        )
}

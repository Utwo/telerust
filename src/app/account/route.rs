use super::model::Account;
use crate::middlewares::auth::AuthUser;
use rocket::State;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::PgPool;

#[derive(Serialize, Deserialize)]
pub struct AccountUpdateRequest {
    pub meta: Value,
}

#[get("/")]
pub async fn get_authenticated(db_pool: State<'_, PgPool>, auth_user: AuthUser) -> String {
    //println!("{:?}", auth_user.0.user_id);
    // let result = Account::get_authenticated(&db_pool.inner()).await;
    "helllo".to_string()
    // match result {
    //     Ok(account) => Json({}),
    //     Err(error) => panic!(error),
    // }
}

#[put("/", data = "<body>")]
pub async fn update(db_pool: State<'_, PgPool>, body: Json<AccountUpdateRequest>) -> Json<Account> {
    unimplemented!("unimplemented update account");
}

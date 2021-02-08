// use firestore_db_and_auth::credentials::Credentials;
// use firestore_db_and_auth::errors::FirebaseError;
// use firestore_db_and_auth::{sessions, UserSession};
// use rocket::request::{FromRequest, Outcome, Request};
// use rocket::{http::Status, State};

// pub struct AuthUser(pub sessions::user::Session);

// #[rocket::async_trait]
// impl<'a, 'r> FromRequest<'a, 'r> for AuthUser {
//     type Error = FirebaseError;
//     async fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
//         let r = request
//             .headers()
//             .get_one("Authorization")
//             .unwrap_or_default()
//             .split_whitespace()
//             .last();

//         let bearer = match r {
//             Some(re) => re,
//             _ => return Outcome::Forward(()),
//         };

//         // You MUST make the credentials object available as managed state to rocket!
//         let db = match request.guard::<State<Credentials>>().await {
//             Outcome::Success(db) => db,
//             _ => {
//                 return Outcome::Failure((
//                     Status::InternalServerError,
//                     FirebaseError::Generic("Firestore credentials not set!"),
//                 ))
//             }
//         };

//         let session = sessions::user::Session::by_access_token(&db, bearer);
//         if session.is_err() {
//             return Outcome::Forward(());
//         }
//         Outcome::Success(AuthUser(session.unwrap()))
//     }
// }

use crate::app::account::model::FirebaseIdentities;
use jwks_client::error::Error;
use jwks_client::keyset::KeyStore;
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome, Request};
use serde::Deserialize;
use sqlx::types::Json;

#[derive(Deserialize)]
pub struct AuthUser {
    pub user_id: String,
    pub name: String,
    pub email: String,
    pub picture: String,
    pub firebase: Json<FirebaseIdentities>,
}

#[derive(Debug)]
pub enum AuthUserError {
    Invalid,
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'a, 'r> for AuthUser {
    type Error = AuthUserError;

    async fn from_request(req: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let jkws_url =
            "https://www.googleapis.com/service_accounts/v1/jwk/securetoken@system.gserviceaccount.com";
        let key_set = KeyStore::new_from(jkws_url).await.unwrap();
        let token = req
            .headers()
            .get_one("Authorization")
            .unwrap_or_default()
            .split_whitespace()
            .last();
        if (token.is_none()) {
            return Outcome::Failure((Status::BadRequest, AuthUserError::Invalid));
        }
        match key_set.verify(token.unwrap()) {
            Ok(jwt) => {
                let claims = jwt.payload().into::<AuthUser>().unwrap();
                Outcome::Success(claims)
            }
            Err(Error { msg, typ: _ }) => {
                Outcome::Failure((Status::BadRequest, AuthUserError::Invalid))
            }
        }
    }
}

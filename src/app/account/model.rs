use jwks_client::error::Error;
use jwks_client::keyset::KeyStore;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{types::chrono, types::Json, types::Uuid, FromRow, PgPool};

#[derive(Serialize, Deserialize)]
pub struct Identities {
    google_com: Vec<String>,
    github_com: Vec<String>,
    email: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FirebaseIdentities {
    identities: Json<Identities>,
}

// this struct will be used to represent database record
#[derive(Serialize, FromRow)]
pub struct Account {
    pub id: Uuid,
    pub authId: String,
    pub name: String,
    pub email: String,
    pub picture: String,
    pub identities: Json<FirebaseIdentities>,
    pub meta: Value,
    #[allow(non_snake_case)]
    pub createdAt: chrono::DateTime<chrono::Utc>,
    #[allow(non_snake_case)]
    pub updatedAt: chrono::DateTime<chrono::Utc>,
}

impl Account {
    pub async fn get_authenticated(pool: &PgPool) {
        let jkws_url =
            "https://raw.githubusercontent.com/jfbilodeau/jwks-client/0.1.8/test/test-jwks.json";

        let key_set = KeyStore::new_from(jkws_url).await.unwrap();

        let token = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IjEifQ.eyJuYW1lIjoiQWRhIExvdmVsYWNlIiwiaXNzIjoiaHR0cHM6Ly9jaHJvbm9nZWFycy5jb20vdGVzdCIsImF1ZCI6InRlc3QiLCJhdXRoX3RpbWUiOjEwMCwidXNlcl9pZCI6InVpZDEyMyIsInN1YiI6InNidTEyMyIsImlhdCI6MjAwLCJleHAiOjUwMCwibmJmIjozMDAsImVtYWlsIjoiYWxvdmVsYWNlQGNocm9ub2dlYXJzLmNvbSJ9.eTQnwXrri_uY55fS4IygseBzzbosDM1hP153EZXzNlLH5s29kdlGt2mL_KIjYmQa8hmptt9RwKJHBtw6l4KFHvIcuif86Ix-iI2fCpqNnKyGZfgERV51NXk1THkgWj0GQB6X5cvOoFIdHa9XvgPl_rVmzXSUYDgkhd2t01FOjQeeT6OL2d9KdlQHJqAsvvKVc3wnaYYoSqv2z0IluvK93Tk1dUBU2yWXH34nX3GAVGvIoFoNRiiFfZwFlnz78G0b2fQV7B5g5F8XlNRdD1xmVZXU8X2-xh9LqRpnEakdhecciFHg0u6AyC4c00rlo_HBb69wlXajQ3R4y26Kpxn7HA";
        match key_set.verify(token) {
            Ok(jwt) => {
                println!("name={}", jwt.payload().get_str("name").unwrap());
            }
            Err(Error { msg, typ: _ }) => {
                eprintln!("Could not verify token. Reason: {}", msg);
            }
        }
    }
}

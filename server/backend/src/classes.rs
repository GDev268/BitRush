use actix_web::web;
use bcrypt::{hash, verify, DEFAULT_COST};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

use rand::Rng;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JsonUser {
    pub username: String,
    pub password: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub user_id: u64,
    pub username: String,
    pub password: String,
    pub email: String,
    pub server_id: Option<u32>,
}

impl User {
    pub fn create_user(user: JsonUser, user_id:u64) -> Self {
        let encrypted_password = hash(user.password, DEFAULT_COST).unwrap();

        Self {
            user_id,
            username: user.username,
            password: encrypted_password,
            email: user.email,
            server_id: None,
        }
    }
}

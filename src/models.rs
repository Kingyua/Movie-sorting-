use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::FromRow;

#[derive(serde::Deserialize,Serialize,FromRow)]  
pub struct Movie {
    pub id: Option<uuid::Uuid>,
    pub title: String,
    pub genre: Option<String>,
    pub release_year: Option<i32>,
    pub rating: Option<BigDecimal>,
    pub watched: Option<bool>,
}
    
#[derive(Serialize, Deserialize,FromRow)]
    pub struct User {
        pub id: Option<Uuid>,       // User ID
        pub username: String,       // Username
        pub password_hash: String,  // Hashed password
    }
    

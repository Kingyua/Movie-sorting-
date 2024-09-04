use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct Movie {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Uuid>, // Make ID optional
    pub title: String,
    pub watched: bool,
}

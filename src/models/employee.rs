use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct Employee {
    pub id: i64,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}
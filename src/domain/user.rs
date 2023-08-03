use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Debug,Serialize)]
pub struct User {
    //pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

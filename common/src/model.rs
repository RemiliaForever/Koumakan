use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(FromRow))]
pub struct Comment {
    pub id: Option<i64>,
    pub article_id: i64,
    pub name: String,
    pub email: String,
    pub website: String,
    pub content: String,
    pub avatar: String,
    pub date: chrono::NaiveDateTime,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", derive(FromRow))]
pub struct Article {
    pub id: Option<i64>,
    pub title: String,
    pub brief: String,
    pub content: String,
    pub category: String,
    pub labels: String,
    pub date: chrono::NaiveDateTime,
}

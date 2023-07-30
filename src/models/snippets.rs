use sqlx::PgPool;
use crate::domain::Snippet;

/*
use chrono::{DateTime, Utc};
use std::fmt::Debug;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Snippet {
    id: i64,
    title: String,
    content: String,
    created_at: DateTime<Utc>,
    expired_at: DateTime<Utc>,
}
*/

pub struct SnippetModel<'a> {
    pub db: &'a PgPool,
}

impl SnippetModel<'_> {
    pub async fn insert(&self, title: impl AsRef<str>, content: impl AsRef<str>, expires: i32) -> Result<i64, sqlx::Error> {
        let  rec  =  sqlx::query!(
            r#"
               insert into snippets (title, content, expired_at) 
               values($1, $2, now() + make_interval(days => $3)) returning  id
            "#,
            title.as_ref(),
            content.as_ref(),
            expires,
            )
            .fetch_one(self.db)
            .await?;
        Ok(rec.id)
    }

    pub async fn remove(&self, id: i64) -> Result<u64, sqlx::Error>  {
         let remove_count = sqlx::query!(
             r#"
               delete from  snippets where  id = $1
            "#,
            id,
          )
          .execute(self.db)
          .await?
          .rows_affected();

          Ok(remove_count)
    }

    pub async fn get(&self, id: i64) -> Result<Option<Snippet>, sqlx::Error> {
        let row = sqlx::query_as!(
            Snippet,
            r#"
              select id, title, content, created_at, expired_at from snippets where expired_at > now() and id = $1;
            "#,
            id,
        )
        .fetch_optional(self.db)
        .await?;
        Ok(row)
    }

    pub async fn latest(&self) -> Result<Vec<Snippet>, sqlx::Error> {
        let rows = sqlx::query_as!(
            Snippet,
            r#"
                select id, title, content, created_at, expired_at from snippets order by id desc limit 10
            "#,
        )
        .fetch_all(self.db)
        .await?;
        Ok(rows)
    }
}

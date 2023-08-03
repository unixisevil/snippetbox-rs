use anyhow::Context;
use secrecy::{ExposeSecret, Secret};
use sqlx::PgPool;
use std::fmt::Debug;
use crate::domain::User;

#[derive(thiserror::Error, Debug)]
pub enum RegUserError {
    #[error("Email address is already in use")]
    DuplicateEmail,
    #[error(transparent)]
    UnexpectedError(#[from] sqlx::Error),
}

#[derive(Copy, Clone)]
pub struct UserModel<'a> {
    pub db: &'a PgPool,
}

impl UserModel<'_> {
    pub async fn exists(&self, user_id: i64) -> Result<bool, anyhow::Error> {
        let row = sqlx::query!(
            r#"
               select exists(select true from users where id = $1) as "exists!"
            "#,
            user_id,
        )
        .fetch_one(self.db)
        .await
        .context("Failed to execute user exist query")?;

        Ok(row.exists)
    }

    pub async fn get(&self, user_id: i64) -> Result<Option<User>, sqlx::Error> {
        let row = sqlx::query_as!(
            User,
            r#"
               select name, email, created_at from users where id = $1
            "#,
            user_id,
        )
        .fetch_optional(self.db)
        .await?;

        Ok(row)
    }
    
    pub async fn get_email(&self, user_id: i64) -> Result<String, anyhow::Error> {
         let row = sqlx::query!(
             r#"
                select email from users  where id = $1
             "#,
             user_id,
         )
        .fetch_one(self.db)
        .await
        .context("Failed to performed a query to retrieve email.")?;
         Ok(row.email)
    }

    //#[tracing::instrument(name = "Get stored credentials", skip(email))]
    pub async fn get_stored_credentials(
        &self,
        email: impl AsRef<str>,
    ) -> Result<Option<(i64, Secret<String>)>, anyhow::Error> {
        let row = sqlx::query!(
            r#"
                select id, password_hash
                from users
                where email = $1
            "#,
            email.as_ref(),
        )
        .fetch_optional(self.db)
        .await
        .context("Failed to performed a query to retrieve stored credentials.")?
        .map(|row| (row.id, Secret::new(row.password_hash)));
        Ok(row)
    }

    pub async fn update_password_hash(
        &self,
        user_id: i64,
        password_hash: Secret<String>,
    ) -> Result<(), anyhow::Error> {
        sqlx::query!(
            r#"
                update users set password_hash = $1 where id = $2
            "#,
            password_hash.expose_secret(),
            user_id
        )
        .execute(self.db)
        .await
        .context("Failed to update password_hash.")?;

        Ok(())
    }

    pub async fn add_user(
        &self,
        name: impl AsRef<str>,
        email: impl AsRef<str>,
        password_hash: Secret<String>,
    ) -> Result<(), RegUserError> {
        sqlx::query!(
            r#"
               insert into users (name, email, password_hash)
               values($1, $2, $3)
            "#,
            name.as_ref(),
            email.as_ref(),
            password_hash.expose_secret(),
        )
        .execute(self.db)
        .await
        .map_err(|e| {
            if let Some(de) = e.as_database_error() {
                if de.is_unique_violation() {
                    RegUserError::DuplicateEmail
                } else {
                    RegUserError::UnexpectedError(e)
                }
            } else {
                RegUserError::UnexpectedError(e)
            }
        })?;
        //.context("Failed to add new user.")?;

        Ok(())
    }
}

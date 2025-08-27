use async_trait::async_trait;

#[derive(Clone)]
pub struct Database {
    pool: PgPool,
}

impl Database {
    pub async fn new_from_url(url: &str) -> Self {
        let connection = PgPoolOptions::new()
            .max_connections(5)
            .connect(&url)
            .await
            .expect("Failed to connect to database 😢");

        return Self { pool: connection };
    }
}

#[async_trait]
impl IUserDB for Database {
    async fn get_users(&self) -> anyhow::Result<Vec<User>> {
        let users = sqlx::query_as("SELECT * FROM users")
            .fetch_all(&self.pool)
            .await?;

        Ok(users)
    }

    async fn get_user(&self, id: uuid::Uuid) -> anyhow::Result<User> {
        let user = sqlx::query_as("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await?;

        return Ok(user);
    }

    async fn create_user(&self, input: UserInput) -> anyhow::Result<User> {
        let created_user =
            sqlx::query_as("INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *")
                .bind(input.name)
                .bind(input.email)
                .fetch_one(&self.pool)
                .await?;

        return Ok(created_user);
    }

    async fn edit_user(&self, id: uuid::Uuid, update: UserInput) -> anyhow::Result<User> {
        let updated_user =
            sqlx::query_as("UPDATE users SET name = $1 , email = $2 WHERE id = $3 RETURNING *")
                .bind(update.name)
                .bind(update.email)
                .bind(id)
                .fetch_one(&self.pool)
                .await?;

        return Ok(updated_user);
    }

    async fn delete_user(&self, id: uuid::Uuid) -> anyhow::Result<()> {
        let result = sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(sqlx::Error::RowNotFound.into());
        }
        return Ok(());
    }
}

pub async fn connect_db() -> PgPool {
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL should be there in your env.. 👀");

    return PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database 😢");
}

use crate::user::model::*;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;

use crate::user::model::{IUserDB, User, UserInput};

use async_trait::async_trait;

#[derive(Serialize, Deserialize, FromRow, Debug, Clone)]
pub struct User {
    id: Uuid,
    name: String,
    email: String,
}

impl User {
    pub fn new(id: Uuid, name: String, email: String) -> Self {
        return Self {
            id: id,
            name: name,
            email: email,
        };
    }
}

#[derive(Deserialize)]
pub struct UserInput {
    pub name: String,
    pub email: String,
}

// interface for databases
#[async_trait]
pub trait IUserDB: Send + Sync + Clone + 'static {
    async fn get_users(&self) -> anyhow::Result<Vec<User>>;
    async fn create_user(&self, input: UserInput) -> anyhow::Result<User>;
    async fn get_user(&self, id: Uuid) -> anyhow::Result<User>;
    async fn edit_user(&self, id: Uuid, update: UserInput) -> anyhow::Result<User>;
    async fn delete_user(&self, id: Uuid) -> anyhow::Result<()>;
}

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

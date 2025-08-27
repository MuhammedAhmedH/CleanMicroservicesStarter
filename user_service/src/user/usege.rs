pub struct UserUsage<T: IUserDB> {
    database: T,
}

impl<T: IUserDB> UserUsage<T> {
    pub fn new(db: T) -> Self {
        return Self { database: db };
    }

    pub async fn get_users(&self) -> anyhow::Result<Vec<User>> {
        return self.database.get_users().await;
    }

    pub async fn create_user(&self, input: UserInput) -> anyhow::Result<User> {
        return self.database.create_user(input).await;
    }

    pub async fn get_user(&self, id: Uuid) -> anyhow::Result<User> {
        return self.database.get_user(id).await;
    }

    pub async fn edit_user(&self, id: Uuid, updated_info: UserInput) -> anyhow::Result<User> {
        return self.database.edit_user(id, updated_info).await;
    }

    pub async fn delete_user(&self, id: Uuid) -> anyhow::Result<()> {
        return self.database.delete_user(id).await;
    }
}

use crate::user::model::{IUserDB, User, UserInput};
use uuid::Uuid;

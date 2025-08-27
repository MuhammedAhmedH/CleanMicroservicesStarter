use std::env;

use crate::{
    databases,
    server::{configs::Configs, routering::MainRouter},
    user::{handlers::UserHandler, usege::UserUsage},
};

pub struct Server {
    configs: Configs,
}

impl Server {
    pub fn new(configs: Configs) -> Self {
        return Self { configs };
    }

    pub async fn serve(&self, address: &str) -> anyhow::Result<()> {
        let database_url = env::var(self.configs.database_url_env).expect(
            " 😢 check the server configurations and make sure to config your env varibales.. 👀",
        );

        let postgres_db = databases::postgresql::Database::new_from_url(&database_url).await;

        let user_apps = UserUsage::new(postgres_db);

        let user_handlers = UserHandler::new(user_apps);

        let app = MainRouter::new().add(user_handlers).build_router();

        let listener = tokio::net::TcpListener::bind(address).await?;

        println!("I'M Running As A Beast 😎🔥");

        axum::serve(listener, app).await?;

        return Ok(());
    }
}

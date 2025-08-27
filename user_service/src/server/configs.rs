use dotenvy::dotenv;

pub struct Configs {
    pub database_url_env: &'static str,
}

impl Configs {
    pub fn init() -> Self {
        dotenv().ok();

        return Self {
            database_url_env: "DATABASE_URL",
        };
    }
}

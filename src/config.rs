use dotenv::dotenv;
use std::env;

pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub dbname: String,
}

impl DbConfig {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            host: env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            port: env::var("DB_PORT").unwrap_or_else(|_| "5432".to_string()).parse().unwrap_or(5432),
            user: env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string()),
            password: env::var("DB_PASSWORD").unwrap_or_else(|_| "secret".to_string()),
            dbname: env::var("DB_NAME").unwrap_or_else(|_| "perp_finder".to_string()),
        }
    }
}

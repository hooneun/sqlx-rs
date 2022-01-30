use std::env;

#[derive(Debug)]
pub struct DB {
    pub connection: String,
    pub host: String,
    pub port: String,
    pub database: String,
    pub name: String,
    pub password: String,
}

impl DB {
    pub fn url() -> String {
        let connection = env::var("DB_CONNECTION").expect("DB_CONNECTION Not Found");
        let host = env::var("DB_HOST").expect("DB_HOST Not Found");
        let port = env::var("DB_PORT").expect("DB_PORT Not Found");
        let database = env::var("DB_DATABASE").expect("DB_DATABASE Not Found");
        let name = env::var("DB_USERNAME").expect("DB_USERNAME Not Found");
        let password = env::var("DB_PASSWORD").expect("DB_PASSWORD Not Found");

        format!("{connection}://{name}:{password}@{host}:{port}/{database}")
    }
}

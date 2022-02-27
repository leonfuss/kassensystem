use dotenv::dotenv;
use std::env;

pub struct Config {
    database_url: String,
}

impl Config {
    pub fn load() -> Config {
        dotenv().expect(".env file seems to be corrupted or missing");

        let database_url = match env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(e) => panic!("DATABASE_URL does not exist:\n {}", e),
        };

        Config { database_url }
    }
}

use std::net::TcpListener;

#[derive(serde::Deserialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}

#[derive(serde::Deserialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}

impl Settings {
    pub fn new() -> Result<Self, config::ConfigError> {
        // Creates new Settings struct from Configuration top-level file named
        // "configuration" every format that is supported by rs-config will
        // parse
        config::Config::builder()
            .add_source(config::File::with_name("configuration"))
            .build()?
            .try_deserialize()
    }

    pub fn tcp_listener(&self) -> Result<TcpListener, std::io::Error> {
        let address = format!("127.0.0.1:{}", self.application_port);
        std::net::TcpListener::bind(address)
    }
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn connection_string_without_db(&self) -> String {
        format!("postgres://{}:{}@{}:{}", self.username, self.password, self.host, self.port)
    }
}

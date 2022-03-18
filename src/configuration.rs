#[derive(serde::Deserialize)]
pub struct Settings {
    pub databse: DatabaseSettings,
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
        // Creates new Settings struct from Configuration top-level file named "configuration" 
        // every format that is supported by rs-config will parse
        config::Config::builder()
            .add_source(config::File::with_name("configuration"))
            .build()?
            .try_deserialize()
    }
}

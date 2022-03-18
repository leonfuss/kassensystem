use ausgleichende_gerechtigkeit::{configuration::Settings, startup};

use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let configuration = Settings::new().expect("Failed to load configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    let listener = configuration.tcp_listener()?;

    startup::run(listener, connection_pool)?.await
}

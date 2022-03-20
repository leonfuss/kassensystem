use std::net::TcpListener;

use ausgleichende_gerechtigkeit::{
    configuration::Settings,
    startup,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::postgres::PgPoolOptions;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("a_g".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // Application settings and startup
    let configuration = Settings::new().expect("Failed to load configuration");
    let connection_pool = PgPoolOptions::new()
        .connect_timeout(std::time::Duration::from_secs(5))
        .connect_lazy_with(configuration.database.with_db());

    let listener =
        TcpListener::bind(configuration.application.address()).expect("Failed to bind address");

    startup::run(listener, connection_pool)?.await
}

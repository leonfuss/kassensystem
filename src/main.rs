use ausgleichende_gerechtigkeit::{
    configuration::Settings,
    startup,
    telemetry::{get_subscriber, init_subscriber},
};
use secrecy::ExposeSecret;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let subscriber = get_subscriber("a_g".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    // Application settings and startup
    let configuration = Settings::new().expect("Failed to load configuration");
    let connection_pool =
        PgPool::connect(configuration.database.connection_string().expose_secret())
            .await
            .expect("Failed to connect to postgres");
    let listener = configuration.tcp_listener()?;

    startup::run(listener, connection_pool)?.await
}

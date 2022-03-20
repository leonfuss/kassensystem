use ausgleichende_gerechtigkeit::{configuration::Settings, startup};
use sqlx::PgPool;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_log::LogTracer;
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Redirect all log events to subscriber
    LogTracer::init().expect("Failed to set logger");

    // Initialize logger for whole project
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("a_g".into(), std::io::stdout);

    let subscriber =
        Registry::default().with(env_filter).with(JsonStorageLayer).with(formatting_layer);
    set_global_default(subscriber).expect("Failed to st subscriber");

    let configuration = Settings::new().expect("Failed to load configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Failed to connect to postgres");
    let listener = configuration.tcp_listener()?;

    startup::run(listener, connection_pool)?.await
}

use ausgleichende_gerechtigkeit::{
    configuration::{DatabaseSettings, Settings},
    telemetry::{get_subscriber, init_subscriber},
};
use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Executor, PgPool};

static TRACING: Lazy<()> = Lazy::new(|| {
    let default_filter_level = "info".to_string();
    let subscriber_name = "test".to_string();

    if std::env::var("TEST_LOG").is_ok() {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::stdout);
        init_subscriber(subscriber);
    } else {
        let subscriber = get_subscriber(subscriber_name, default_filter_level, std::io::sink);
        init_subscriber(subscriber);
    }
});

#[derive(Debug)]
pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

#[tokio::test]
async fn health_check_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", &app.address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn create_user_returns_a_200_for_vaild_form_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "matrikelnummer=6083015&email=max%40student.uni-tuebingen.de&name=Max%20Muster";
    let response = client
        .post(&format!("{}/user/create", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name, matrikelnummer FROM users")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved user");

    assert_eq!(saved.email, "max@student.uni-tuebingen.de");
    assert_eq!(saved.name, "Max Muster");
    assert_eq!(saved.matrikelnummer, 6083015);
}

#[tokio::test]
async fn create_user_returns_a_400_when_data_is_missing() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let test_cases = vec![
        (
            "email=max%40student.uni-tuebingen.de&name=max%20muster",
            "missing matrikelnummer",
        ),
        ("matrikelnummer=6083015&name=max%20muster", "missing email"),
        (
            "matrikelnummer=6083015&email=max%40student.uni-tuebingen.de",
            "missing name",
        ),
        (
            "email=max%40student.uni-tuebingen.de",
            "missing name and matrikelnummer",
        ),
        ("matrikelnummer=6083015", "missing name and email"),
        ("name=max%20muster", "missing email and matrikelnummer"),
        ("", "missing all"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/user/create", &app.address))
            .header("content-type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("failed to execute request");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        );
    }
}

async fn spawn_app() -> TestApp {
    // Init test tracing only on first test run
    Lazy::force(&TRACING);

    // port '0' provides a random free port
    // required for spawning multiple test instances without test collision
    let listener =
        std::net::TcpListener::bind("127.0.0.1:0").expect("Failed to bind to random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    // random database name for each test to isolate tests
    let mut configuration = Settings::new().expect("Failed to read configuration");
    configuration.database.database_name = uuid::Uuid::new_v4().to_string();

    let connection_pool = configure_database(&configuration.database).await;

    let server = ausgleichende_gerechtigkeit::startup::run(listener, connection_pool.clone())
        .expect("Failed to bind address");

    let _ = tokio::spawn(server);

    TestApp {
        address,
        db_pool: connection_pool,
    }
}

async fn configure_database(config: &DatabaseSettings) -> PgPool {
    let connection = PgPool::connect(&config.connection_string_without_db().expose_secret())
        .await
        .expect("Failed to connect to postgres");
    connection
        .execute(format!(r#"CREATE DATABASE "{}"; "#, config.database_name).as_str())
        .await
        .expect("Failed to create database");

    // Migrate Database
    let connection_pool = PgPool::connect(&config.connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate database");

    connection_pool
}

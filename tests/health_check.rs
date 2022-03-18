#[tokio::test]
async fn health_check_works() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health_check", app_address))
        .send()
        .await
        .expect("Failed to execute request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

#[tokio::test]
async fn create_user_returns_a_200_for_vaild_form_data() {
    let app_address = spawn_app();
    let client = reqwest::Client::new();

    let body = "matrikelnummer=6083015&email=max%40student.uni-tuebingen.de&name=Max%20Muster";
    let response = client
        .post(&format!("{}/user/create", app_address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn create_user_returns_a_400_when_data_is_missing() {
    let app_address = spawn_app();
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
            .post(&format!("{}/user/create", app_address))
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

fn spawn_app() -> String {
    let listener =
        std::net::TcpListener::bind("127.0.0.1:0").expect("failed to bind to random port");

    let port = listener.local_addr().unwrap().port();
    let server = ausgleichende_gerechtigkeit::startup::run(listener).expect("Failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

use actix_web::{web, App, HttpServer};

async fn greet() -> String {
    "Hello there".to_owned()
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(greet))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

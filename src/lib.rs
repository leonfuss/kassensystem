use actix_web::{dev::Server, web, App, HttpServer, HttpResponse, HttpRequest};

async fn health_check(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().finish()
}

pub fn run(address: &str) -> Result<Server, std::io::Error>{
    let server = HttpServer::new(|| { App::new() .route("/health_check", web::get().to(health_check)) })
    .bind(address)?
    .run();

    Ok(server)
}

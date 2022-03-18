use actix_web::{dev::Server, web, App, HttpServer};
use std::net::TcpListener;

use crate::routes;


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| {
        App::new()
            .route("/health_check", web::get().to(routes::health_check))
            .service(web::scope("/user").service(routes::create_user))
    })
    .listen(listener)?
    .run();

    Ok(server)
}

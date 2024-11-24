use actix_web::{web, App, HttpServer, HttpResponse, Responder, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default()) // Log requests
            .wrap(
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .route("/", web::get().to(|| async { HttpResponse::Ok().body("Hello, world!") }))
            .route("/api/greet", web::get().to(|| async {
                HttpResponse::Ok().body("Hello from the backend!")
            }))
    })
    .bind("0.0.0.0:4100")?
    .run()
    .await
}

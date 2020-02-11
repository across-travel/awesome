use kayrx::web::middleware::{Cors, Logger};
use kayrx::http::header;
use kayrx::web::{web, App, HttpServer};

mod user;

#[kayrx::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "kayrx::web=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .wrap(Logger::default())
            .service(web::resource("/user/info").route(web::post().to(user::info)))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

use kayrx::web::{middleware, web, App, HttpRequest, HttpServer};

async fn index(_req: HttpRequest) -> &'static str {
    "Hello world!"
}

#[kayrx::main]
async fn main() -> std::io::Result<()> {
    ::std::env::set_var("RUST_LOG", "kayrx_server=info,kayrx_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger - always register kayrx-web Logger middleware last
            .wrap(middleware::Logger::default())
            .service(
                web::resource("/index.html")
                    .route(web::get().to(|| async { "Hello world!" })),
            )
            .service(web::resource("/").to(index))
    })
    .bind_uds("/tmp/kayrx-uds.socket")?
    .run()
    .await
}
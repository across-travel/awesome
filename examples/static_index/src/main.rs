use kayrx::web::{middleware, App, HttpServer, file as fs};

#[kayrx::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "kayrx::web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(
                // static files
                fs::Files::new("/", "./static/").index_file("index.html"),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

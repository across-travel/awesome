use kayrx::web::{middleware, App, HttpServer};

use async_ex::appconfig::config_app;

#[kayrx::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "kayrx::server=info,kayrx::web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .configure(config_app)
            .wrap(middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

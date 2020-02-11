#[macro_use]
extern crate kayrx;

use kayrx::web::{middleware, web, App, HttpServer, HttpRequest, Responder};

async fn index(req: HttpRequest) -> &'static str {
    println!("REQ: {:?}", req);
    "Hello world!"
}

#[get("/{id}/{name}/index.html")]
async fn hello(info: web::Path<(u32, String)>) -> impl Responder {
      format!("Hello {}! id:{}", info.1, info.0)
}

#[kayrx::main]
async fn main() -> std::io::Result<()> {
        std::env::set_var("RUST_LOG", "kayrx::web=info");
        env_logger::init();

        HttpServer::new(|| 
            App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| async { "Hello world!" }))
            .service(web::resource("/").to(index))
            .service(hello))
            .bind("127.0.0.1:8080")?
            .run()
            .await
}

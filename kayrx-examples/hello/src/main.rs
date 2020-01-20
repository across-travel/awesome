#[macro_use]
extern crate kayrx;

use kayrx::web::{web, App, HttpServer, Responder};

#[get("/{id}/{name}/index.html")]
async fn index(info: web::Path<(u32, String)>) -> impl Responder {
      format!("Hello {}! id:{}", info.1, info.0)
}

#[kayrx::main]
async fn main() -> std::io::Result<()> {
      HttpServer::new(|| App::new().service(index))
          .bind("127.0.0.1:8080")?
          .run()
          .await
}
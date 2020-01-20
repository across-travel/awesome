#[macro_use]	
extern crate kayrx;	

use std::{env, io};	
use keclc_file as fs;	
use kayrx::util::mpsc;	
use kayrx::http::{header, Method, StatusCode};	
use kayrx::web::{	
    error, guard, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer,	
    Result,	
};	
use bytes::Bytes;


#[get("/favicon")]
async fn favicon() -> Result<fs::NamedFile> {	      
    Ok(fs::NamedFile::open("static/favicon.ico")?)	
}	

/// simple index handler	
#[get("/welcome")]	
async fn welcome(req: HttpRequest) -> Result<HttpResponse> {	
    println!("{:?}", req);	

    // response	
    Ok(HttpResponse::build(StatusCode::OK)	
        .content_type("text/html; charset=utf-8")	
        .body(include_str!("../static/welcome.html")))	
}	

/// response body	
async fn response_body(path: web::Path<String>) -> HttpResponse {	
    let text = format!("Hello {}!", *path);	

    let (tx, rx_body) = mpsc::channel();	
    let _ = tx.send(Ok::<_, Error>(Bytes::from(text)));	

    HttpResponse::Ok().streaming(rx_body)	
}	

/// handler with path parameters like `/user/{name}/`	
async fn with_param(req: HttpRequest, path: web::Path<(String,)>) -> HttpResponse {	
    println!("{:?}", req);	

    HttpResponse::Ok()	
        .content_type("text/plain")	
        .body(format!("Hello {}!", path.0))	
}	

/// 404 handler	
async fn p404() -> Result<fs::NamedFile> {	
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(StatusCode::NOT_FOUND))	
}

#[kayrx::main]
async fn main() -> io::Result<()> {	
    env::set_var("RUST_LOG", "eternal=debug,eternal=info");	      
    env_logger::init();

    HttpServer::new(|| {	         
        App::new()	
            // enable logger - always register eternal Logger middleware last	
            .wrap(middleware::Logger::default())	
            // register favicon	
            .service(favicon)	
            // register simple route, handle all methods	
            .service(welcome)	
            // with path parameters	
            .service(web::resource("/user/{name}").route(web::get().to(with_param)))	
            // async response body	
            .service(	
                web::resource("/async/{name}").route(web::get().to(response_body)),	
            )	
            .service(	
                web::resource("/test").to(|req: HttpRequest| match *req.method() {	
                    Method::GET => HttpResponse::Ok(),	
                    Method::POST => HttpResponse::MethodNotAllowed(),	
                    _ => HttpResponse::NotFound(),	
                }),	
            )	
            .service(web::resource("/error").to(|| {	
                async {	
                    error::InternalError::new(	
                        io::Error::new(io::ErrorKind::Other, "test"),	
                        StatusCode::INTERNAL_SERVER_ERROR,	
                    )	
                }	
            }))	
            // static files	
            .service(fs::Files::new("/static", "static").show_files_listing())	
            // redirect	
            .service(web::resource("/").route(web::get().to(|req: HttpRequest| {	
                println!("{:?}", req);	
                HttpResponse::Found()	
                    .header(header::LOCATION, "static/welcome.html")	
                    .finish()	
            })))	
            // default	
            .default_service(	
                // 404 for GET request	
                web::resource("")	
                    .route(web::get().to(p404))	
                    // all requests that are not `GET`	
                    .route(	
                        web::route()	
                            .guard(guard::Not(guard::Get()))	
                            .to(HttpResponse::MethodNotAllowed),	
                    ),	
            )	
    })	
    .bind("127.0.0.1:8080")?	
    .run()	
    .await	
} 
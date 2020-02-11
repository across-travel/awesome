#[macro_use]
extern crate kayrx;

use kayrx::web::{error, web, App, HttpResponse, HttpServer};
use config::ConfigError;
use keclc_pgpool::{Client, Pool, PoolError};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
struct Config {
    listen: String,
    pg: keclc_pgpool::Config,
}

impl Config {
    fn from_env() -> Result<Self, ConfigError> {
        let mut cfg = ::config::Config::new();
        cfg.merge(::config::Environment::new())?;
        cfg.try_into()
    }
}

#[derive(Serialize, Deserialize)]
struct Event {
    id: i32,
    title: String,
}

#[derive(failure::Fail, Debug)]
enum Error {
    #[fail(display = "An internal error occured. Please try again later.")]
    PoolError(PoolError),
}

impl From<PoolError> for Error {
    fn from(error: PoolError) -> Self {
        Self::PoolError(error)
    }
}

impl error::ResponseError for Error {}

async fn event_list(pool: &Pool) -> Result<Vec<Event>, PoolError> {
    let client: Client = pool.get().await?;
    let stmt = client.prepare("SELECT id, title FROM event").await?;
    let rows = client.query(&stmt, &[]).await?;
    Ok(rows
        .into_iter()
        .map(|row| Event {
            id: row.get(0),
            title: row.get(1),
        })
        .collect())
}

#[get("/v1/event")]
async fn index(db_pool: web::Data<Pool>) -> Result<HttpResponse, Error> {
    let events = event_list(&db_pool).await?;
    Ok(HttpResponse::Ok().json(events))
}

#[kayrx::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(kpgres::NoTls).unwrap();
    let server = HttpServer::new(move || App::new().data(pool.clone()).service(index))
        .bind(&config.listen)?
        .run();
    println!("Server running at http://{}/", &config.listen);
    println!(
        "Try the following URLs: http://{}/v1/event",
        &config.listen,
    );
    server.await
}
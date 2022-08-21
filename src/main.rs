mod config;
mod models;
mod handlers;
mod errors;
mod db;

use actix_web::web::Data;
use actix_web::{ HttpServer, App};
use deadpool_postgres::{ Runtime };
use deadpool_postgres::tokio_postgres::NoTls;

use dotenv::dotenv;
use slog::{Logger, o, Drain, info};
use slog_term;
use slog_async;

use crate::config::Config;
use crate::models::AppState;

fn configure_log () -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator).build().fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    slog::Logger::root(drain, o!("v" => env!("CARGO_PKG_VERSION")))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    dotenv().ok();

    let config = config::Config::from_env().unwrap();
    let all_settings = config.try_deserialize::<Config>().unwrap();
    let (host, port) = (all_settings.server.host, all_settings.server.port ) ;

    let dead_pool = all_settings.pg.create_pool(Some(Runtime::Tokio1),NoTls).unwrap();

    let log = configure_log();

    info!(log, "Starting server at http://{}:{}/", host, port);

    HttpServer::new( move || {
        App::new()
        .app_data(Data::new(
            AppState {
                db_pool: dead_pool.clone(),
                log: log.clone(),
            }
        ))
        .service(handlers::health)
        .service(handlers::index)
        .service(handlers::show)
        .service(handlers::create)
        .service(handlers::check_item)
    })
    .bind( format!("{}:{}", host, port))?
    .run()
    .await
}

// actix
use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
// diesel
#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

// dotenv
#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;
use dotenv::dotenv;

// modules

mod data_models;
mod database;

mod apis;
mod redis;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv
    dotenv().ok();
    // env_logger
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();
    // actix config
    let actix_host_dev: &str = dotenv!("ACTIX_HOST_DEV");
    let actix_port_dev: u16 = dotenv!("ACTIX_PORT_DEV").to_string().parse().unwrap();
    // creat sqlite connection
    let sqlite_spec = "webscrape.db";
    // r2d2 connection manager
    let manager = ConnectionManager::<SqliteConnection>::new(sqlite_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    // initialize app state
    let app_state = routes::AppState {
        app_name: String::from("Actix Web Scraper Server"),
        app_host: actix_host_dev.to_string(),
        app_port: actix_port_dev,
    };

    // dummy database
    let health_check_db = data_models::health_check_model::HealthCheckState::init();
    let app_data = web::Data::new(health_check_db);

    // create new http actix server with 2 threads
    let app = HttpServer::new(move || {
        // cors config
        let cors = Cors::default()
            .allowed_origin(&format!("http://{}:{}/", actix_host_dev, actix_port_dev))
            .allowed_origin(&format!("http://{}:{}/", actix_host_dev, actix_port_dev))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .app_data(web::Data::new(app_state.clone()))
            .app_data(app_data.clone())
            .configure(apis::config)
            .service(routes::index)
    })
    .workers(2)
    .bind((actix_host_dev, actix_port_dev))?
    .run();

    let res = redis::redis_client::try_redis();
    println!("Redis test: {}", res.unwrap());

    println!(
        "Actix dev server running at http://{}:{}/ ",
        actix_host_dev, actix_port_dev
    );
    app.await
}

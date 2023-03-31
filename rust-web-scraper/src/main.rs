// actix
use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
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
mod api;
mod redis;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // dotenv
    dotenv().ok();
    // actix config
    let actix_host_dev: &str = dotenv!("ACTIX_HOST_DEV");
    let actix_port_dev: u16 = dotenv!("ACTIX_PORT_DEV").to_string().parse().unwrap();
    // creat sqlite connection
    let sqlite_spec = "webscrape.db";
    // r2d2 connection manager
    let manager = ConnectionManager::<SqliteConnection>::new(sqlite_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expectt("Failed to create pool.");

    // initialize app state
    let app_state = routes::AppState {
        app_name: String::from("Actix Web Scraper Server"),
        app_host: actix_host_dev.to_string(),
        app_port: actix_port_dev,
    };

    // create new http actix server with 2 threads
    let app = HttpServer::new(move || {
        // cors config
        let cors = Cors::default()
            .allowed_origin(&format!("http://{}:{}/", actix_host_dev, actix_port_dev))
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(app_state.clone()))
            .configure(api::config)
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

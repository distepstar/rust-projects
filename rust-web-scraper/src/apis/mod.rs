use actix_web::web;

pub mod health_check_api;
pub mod scrape_api;
pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(scrape_api::route_scope())
        .service(health_check_api::route_scope());
    conf.service(scope);
}

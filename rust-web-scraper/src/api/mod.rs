use actix_web::web;

pub mod scrape_api;

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(scrape_api::get_scraped_data);
    conf.service(scope);
}

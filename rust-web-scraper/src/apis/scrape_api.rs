use actix_web::{
    error::ResponseError,
    get,
    http::{header::ContentType, StatusCode},
    post, put, web,
    web::Data,
    web::Json,
    web::Path,
    HttpResponse, Responder,
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct TestingData {
    name: String,
    message: String,
}

#[derive(Serialize, Debug)]
struct SingleResponseTest {
    status: String,
    data: TestingData,
}

#[get("/get_scraped_data/")]
async fn get_scraped_data() -> impl Responder {
    let data = TestingData {
        name: "Louis".to_string(),
        message: "Testing for scrape_api".to_string(),
    };

    let json_response = SingleResponseTest {
        status: "success".to_string(),
        data: data.clone(),
    };

    HttpResponse::Ok().json(json_response)
}

pub fn route_scope() -> actix_web::Scope {
    web::scope("/scrap_api").service(get_scraped_data)
}

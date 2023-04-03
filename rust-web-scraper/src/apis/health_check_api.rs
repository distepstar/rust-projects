use actix_web::{
    get,
    http::{header::ContentType, StatusCode},
    post, web,
    web::Json,
    HttpResponse, Responder,
};

use crate::data_models::health_check_model::{self, HealthCheckState};

#[get("/check_get_request")]
async fn check_get_request() -> impl Responder {
    let data = health_check_model::HealthCheck::new(
        "Louis".to_string(),
        "testing".to_string(),
        "20:20:2020".to_string(),
    );

    println!(
        "name: {}, action: {}, date: {}",
        data.tester, data.message, data.test_date
    );

    let res_json = health_check_model::HealthCheckResponse::new(
        200,
        "response successful".to_string(),
        "/health_check_api/check_get_request".to_string(),
        data.to_owned(),
    );

    HttpResponse::Ok().json(res_json)
}

#[post("/check_post_request")]
async fn check_post_request(
    mut body: web::Json<health_check_model::HealthCheck>,
    data: web::Data<health_check_model::HealthCheckState>,
) -> impl Responder {
    let mut vec = data.health_check_db.lock().unwrap();

    let health_check = vec
        .iter()
        .find(|health_check| health_check.message == body.message);

    if health_check.is_some() {
        let error_res = health_check_model::HealthCheckResponse::new(
            409,
            "Error when health checking: message repeated!".to_string(),
            "/health_check_api/check_get_request".to_string(),
            body.to_owned(),
        );

        return HttpResponse::Conflict().json(error_res);
    }

    let health_check_res = body.to_owned();

    vec.push(body.into_inner());

    let res_json = health_check_model::HealthCheckResponse::new(
        200,
        "response successful".to_string(),
        "/check_get_request".to_string(),
        health_check_res,
    );

    HttpResponse::Ok().json(res_json)
}

pub fn route_scope() -> actix_web::Scope {
    web::scope("/health_check_api")
        .service(check_get_request)
        .service(check_post_request)
}

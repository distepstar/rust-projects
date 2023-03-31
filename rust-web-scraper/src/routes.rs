use actix_web::{get, web};

#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
    pub app_host: String,
    pub app_port: u16,
}

// default starting page
#[get("/")]
pub async fn index(data: web::Data<AppState>) -> String {
    let AppState {
        app_name,
        app_host,
        app_port,
    } = &data.as_ref();
    format!(
        "Dev server {} running on http://{}:{}/",
        app_name, app_host, app_port
    )
}

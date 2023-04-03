use diesel::{
    r2d2::{self, ConnectionManager},
    SqliteConnection,
};

use crate::data_models::health_check_model::HealthCheck;

pub type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;

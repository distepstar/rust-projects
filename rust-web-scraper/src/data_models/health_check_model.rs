use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

pub struct HealthCheckState {
    pub health_check_db: Arc<Mutex<Vec<HealthCheck>>>,
}

impl HealthCheckState {
    pub fn init() -> HealthCheckState {
        HealthCheckState {
            health_check_db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HealthCheck {
    pub tester: String,
    pub message: String,
    pub test_date: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct HealthCheckResponse {
    pub status: i16,
    pub message: String,
    pub endpoint: String,
    pub data: HealthCheck,
}

impl HealthCheck {
    pub fn new(tester: String, message: String, test_date: String) -> Self {
        Self {
            tester,
            message,
            test_date,
        }
    }
}

impl HealthCheckResponse {
    pub fn new(status: i16, message: String, endpoint: String, data: HealthCheck) -> Self {
        Self {
            status,
            message,
            endpoint,
            data,
        }
    }
}

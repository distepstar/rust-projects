pub struct AdminStruct {
    pub username: String,
    pub password: String,
}

impl AdminStruct {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

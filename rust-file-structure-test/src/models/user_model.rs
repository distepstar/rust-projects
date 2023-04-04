pub struct UserStruct {
    username: String,
    password: String,
}

impl UserStruct {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }
}

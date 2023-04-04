use crate::models;

pub fn create_new_admin(username: String, password: String) -> models::admin_model::AdminStruct {
    models::admin_model::AdminStruct::new(username, password)
}

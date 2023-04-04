mod models {
    pub mod admin_model;
    pub mod user_model;
}

mod controller;

fn main() {
    let admin_new = controller::create_new_admin("louislei".to_string(), "mkj200200".to_string());
    println!(
        "admin name: {}, password: {}",
        admin_new.username, admin_new.password
    );
}

use super::databae::{Status, connect_to_db, get_user};

fn authenticate(user: ()) {
    // auth logic
}

pub fn login(cred: models::Credentials) -> bool {
    if let Status::Connected = connect_to_db() {
        let user = get_user();
        authenticate(user);
        // login logic
        return true;
    }
    false
}

pub mod models;
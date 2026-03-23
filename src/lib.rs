#![allow(unused)]

mod databae {
    pub enum Status {
        Connected,
        Interupted,
    }

    pub fn connect_to_db() -> Status {
        Status::Connected
    }

    pub fn get_user() {
        // get user from db
    }
}

pub mod auth {
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

    pub mod models {
        pub struct Credentials {
            pub username: String,
            pub password: String,
        }
    }
}

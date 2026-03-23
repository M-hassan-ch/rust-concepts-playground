#![allow(unused)]
pub struct Credentials{
    pub username: String,
    pub password: String
}

enum Status {
    Connected,
    Interupted
}

fn connect_to_db() -> Status {
    Status::Connected
}

fn get_user(){
    // get user from db
}

fn authenticate(user: ()){
    // auth logic
}

pub fn login(cred: Credentials) -> bool{
    if let Status::Connected = connect_to_db() {
        let user = get_user();
        authenticate(user);
        // login logic
        return true;
    }
    false
}


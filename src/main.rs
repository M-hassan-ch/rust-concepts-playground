use learning::auth::{login, models::Credentials};

fn main(){
    let cred = Credentials{
        password: String::from("admin"),
        username: String::from("admin")
    };

    login(cred);
}
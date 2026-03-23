use learning::{login, Credentials};


fn main(){
    let cred = Credentials{
        password: String::from("admin"),
        username: String::from("admin")
    };

    login(cred);
}
#![allow(unused)]

#[derive(Debug)]
struct Lang {
    language: String,
    version: String
}

fn main() {
    let lang = Lang {
        language: "rust".to_string(),
        version: "1.83".to_string()
    };

    println!("{:?}", lang);
    println!("{:#?}", lang);
}

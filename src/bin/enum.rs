
#[derive(Debug)]
#[derive(PartialEq)]
enum Color{
    Red,
    Hex(String),
    Struct{a: u8, b: bool}
}
fn main() {
    let enum1 = Color::Red;
    let enum2 = Color::Hex("0xff".to_string());
    let enum3 = Color::Struct { a: 8, b: true };

    println!("{:#?}", enum1);
    println!("{:#?}", enum2);
    println!("{:#?}", enum3);
    println!("{}", enum3 == Color::Struct{a:8, b:true});
}

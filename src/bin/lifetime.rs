fn get_longest_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn main() {
    let x = "rust".to_string();
    let longest = {
        let y = "longest string".to_string();
        get_longest_str(&x, &y)
    };
    // Error: y does not live long enough
    println!("{}", longest);
}

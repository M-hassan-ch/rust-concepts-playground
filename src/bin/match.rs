#![allow(unused)]

fn main() {
    let num = 2;

    match num {
        i @ 0..10 => println!("Number {i} between 0-10"),
        _ => println!("Number is greater then 10"),
    }

    // match alternative

    let num = Some(2);
    if let Some(v) = num {
        println!("Got some {v} ");
    } else {
        println!("Got None")
    }

    let num: Result<i32, String> = Err("error".to_string());
    if let Ok(v) = num {
        println!("Got OK {v} ");
    } else {
        println!("Got error")
    }
}

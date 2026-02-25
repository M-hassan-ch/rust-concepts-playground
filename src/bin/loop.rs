#![allow(unused)]
fn main() {
    // loop
    let mut k = 0;
    loop {
        println!("loop {}", k);
        if k == 2 {
            break;
        }
        k += 1;
    }

    // loop return value
    let mut k = 0;
    let result  = loop {
        if k == 2 {
            break k
        }
        k += 1;
    };
    println!("loop return value {}", result);

    // while``
    let mut k = 0;
    while k < 3 {
        println!("while {}", k);
        k += 1;
    }

    // for
    for k in 0..=3 {
        println!("for {}", k);
    }

    // for arrays
    let arr = [1, 2 , 3];
    for item in arr {
        println!("for item {}", item);
    }

    // labels
    'outer: for i in 0..3 {
        'inner: for j in 0..3 {
            println!("{} : {}", i, j);
            if j == 2 {
                break 'outer;
            }
        }
    }
}

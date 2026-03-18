use std::thread;
use std::time::Duration;

fn main() {
    let thread_1 = thread::spawn(|| {
        for i in 1..=5 {
            println!("1st thread {}", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });

    let thread_2 = thread::spawn(|| {
        for i in 1..=5 {
            println!("2nd thread {}", i);
            thread::sleep(Duration::from_millis(1000));
        }
    });

    // waitng for threads to complete
    // thread_1.join().unwrap();
    // thread_2.join().unwrap();

    let thread_3 = thread::spawn(|| "Thread".to_string());
    let thread_result = thread_3.join().unwrap();
    println!("{}", thread_result);

    // Scoped threads
    let msg = "Scoped Thread".to_string();
    thread::scope(|scope | {
        scope.spawn(|| {
            // No move operation
            println!("{}", msg)
        });
    });
    println!("{}", msg);
}

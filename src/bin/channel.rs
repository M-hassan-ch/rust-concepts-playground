#![allow(unused)]
use std::sync::mpsc::{self, Receiver, Sender, TryRecvError};
use std::thread;
use std::time::Duration;

fn main() {
    let channel: (Sender<String>, Receiver<String>) = mpsc::channel();
    let (tx, rx) = channel;

    let tr = thread::spawn(move || {
        // thread::sleep(Duration::from_millis(2000));
        tx.send("Hello_1!".to_string()).unwrap();
        tx.send("Hello_2!".to_string()).unwrap();
        tx.send("Hello_3!".to_string()).unwrap();
        tx.send("Hello_4!".to_string()).unwrap();
    });

    // rx.recv() enforces main function to wait for the message to be received
    // loop {
    //     match rx.recv() {
    //         Ok(msg) => println!("Message received {:#?}", msg),
    //         Err(err) => {
    //             println!("Got error {:#?}", err);
    //             break;
    //         }
    //     }
    // }
    // println!("Message received {:#?}", response);

    // rx.try_recv() will not pause main()
    loop {
        match rx.try_recv() {
            Ok(msg) => {
                println!("Message received {:#?}", msg);
            }
            Err(TryRecvError::Disconnected) => {
                println!("Channel Disconnected");
                break;
            }
            Err(TryRecvError::Empty) => {
                println!("Received Empty message");
            }
        }
    }
}

#[allow(unused)]
use tokio::time::{sleep, Duration};

async fn f1(){
    println!("Executing f1");
    sleep(Duration::from_millis(4000)).await;
    println!("Executed f1");
}

async fn f2(){
    println!("Executing f2");
    sleep(Duration::from_millis(2000)).await;
    println!("Executed f2");
}

async fn f3(){
    println!("Executing f3");
    sleep(Duration::from_millis(4000)).await;
    println!("Executed f3");
}

async fn f(){
    tokio::join!(f1(), f2(), f3());
}

#[tokio::main]
async fn main(){
    f().await; 
}
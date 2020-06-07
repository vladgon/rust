extern crate web_server;

use core::result::Result;
use std::env;
use std::sync::{Arc, Mutex};

use tokio::task::JoinError;
use tracing::instrument;

#[instrument(err)]
#[tokio::main(threaded_scheduler, core_threads = 5, max_threads = 10)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::debug!("Starting main");
    tracing::debug!("Log env '{}'", env::var("RUST_LOG").unwrap_or("INFO".into()));
    println!("Res {:?}", demo_task().await);
    Ok(())
}

#[instrument(debug)]
fn main_man() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    tracing::debug!("Starting main");
    tracing::debug!("Log env '{}'", env::var("RUST_LOG").unwrap_or("INFO".into()));
    tokio::runtime::Builder::new()
        .threaded_scheduler()
        .core_threads(5)
        .max_threads(10)
        .build()?
        .block_on(async {
            println!("Res {:?}", demo_task().await);
            Ok(())
        })
}

#[instrument]
async fn demo_task() -> Result<Vec<i32>, JoinError> {
    let hello = String::from("Hello");
    let hello = Arc::new(Mutex::new(hello));

    let blocking_task: Vec<_> = (1..=10)
        .map(move |i| {
            let hello = hello.clone();
            tokio::task::spawn(async move {
                test_f(format!("test_f {} {:?}", i, hello.lock().unwrap()), i).await
            })
            // test_f(format!("test_f {} {:?}", i, hello.lock().unwrap()), i)
        })
        .collect();
    futures::future::try_join_all(blocking_task).await
    // let result = futures::future::try_join_all(blocking_task).await;
}


#[instrument]
async fn test_f(s: String, i: i32) -> i32 {
    tracing::debug!("arg {}, {:?}", s, std::thread::current());
    i
}
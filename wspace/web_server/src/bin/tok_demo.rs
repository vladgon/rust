extern crate web_server;

use std::env;
use std::sync::{Arc, Mutex};

use ctor::ctor;
use futures::{StreamExt, TryStreamExt};
use log::{debug, info};
use tracing::instrument;

use wg_util::Result;

#[ctor]
fn init_tracing() {
    tracing_subscriber::fmt()
        .with_target(true)
        .with_file(false)
        .with_timer(tracing_subscriber::fmt::time::time())
        .with_line_number(true)
        .with_thread_ids(true)
        .with_level(true)
        .with_max_level(tracing::Level::INFO)
        .init();
    // wg_util::common::config::rust_app::init(Debug, false);
}

#[instrument(err)]
#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
#[instrument(level = "debug")]
async fn main() -> Result<()> {
    info!("Starting main");
    info!("Log env '{}'", env::var("RUST_LOG").unwrap_or("INFO".into()));
    info!("Res {:?}", demo_task().await);
    Ok(())
}

#[instrument(level = "debug")]
fn main_man() -> Result<()> {
    debug!("Starting main");
    debug!("Log env '{}'", env::var("RUST_LOG").unwrap_or("INFO".into()));
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(5)
        .build()?
        .block_on(async {
            println!("Res {:?}", demo_task().await);
            Ok(())
        })
}

#[instrument]
async fn demo_task() -> Result<Vec<i32>> {
    let hello = Arc::new(Mutex::new(String::from("Hello")));
    let res = futures::stream::iter(0..10)
        .map(move |i| {
            let hello = hello.clone();
            tokio::task::spawn(async move {
                test_f(format!("test_f {} {:?}", i, hello.lock().unwrap()), i).await
            })
        })
        .buffered(12)
        .try_collect()
        // .map_err(JoinError::into)
        .await;
    res.map_err(|e| e.into())
}


#[instrument]
async fn test_f(s: String, i: i32) -> i32 {
    info!("arg {}, {:?}", s, std::thread::current());
    // let i = 1 / 0;
    i
}
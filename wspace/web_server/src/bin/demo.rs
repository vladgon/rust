use std::sync::Arc;
use std::sync::atomic::AtomicI16;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::spawn;
use std::time::Duration;

use ctor::ctor;
use futures::{FutureExt, StreamExt, TryStreamExt};
use rand::random;
use tokio::task::JoinError;

use wg_util::common::config::log::{LogDefaults, LogLevelEntry};
use wg_util::common::config::log::Level::Debug;
use wg_util::common::config::log::LogImplType::Tracing;
use wg_util::common::config::rust_app;

#[ctor]
fn init() {
    spawn(|| {
        _ = rust_app::init(LogDefaults::new(Tracing, &[LogLevelEntry::Level(Debug)]), false);
    })
        .join()
        .expect("Failed to init the app");
}

#[tokio::main(flavor = "multi_thread", worker_threads = 30)]
async fn main() -> wg_util::Result<()> {
    run_most_of().await?;

    // let hello = Arc::new(Mutex::new("hello"));
    // let blocking_task: Vec<JoinHandle<i32>> = (1..10)
    //     .map(move |i| {
    //         let hello = hello.clone();
    //         tokio::task::spawn(async move {
    //             let guard = hello.lock().unwrap();
    //             println!("{:?}, {:?}", std::thread::current(), guard);
    //             i
    //         })
    //     })
    //     .collect();

// let res1: Vec<i32> = tokio::task::spawn(async move {
//     let mut res = vec![];
//     for f in blocking_task {
//         res.push(f.await.unwrap());
//     }
//     res
//     // blocking_task.iter().clone()
//     //     .map(|h: JoinHandle<i32>| { h.await })
//     //     .map(|r| r.unwrap())
//     //     .collect()
// }).await.unwrap();
// let blocking_task: Vec<i32> = futures::future::join_all(blocking_task).await
//     .into_iter()
//     .map(|r| r.unwrap()).collect();

// println!("{:?}", res1);
// println!("{:?}", blocking_task);


// let res2 = tokio::task::spawn_blocking(|| {
//     (1..10)
//         .map(|i| {
//             println!("{:?}", std::thread::current());
//             std::thread::sleep(Duration::from_secs(1));
//             i
//         })
//         .collect::<Vec<i32>>()
// }).await.unwrap();
// .into_iter()
// .map(|r| r.unwrap());
//
// println!("res2 {:?}", res2);
    Ok(())
}

async fn run_most_of() -> Result<Vec<i32>, JoinError> {
    const PARALLELISM: usize = 10;
    const DELAY_MILLIS: u64 = 5_000;
    let counter = Arc::new(AtomicI16::from(0));
    futures::stream::iter(0..100)
        .map(|i: i32| {
            let counter = counter.clone();
            // let delay = between.sample(&rng);
            tokio::task::spawn(async move {
                counter.fetch_add(1, Relaxed);
                let delay = random::<u64>() % DELAY_MILLIS;
                tokio::time::sleep(Duration::from_millis(delay))
                    .then(|()| async move {
                        counter.fetch_add(-1, Relaxed);
                        println!("{:?}, buffered: {}, index: {i}, delay {delay}",
                                 std::thread::current().id(),
                                 counter.load(Relaxed)
                        );
                        i
                    })
                    .await
            })
        })
        .buffer_unordered(PARALLELISM)
        .try_collect::<Vec<i32>>()
        .await
}
use core::result::Result;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use tokio::task::{JoinError, JoinHandle};

#[tokio::main(flavor = "multi_thread", worker_threads = 5)]
async fn main() -> Result<(), JoinError> {
    let hello = String::from("Hello");
    let hello = Arc::new(Mutex::new(hello));

    let blocking_task: Vec<JoinHandle<i32>> = (1..10)
        .map(move |i| {
            let hello = hello.clone();
            tokio::task::spawn(async move {
                let guard = hello.lock().unwrap();
                println!("{:?}, {:?}", std::thread::current(), guard);
                i
            })
        })
        .collect();

    let res1: Vec<i32> = tokio::task::spawn(async move {
        let mut res = vec![];
        for f in blocking_task {
            res.push(f.await.unwrap());
        }
        res
        // blocking_task.iter().clone()
        //     .map(|h: JoinHandle<i32>| { h.await })
        //     .map(|r| r.unwrap())
        //     .collect()
    }).await.unwrap();
    // let blocking_task: Vec<i32> = futures::future::join_all(blocking_task).await
    //     .into_iter()
    //     .map(|r| r.unwrap()).collect();

    println!("{:?}", res1);
    // println!("{:?}", blocking_task);


    let res2 = tokio::task::spawn_blocking(|| {
        (1..10)
            .map(|i| {
                println!("{:?}", std::thread::current());
                std::thread::sleep(Duration::from_secs(1));
                i
            })
            .collect::<Vec<i32>>()
    }).await.unwrap();
    // .into_iter()
    // .map(|r| r.unwrap());
    //
    println!("res2 {:?}", res2);
    Ok(())
}
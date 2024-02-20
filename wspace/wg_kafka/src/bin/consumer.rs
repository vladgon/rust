extern crate wg_kafka;

use std::sync::Arc;
use std::sync::atomic::{AtomicI16, Ordering};
use std::thread::sleep;
use std::time::Duration;

use log::debug;

use wg_kafka::consumer;
use wg_kafka::model::SampleData;
use wg_util::{IteratorExt, Result};
use wg_util::common::config;
use wg_util::common::config::app_config;
use wg_util::common::config::rust_app::Options;

fn main() -> Result<()> {
    config::rust_app::init(Options::DefaultLogNoClap)?;
    let settings = app_config::settings()?;

    let topic = settings.kafka.topic.to_owned();
    let group = "my-group".to_owned();
    consume_messages(group, topic, &[settings.kafka.broker.clone()])
}

fn consume_messages(group: String, topic: String, brokers: &[String]) -> Result<()> {
    let mut consumer = consumer(group, topic, brokers)?;
    let counter = Arc::new(AtomicI16::new(0));
    loop {
        let mss = consumer.poll()?;
        if mss.is_empty() {
            debug!("No messages available, sleeping ...");
            sleep(Duration::from_millis(app_config::settings()?.kafka.pollSleep));
            continue;
        }

        _ = mss.iter()
               .tap(|ms| {
                   use rayon::prelude::*;
                   ms.messages()
                     .par_iter()
                     .for_each(|mes| debug!("{}:  {}:{}@{}: {:?}",
                        counter.fetch_add(1, Ordering::Relaxed),
                        ms.topic(),
                        ms.partition(),
                        mes.offset,
                        serde_json::from_slice::<SampleData>(mes.value)
                ))
               })
               .try_for_each(|ms| consumer.consume_messageset(ms));
        consumer.commit_consumed()?
    }
}

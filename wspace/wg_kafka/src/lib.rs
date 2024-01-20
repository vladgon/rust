use std::time::Duration;

use kafka::client::{FetchOffset, GroupOffsetStorage};
use kafka::consumer::Consumer;
use kafka::producer::{Producer, RequiredAcks};
use log::debug;

use wg_util::{Result, ResultExt};

pub mod model;

pub fn producer(brokers: &[String]) -> Result<Producer> {
    debug!("Creating Producer {:?}", brokers);
    Producer::from_hosts(brokers.into())
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .into_std_error()
}

pub fn consumer(group: String, topic: String, brokers: &[String]) -> Result<Consumer> {
    debug!("Creating Producer {:?}", brokers);
    Consumer::from_hosts(brokers.into())
        .with_topic(topic)
        .with_group(group)
        .with_fallback_offset(FetchOffset::Latest)
        .with_offset_storage(Some(GroupOffsetStorage::Kafka))
        .create()
        .into_std_error()
}
use std::time::Duration;

use kafka::Error;
use kafka::producer::{Producer, RequiredAcks};
use log::debug;

use wg_util::Result;

pub mod model;

pub fn producer(brokers: Vec<String>) -> Result<Producer> {
    debug!("Creating Producer {:?}", brokers);
    Producer::from_hosts(brokers)
        .with_ack_timeout(Duration::from_secs(1))
        .with_required_acks(RequiredAcks::One)
        .create()
        .map_err(Error::into)
}
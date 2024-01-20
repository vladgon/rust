extern crate wg_kafka;

use chrono::Local;
use kafka::producer::Record;
use log::debug;

use wg_kafka::model;
use wg_util::common::config;
use wg_util::common::config::log::LogDefaults;
use wg_util::ResultExt;

fn main() -> wg_util::Result<()> {
    config::rust_app::init(LogDefaults::default(), false)?;
    let settings = config::app_config::settings()?;

    let mut producer = wg_kafka::producer(&[settings.kafka.broker.clone()])?;
    let topic = settings.kafka.topic.as_str();
    (1..100)
        .try_for_each(|_| {
            let data = serde_json::to_string(&model::SampleData {
                name: "Hello".to_string(),
                time: Local::now(),
            })?;
            debug!("Publishing {data}");
            producer.send(&Record::from_value(topic, data))
                .into_std_error()
        })
        .map(|_| debug!("Done!"))
}


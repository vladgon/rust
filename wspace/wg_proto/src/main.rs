use chrono::Utc;
use log::debug;
use prost::Message;
use prost_wkt_types::Timestamp;

use wg_util::common::config::log::LogDefaults;
use wg_util::common::config::rust_app;
use wg_util::Result;

use crate::proto::model;

include!(concat!(env!("OUT_DIR"), "/proto.model.include.rs"));

fn main() -> Result<()> {
    rust_app::init(LogDefaults::default(), false)?;

    let mut shirt = model::Shirt::default();
    shirt.color = Some("Green".to_string());
    shirt.size = model::shirt::Size::Large.into();
    shirt.set_size(model::shirt::Size::Large);
    shirt.created_on = Some(Timestamp::from(Utc::now()));
    let res: Vec<u8> = shirt.encode_to_vec();
    debug!("Shirt {:?}", res);
    debug!("Shirt {:?}", serde_json::to_string(&shirt)?);

    let foo = model::Foo {
        name: str::to_string("foo"),
        types: model::PhoneType::Home1.into(),
        ..Default::default()
    };
    debug!("{} {:?}", "ProtoSchema::",  serde_json::to_string(&foo)?);
    Ok(())
}
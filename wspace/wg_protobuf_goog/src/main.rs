use std::time::SystemTime;

use log::debug;
use protobuf::{Message, MessageDyn, MessageField};
use protobuf::well_known_types::timestamp::Timestamp;

use proto_generated::sample;
use sample::Shirt;
use wg_util::common::config::log::LogDefaults;
use wg_util::common::config::rust_app;

use crate::proto_generated::sample1::{Foo, PhoneType};
use crate::proto_generated::sample::shirt::Size;

mod proto_generated {
    include!(concat!(env!("OUT_DIR"), "/proto_generated/mod.rs"));
}

fn main() -> wg_util::Result<()> {
    rust_app::init(LogDefaults::default(), false)?;

    let shirt = Shirt {
        color: Some("Green".to_string()),
        size: Size::LARGE.into(),
        createdOn: MessageField::some(Timestamp::from(SystemTime::now() - chrono::Duration::days(1).to_std()?)),
        ..Default::default()
    };
    let bytes = shirt.write_to_bytes()?;
    debug!("To Bytes Shirt: {:?}", bytes);
    let from_bytes = Shirt::parse_from_bytes(bytes.as_slice())?;
    debug!("From Bytes {}", from_bytes);
    debug!("Json Shirt {}", protobuf_json_mapping::print_to_string(&shirt)?);

    let foo_struct = Foo {
        name: str::to_string("foo"),
        types: PhoneType::HOME1.into(),
        ..Default::default()
    };
    debug!("Foo Json: {}",  protobuf_json_mapping::print_to_string(&foo_struct)?);

    let desc = Shirt::new().descriptor_dyn();
    debug!("Dynamic fields for Shirt {:?}", desc.fields()
        .map(|fd|fd.full_name().to_owned())
        .collect::<Vec<String>>());
    Ok(())
}

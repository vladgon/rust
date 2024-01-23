use log::debug;
use prost::Message;

use wg_util::common::config::log::LogDefaults;
use wg_util::common::config::rust_app;
use wg_util::Result;

use crate::wg::proto::model;

pub mod wg {
    pub mod proto {
        pub mod model {
            include!(concat!(env!("OUT_DIR"), "/wg.proto.model.rs"));
        }
    }
}

fn main() -> Result<()> {
    rust_app::init(LogDefaults::default(), false)?;

    let mut shirt = model::Shirt::default();
    let mut foo1 = model::Foo1::default();
    shirt.color = Some("Green".to_string());
    shirt.set_size(model::shirt::Size::Large);
    let res: Vec<u8> = shirt.encode_to_vec();
    debug!("Shirt {:?}", res);
    debug!("Shirt {:?}", serde_json::to_string(&shirt)?);
    foo1.name = "foo1".to_string();
    foo1.set_types(model::PhoneType1::Mobile1);
    debug!("{} {:?}", "ProtoSchema::",  serde_json::to_string(&foo1)?);
    Ok(())
}
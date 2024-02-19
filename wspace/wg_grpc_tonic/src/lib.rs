pub use prost_wkt_types::Timestamp;
pub use tonic::{Request, Response, Status};
pub use tonic::transport::Channel;
pub use tonic::transport::Error;

pub use crate::hello_world::helloworld::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("include");
}

#[derive(utoipa::ToSchema, utoipa::IntoParams)]
#[allow(non_snake_case)]
#[schema(as =::prost_wkt_types::Timestamp, example = "2024-02-19T14:20:19.427861Z", title = "Timestamp")]
pub struct TimestampReplacement {
    pub isoDateTime: String,
}
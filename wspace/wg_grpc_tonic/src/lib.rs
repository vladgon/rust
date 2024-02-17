pub use tonic::{Request, Response, Status};
pub use tonic::transport::Channel;
pub use tonic::transport::Error;

pub use crate::hello_world::helloworld::{HelloReply, HelloRequest};

pub mod hello_world {
    tonic::include_proto!("include");
}


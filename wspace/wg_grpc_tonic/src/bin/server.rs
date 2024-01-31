use std::time::SystemTime;

use log::debug;
use prost_wkt_types::Timestamp;
use tonic::{Request, Response, Status, transport::Server};

use helloworld::{HelloReply, HelloRequest};
use helloworld::greeter_server::Greeter;
use helloworld::greeter_server::GreeterServer;
use wg_util::common::config::app_config::settings;
use wg_util::common::config::log::{LogConfig, Logger};
use wg_util::common::config::log::Level::Debug;
use wg_util::common::config::log::LogProvider::Tracing;
use wg_util::common::config::model::Grpc;
use wg_util::common::config::rust_app;
use wg_util::ResultExt;

include!(concat!(env!("OUT_DIR"), "/include.rs"));

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        debug!("Got a request: {:?}", request);
        let reply = HelloReply {
            message: format!("Hello {}!", request.into_inner().name),
            created_on: Timestamp::from(SystemTime::now()).into(),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 30)]
async fn main() -> wg_util::Result<()> {
    rust_app::init(LogConfig::new(Tracing,
                                  &[Logger::LoggerForModule("wg_util", Debug),
                                      Logger::LoggerForModule("server", Debug)
                                  ]),
                   false)?;
    let Grpc { host, port } = &settings()?.grpc;
    let addr = format!("{host}:{port}").parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await
        .into_std_error()
}
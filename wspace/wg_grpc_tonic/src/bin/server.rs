use std::time::SystemTime;

use log::debug;
use prost_wkt_types::Timestamp;
use tonic::{Request, Response, Status, transport::Server};

use helloworld::{HelloReply, HelloRequest};
use helloworld::greeter_server::Greeter;
use helloworld::greeter_server::GreeterServer;
use Options::LogAndClap;
use wg_util::common::config::app_config::settings;
use wg_util::common::config::log::{LogConfig, Logger};
use wg_util::common::config::log::Level::Debug;
use wg_util::common::config::log::LogProvider::Tracing;
use wg_util::common::config::model::HostPort;
use wg_util::common::config::rust_app;
use wg_util::common::config::rust_app::Options;
use wg_util::ResultExt;

include!(concat!(env!("OUT_DIR"), "/include.rs"));

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Greeter for MyGreeter {
    async fn say_hello(&self, request: Request<HelloRequest>) -> Result<Response<HelloReply>, Status> {
        debug!("Got a request: {:?}", request);
        let request = request.into_inner();
        let reply = HelloReply {
            message: format!("Hello {}!", request.name),
            created_on: request.created_on.clone().or_else(||
                Timestamp::from(SystemTime::now()).into()),
        };

        Ok(Response::new(reply))
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 30)]
async fn main() -> wg_util::Result<()> {
    rust_app::init(LogAndClap(LogConfig::new(Tracing,
                                             &[Logger::LoggerForModule("wg_util", Debug),
                                                 Logger::LoggerForModule("server", Debug)
                                             ]),
                              false)
    )?;
    let HostPort { host, port } = &settings()?.grpc;
    let addr = format!("{host}:{port}").parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(GreeterServer::new(greeter))
        .serve(addr)
        .await
        .into_std_error()
}
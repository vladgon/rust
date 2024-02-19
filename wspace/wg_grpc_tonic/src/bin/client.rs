use std::backtrace::Backtrace;
use std::time::SystemTime;

use futures::{StreamExt, TryStreamExt};
use log::{error, info};
use prost_wkt_types::Timestamp;
use tonic::{Response, Status};

use wg_util::{IteratorExt, ResultExt};
use wg_util::common::config::app_config::settings;
use wg_util::common::config::log::{LogConfig, Logger};
use wg_util::common::config::log::Level::{Debug, Info};
use wg_util::common::config::log::LogProvider::Tracing;
use wg_util::common::config::model::HostPort;
use wg_util::common::config::rust_app;
use wg_util::common::config::rust_app::Options::LogAndClap;
use wg_util::common::result_ext::ResultTap;

use crate::hello_world::helloworld::{HelloReply, HelloRequest};
use crate::hello_world::helloworld::greeter_client::GreeterClient;

pub mod hello_world {
    tonic::include_proto!("include");
}

#[tokio::main(flavor = "multi_thread", worker_threads = 30)]
async fn main() -> wg_util::Result<()> {
    rust_app::init(LogAndClap(LogConfig::new(Tracing,
                                             &[
                                                 Logger::LoggerRoot(Info),
                                                 Logger::LoggerForModule("client", Debug)
                                             ]),
                              false)
    )?;

    let HostPort { host, port } = &settings()?.grpc;
    _ = (0..1)
        .tap(|_| {})
        .collect::<Vec<_>>();
    let responses = futures::stream::iter(0..1_000)
        .map(|_| {
            tokio::spawn(async move {
                let request = tonic::Request::new(
                    HelloRequest {
                        name: "Tonic".into(),
                        created_on: Timestamp::from(SystemTime::now()).into(),
                    });
                let mut client = GreeterClient::connect(format!("http://{host}:{port}")).await
                                                                                        .map_err(|e| Status::from_error(e.into()))?;
                client.say_hello(request).await
                      .tap_ignore_result(|ok| {
                          info!("Response as Json: {}",
                              serde_json::to_string(ok.get_ref()).map_err(|e| Status::unknown(e.to_string()))?);
                          Ok::<_, Status>(())
                      })
            })
        })
        .buffer_unordered(50)
        .map(|r| r?.into_std_error())
        .try_collect::<Vec<Response<HelloReply>>>()
        .await
        .tap_err(|e| error!("Error {}\n{}", e, Backtrace::capture()))?;
    info!("Responses count {}", responses.len());
    Ok(())
}
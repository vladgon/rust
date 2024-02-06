use std::backtrace::Backtrace;

use futures::{FutureExt, StreamExt, TryStreamExt};
use log::{error, info};
use tonic::{Response, Status};

use wg_util::{ResultExt, ResultTap, StdErrorBox};
use wg_util::common::config::app_config::settings;
use wg_util::common::config::log::{LogConfig, Logger};
use wg_util::common::config::log::Level::{Debug, Info};
use wg_util::common::config::log::LogProvider::Tracing;
use wg_util::common::config::model::Grpc;
use wg_util::common::config::rust_app;
use wg_util::common::config::rust_app::Options::LogAndClap;

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

    let Grpc { host, port } = &settings()?.grpc;
    let responses = futures::stream::iter(0..1_000)
        .map(|_| {
            tokio::spawn(async move {
                let request = tonic::Request::new(
                    HelloRequest {
                        name: "Tonic".into(),
                    });
                GreeterClient::connect(format!("http://{host}:{port}"))
                    .then(|client| async move {
                        client.map_err(|e| Status::from_error(e.into()))?.say_hello(request).await
                    })
                    .map(|result| {
                        result.tap_ignore_result(|ok| {
                            info!("Response as Json: {}",
                                serde_json::to_string(ok.get_ref()).map_err(|e| Status::from_error(e.into()))?);
                            Ok::<_, StdErrorBox>(())
                        })
                    })
                    .await
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
use futures::{FutureExt, StreamExt, TryStreamExt};
use log::{error, info};
use tonic::Response;

use wg_util::{ResultExt, Tap};
use wg_util::common::config::app_config::settings;
use wg_util::common::config::log::{LogConfig, Logger};
use wg_util::common::config::log::Level::{Debug, Info};
use wg_util::common::config::log::LogProvider::Tracing;
use wg_util::common::config::rust_app;

use crate::hello_world::helloworld::{HelloReply, HelloRequest};
use crate::hello_world::helloworld::greeter_client::GreeterClient;

pub mod hello_world {
    tonic::include_proto!("include");
}

#[tokio::main(flavor = "multi_thread", worker_threads = 30)]
async fn main() -> wg_util::Result<()> {
    rust_app::init(LogConfig::new(Tracing,
                                  &[
                                      Logger::LoggerRoot(Info),
                                      Logger::LoggerForModule("client", Debug)]),
                   false)?;

    let host = settings()?.grpc.host.as_str();
    let port = settings()?.grpc.port.as_str();
    let responses = futures::stream::iter(0..1_000)
        .map(|_| {
            tokio::spawn(async move {
                let request = tonic::Request::new(
                    HelloRequest {
                        name: "Tonic".into(),
                    });
                GreeterClient::connect(format!("http://{host}:{port}"))
                    .then(|client| async move { client.unwrap().say_hello(request).await })
                    .map(|result| {
                        result.tap(|response| {
                            info!("Response as Json: {}",  serde_json::to_string(response.get_ref()).unwrap());
                            info!("RESPONSE={:?}", response.get_ref());
                        })
                    })
                    .await
            })
        })
        .buffer_unordered(50)
        .map(|r| r?.into_std_error())
        .try_collect::<Vec<Response<HelloReply>>>()
        .await
        .tap_err(|e| error!("Error {}", &e))?;
    info!("Responses count {}", responses.len());
    Ok(())
}
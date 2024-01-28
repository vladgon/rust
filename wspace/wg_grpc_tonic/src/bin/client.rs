use futures::{StreamExt, TryStreamExt};
use log::info;
use tonic::Response;

use wg_util::common::config::app_config::settings;
use wg_util::common::config::log::{LogDefaults, LogEntry};
use wg_util::common::config::log::Level::{Debug, Info};
use wg_util::common::config::log::LogImplType::Tracing;
use wg_util::common::config::rust_app;
use wg_util::ResultExt;

use crate::hello_world::helloworld::{HelloReply, HelloRequest};
use crate::hello_world::helloworld::greeter_client::GreeterClient;

pub mod hello_world {
    tonic::include_proto!("include");
}

#[tokio::main(flavor = "multi_thread", worker_threads = 30)]
async fn main() -> wg_util::Result<()> {
    rust_app::init(LogDefaults::new(Tracing,
                                    &[
                                        LogEntry::all_modules(Info),
                                        LogEntry::new("client", Debug)]),
                   false)?;

    let host = settings()?.grpc.host.as_str();
    let port = settings()?.grpc.port.as_str();
    let responses = futures::stream::iter(0..1_000)
        .map(move |_| {
            tokio::spawn(async move {
                let request = tonic::Request::new(HelloRequest {
                    name: "Tonic".into(),
                });
                let client = GreeterClient::connect(format!("http://{host}:{port}")).await;
                let response = client.unwrap().say_hello(request).await;
                let response_ref = response.as_ref();
                info!("Response as Json: {}",  serde_json::to_string(response_ref.unwrap().get_ref()).unwrap());
                info!("RESPONSE={:?}", response_ref);
                response
            })
        })
        .buffer_unordered(50)
        .map(|r| r.map(Result::into_std_error).into_std_error()?)
        .try_collect::<Vec<Response<HelloReply>>>()
        .await?;
    info!("Responses count {}", responses.len());
    Ok(())
}
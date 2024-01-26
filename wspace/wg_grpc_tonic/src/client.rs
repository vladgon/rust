use futures::{FutureExt, StreamExt, TryStreamExt};
use log::info;
use tonic::codegen::Body;
use tonic::Response;

use wg_util::common::config::app_config::settings;
use wg_util::common::config::log::Level::Info;
use wg_util::common::config::log::LogDefaults;
use wg_util::common::config::log::LogImplType::Tracing;
use wg_util::common::config::rust_app;
use wg_util::ResultExt;

use crate::hello_world::helloworld::{HelloReply, HelloRequest};
use crate::hello_world::helloworld::greeter_client::GreeterClient;

pub mod hello_world {
    tonic::include_proto!("include");
}

#[tokio::main(flavor = "multi_thread", worker_threads = 30)]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    _ = rust_app::init(LogDefaults { log_type: Tracing, default_level: Info }, false);

    let host = settings()?.grpc.host.as_str();
    let port = settings()?.grpc.port.as_str();


    let res = futures::stream::iter(0..1_000)
        .map(move |_| {
            tokio::spawn(async move {
                let request = tonic::Request::new(HelloRequest {
                    name: "Tonic".into(),
                });
                let client = GreeterClient::connect(format!("http://{host}:{port}")).await;
                let response = client.unwrap().say_hello(request).await;
                let response = response.unwrap();
                info!("Response as Json: {}",  serde_json::to_string(response.get_ref()).unwrap());
                info!("RESPONSE={:?}", response.get_ref());
                response
            })
        })
        .buffer_unordered(50)
        .try_collect::<Vec<Response<HelloReply>>>()
        .await;
    Ok(())
}
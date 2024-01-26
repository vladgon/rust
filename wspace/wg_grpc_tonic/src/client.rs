use log::debug;

use wg_util::common::config::app_config::settings;
use wg_util::common::config::log::LogDefaults;
use wg_util::common::config::rust_app;

use crate::hello_world::helloworld::greeter_client::GreeterClient;
use crate::hello_world::helloworld::HelloRequest;

pub mod hello_world {
    tonic::include_proto!("include");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    _ = rust_app::init(LogDefaults::default(), false);

    let host = settings()?.grpc.host.as_str();
    let port = settings()?.grpc.port.as_str();
    let mut client = GreeterClient::connect(format!("http://{host}:{port}")).await?;

    let request = tonic::Request::new(HelloRequest {
        name: "Tonic".into(),
    });

    let response = client.say_hello(request).await?;
    let response = response.get_ref();
    debug!("Foo Json: {}",  serde_json::to_string(response)?);


    println!("RESPONSE={:?}", response);

    Ok(())
}
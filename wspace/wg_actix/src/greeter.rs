use actix_web::{post, web};

use wg_grpc_tonic::{Channel, HelloReply, HelloRequest};
use wg_grpc_tonic::hello_world::helloworld::greeter_client::GreeterClient;
use wg_util::common::config::app_config::settings;
use wg_util::common::config::model::HostPort;
use wg_util::StdErrorBox;

pub async fn greeter_client() -> Result<GreeterClient<Channel>, wg_grpc_tonic::Error> {
    let HostPort { host, port } = &settings().unwrap().grpc;
    GreeterClient::connect(format!("http://{host}:{port}")).await
}

#[utoipa::path(
post,
path = "/sayHello",
request_body = HelloRequest,
responses(
(status = 200, description = "JSON file", body = HelloReply)
)
)]
#[post("/sayHello", guard = "crate::guards::accept_json")]
async fn say_hello(req: web::Json<HelloRequest>) -> Result<web::Json<HelloReply>, StdErrorBox> {
    let reply = greeter_client().await?
        .say_hello(wg_grpc_tonic::Request::new(req.into_inner())).await?
        .into_inner();
    Ok(web::Json(reply))
}

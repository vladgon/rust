use std::net::Ipv6Addr;

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use actix_web::middleware::Logger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use app_config::settings;
use config::log::Logger::LoggerRoot;
use wg_util::common::config;
use wg_util::common::config::app_config;
use wg_util::common::config::log::Level::{Debug, Info, Trace};
use wg_util::common::config::log::LogConfig;
use wg_util::common::config::log::Logger::LoggerForModule;
use wg_util::common::config::log::LogProvider::EnvLog;
use wg_util::common::config::rust_app::Options::LogAndClap;
use wg_util::ResultExt;

mod greeter;
mod guards;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}


async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[derive(OpenApi)]
#[openapi(
paths(greeter::say_hello),
components(schemas(wg_grpc_tonic::HelloRequest, wg_grpc_tonic::HelloReply, wg_grpc_tonic::TimestampReplacement),
),
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> wg_util::Result<()> {
    config::rust_app::init(LogAndClap(LogConfig::new(EnvLog,
                                                     &[
                                                         LoggerRoot(Info),
                                                         LoggerForModule("actix", Info),
                                                         LoggerForModule("wg_", Debug),
                                                         LoggerForModule("utoipa", Trace),
                                                     ]),
                                      false))?;
    HttpServer::new(|| {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-docs/openapi.json", ApiDoc::openapi())
            )
            .wrap(Logger::default())
            .service(hello)
            .service(greeter::say_hello)
            .route("/hey", web::get().to(manual_hello))
    })
        .bind((Ipv6Addr::UNSPECIFIED, settings()?.web.port))?
        .run()
        .await
        .into_std_error()
}


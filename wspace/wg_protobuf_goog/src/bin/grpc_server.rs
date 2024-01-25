// use tonic::{Request, Response, Status, transport::Server};
// use crate::proto_generated::sample;
//
// mod proto_generated {
//     include!(concat!(env!("OUT_DIR"), "/proto_generated/mod.rs"));
// }
//
//
// // defining a struct for our service
// #[derive(Default)]
// pub struct MySay {}
//
// // implementing rpc for service defined in .proto
// #[tonic::async_trait]
// impl sample::Shirt for MySay {
//     // our rpc impelemented as function
//     async fn send(&self, request: Request<GreeterRequest>) -> Result<Response<SayResponse>, Status> {
//         // returning a response as SayResponse message as defined in .proto
//         Ok(Response::new(SayResponse {
//             // reading data from request which is awrapper around our SayRequest message defined in .proto
//             message: format!("hello {}", request.get_ref().name),
//         }))
//     }
// }
//
// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // defining address for our service
//     let addr = "[::1]:50051".parse().unwrap();
//     // creating a service
//     let say = MySay::default();
//     println!("Server listening on {}", addr);
//     // adding our service to our server.
//     Server::builder()
//         .add_service(SayServer::new(say))
//         .serve(addr)
//         .await?;
//     Ok(())
// }
pub mod hello {
    tonic::include_proto!("hello"); // The string specified here must match the proto package name
}

use tonic::{transport::Server, Request, Response, Status};

// Service Name: Say
use hello::say_server::{Say, SayServer}; // It's a convention in Rust gRPC to have {service_name}_server for server-side stubs.

use hello::{SayRequest, SayResponse};

#[derive(Debug, Default)]
pub struct MyGreeter {}

#[tonic::async_trait]
impl Say for MyGreeter {
    // should follow snake case
    async fn send(
        &self,
        request: Request<SayRequest>, // Accept request of type SayRequest
    ) -> Result<Response<SayResponse>, Status> {
        // Return an instance of type SayResponse
        println!("Got a request: {:?}", request);

        let reply = hello::SayResponse {
            // should follow snake case
            response_message: format!("Hello {}!", request.into_inner().name).into(), // We must use .into_inner() as the fields of gRPC requests and responses are private
        };

        Ok(Response::new(reply)) // Send back our formatted greeting
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let greeter = MyGreeter::default();

    Server::builder()
        .add_service(SayServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}

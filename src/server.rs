pub mod hello {
    tonic::include_proto!("hello"); // The string specified here must match the proto package name
}

use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};
use tokio_stream::iter;
use tokio_stream::{wrappers::ReceiverStream, Stream, StreamExt};

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

    // Specify the output of rpc call
    type SendStreamStream = ReceiverStream<Result<SayResponse, Status>>; // It is a convention in Rust gRPC to have {method_name}Stream for server-side streaming methods
                                                                         // implementation for rpc call
    async fn send_stream(
        &self,
        request: Request<SayRequest>,
    ) -> Result<Response<Self::SendStreamStream>, Status> {
        println!("/send_stream request = {:?}", request);

        // creating a queue or channel
        let (mut tx, rx) = mpsc::channel(4);

        // creating a new task
        tokio::spawn(async move {
            // looping and sending our response using stream
            for _ in 0..4 {
                // fixed size stream
                // sending response to our channel
                tx.send(Ok(SayResponse {
                    response_message: format!("hello"),
                }))
                .await
                .unwrap();

                // Sleep for 1 second
                sleep(Duration::from_secs(1)).await;
            }
            println!(" /// done sending");
        });
        // returning our reciever so that tonic can listen on reciever and send the response to client
        Ok(Response::new(ReceiverStream::new(rx)))
    }

    // implementation for rpc call
    async fn receive_stream(
        &self,
        request: Request<tonic::Streaming<SayRequest>>,
    ) -> Result<Response<SayResponse>, Status> {
        println!("/receive_stream request");

        let mut stream = request.into_inner();
        let mut res_message = String::from("");

        // while let Some(req) = stream.message().await? {
        //     res_message.push_str(&format!("Hola {}\n", req.name))
        // }

        // // returning our reciever so that tonic can listen on reciever and send the response to client
        // Ok(Response::new(SayResponse { response_message: res_message }))

        while let Some(say_req) = stream.next().await {
            let say_req = say_req?;
            println!("  ==> say_req = {:?}", say_req);
            res_message.push_str(&format!("Hola {}!, ", say_req.name))
        }
        Ok(Response::new(SayResponse {
            response_message: res_message,
        }))
    }

    type BidirectionalStreamStream = ReceiverStream<Result<SayResponse, Status>>; // convention in Rust gRPC to have {method_name}StreamStream for bidirectional streaming methods.

    // implementation for rpc call
    async fn bidirectional_stream(
        &self,
        request: Request<tonic::Streaming<SayRequest>>,
    ) -> Result<Response<Self::BidirectionalStreamStream>, Status> {
        println!("/bidirectional_stream request");

        let mut stream = request.into_inner();

        // creating a queue or channel
        let (mut tx, rx) = mpsc::channel(4);

        let mut res_message = String::from("");

        tokio::spawn(async move {
            while let Some(say_req) = stream.next().await {
                // let say_req = say_req?;
                let say_req = match say_req {
                    Ok(req) => req,
                    Err(e) => {
                        eprintln!("Error receiving request: {}", e);
                        return;
                    }
                };
                println!("  ==> say_req = {:?}", say_req);
                // send data as soon it is available
                tx.send(Ok(SayResponse {
                    response_message: format!("Hola {}", say_req.name),
                }))
                .await
                .unwrap();

                // Sleep for 1 second
                // sleep(Duration::from_secs(1)).await;
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
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

use hello::say_client::SayClient; // It's a convention in Rust gRPC to have {service_name}_client for client-side stubs.
use hello::SayRequest;

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SayClient::connect("http://[::1]:50051").await?;

    println!("*** SIMPLE RPC ***");
    let request = tonic::Request::new(SayRequest {
        name: "Tonic".into(),
    });
    let response = client.send(request).await?;
    println!("RESPONSE={:?}", response);

    println!("\n*** SERVER STREAMING ***");
    let request = tonic::Request::new(SayRequest {
        name: "Tonic".into(),
    });
    let mut stream = client.send_stream(request).await?.into_inner();
    // println!("stream={:?}", stream);
    while let Some(response) = stream.message().await? {
        println!("NOTE={:?}", response);
    }

    Ok(())
}

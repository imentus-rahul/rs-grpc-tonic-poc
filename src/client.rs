use hello::say_client::SayClient;
use hello::SayRequest;

pub mod hello {
    tonic::include_proto!("hello");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = SayClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(SayRequest {
        name: "Tonic".into(),
    });

    let response = client.send(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}

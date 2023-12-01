use hello::say_client::SayClient; // It's a convention in Rust gRPC to have {service_name}_client for client-side stubs.
use hello::SayRequest;

// use tokio_stream::iter;
use tokio::time::{sleep, Duration};
use async_stream;

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

    // println!("\n*** CLIENT STREAMING Without Delay ***");
    // let request = tonic::Request::new(iter(vec![
    //     SayRequest {
    //         name: String::from("alice"),
    //     },
    //     SayRequest {
    //         name: String::from("bob"),
    //     },
    //     SayRequest {
    //         name: String::from("charlie"),
    //     },
    // ]));
    // let response = client.receive_stream(request).await?.into_inner();
    // println!("RESPONSE={:?}", response);

    println!("\n*** CLIENT STREAMING ***");
    let request = tonic::Request::new(async_stream::stream! {
        let names = vec!["alice", "bob", "charlie"];
        for name in names {
            sleep(Duration::from_secs(1)).await;
            yield SayRequest {
                name: name.to_string(),
            };
        }
    });
    // let response = client.receive_stream(request).await?.into_inner();
    let response = client.receive_stream(request).await?;
    println!("RESPONSE={:?}", response);

    println!("\n*** BIDIRECTIONAL STREAMING ***");
    let request = tonic::Request::new(async_stream::stream! {
        let names = vec!["alice", "bob", "charlie"];
        for name in names {
            sleep(Duration::from_secs(1)).await;
            yield SayRequest {
                name: name.to_string(),
            };
        }
    });
    let mut response_stream = client.bidirectional_stream(request).await?.into_inner();
    while let Some(response) = response_stream.message().await? {
        println!("NOTE={:?}", response);
    }

    Ok(())
}

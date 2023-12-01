### Rust gRPC implementation using Tonic

This is a simple example of a gRPC server and client written in Rust using the [Tonic] library.

### Setup

- Install/Update Rust: `rustup update`
- Initialize a new Rust project: `cargo init`
  cargo add tonic tokio tokio-stream prost prost-types futures async_trait
- create proto files

```
mkdir proto
touch proto/hello.proto
```

- Application dependencies setup

```toml
[dependencies]
tonic = "0.10"
prost = "0.12"
tokio = { version = "1.0", features = ["macros", "rt-multi-thread"] }

[build-dependencies]
tonic-build = "0.10" # tonic-build for compiling .proto files to client libraries

[[bin]]
name="server"
path="src/server.rs"

[[bin]]
name="client"
path="src/client.rs"
```

- refer client and server code from `/src` folder
- create `build.rs` file in root folder

- using gRPC Client from command line

```
# Install grpcurl

# Download the latest release
wget https://github.com/fullstorydev/grpcurl/releases/download/v1.8.9/grpcurl_1.8.9_linux_x86_64.tar.gz

# Extract the downloaded file
tar -xvzf grpcurl_1.8.9_linux_x86_64.tar.gz

# Move grpcurl to your /usr/local/bin directory
sudo mv grpcurl /usr/local/bin/

# remove downloaded file
rm grpcurl_1.8.9_linux_x86_64.tar.gz

# check version
grpcurl -version

# run server
cargo run --bin server

# run client
grpcurl -plaintext -import-path ./proto -proto hello.proto -d '{"name": "Tonic"}' '[::1]:50051' hello.Say/Send

```

- Using gRPC Client from code: `cargo run --bin server` and `cargo run --bin client` on another terminal

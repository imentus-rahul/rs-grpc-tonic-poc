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

# run client: unary request to server and unary response from server
grpcurl -plaintext -import-path ./proto -proto hello.proto -d '{"name": "Tonic"}' '[::1]:50051' hello.Say/Send

# server stream: unary request to server and stream response from server
grpcurl -plaintext -import-path ./proto -proto hello.proto -d '{"name": "Tonic"}' '[::1]:50051' hello.Say/SendStream

# server stream: stream request to server and unary response from server
grpcurl -plaintext -import-path ./proto -proto hello.proto -d '{"name": "Tonic"}' '[::1]:50051' hello.Say/ReceiveStream
```

- Using gRPC Client from code: `cargo run --bin server` and `cargo run --bin client` on another terminal

### Error Handling in gRPC

- https://cloud.google.com/apis/design/errors#error_model
- Note: When using gRPC, errors are included in the headers, and total headers in responses are limited to 8 KB (8,192 bytes). Ensure that errors do not exceed 1-2 KB in size.
- Error Schema:

```proto
// This message defines the error schema for Google's JSON HTTP APIs.
message Error {
  // Deprecated. This message is only used by error format v1.
  message ErrorProto {}
  // This message has the same semantics as `google.rpc.Status`. It uses HTTP
  // status code instead of gRPC status code. It has extra fields `status` and
  // `errors` for backward compatibility with [Google API Client
  // Libraries](https://developers.google.com/api-client-library).
  message Status {
    // The HTTP status code that corresponds to `google.rpc.Status.code`.
    int32 code = 1;
    // This corresponds to `google.rpc.Status.message`.
    string message = 2;
    // Deprecated. This field is only used by error format v1.
    repeated ErrorProto errors = 3;
    // This is the enum version for `google.rpc.Status.code`.
    google.rpc.Code status = 4;
    // This corresponds to `google.rpc.Status.details`.
    repeated google.protobuf.Any details = 5;
  }
  // The actual error payload. The nested message structure is for backward
  // compatibility with [Google API Client
  // Libraries](https://developers.google.com/api-client-library). It also
  // makes the error more readable to developers.
  Status error = 1;
}
```

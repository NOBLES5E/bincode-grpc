use futures::{FutureExt, TryFutureExt};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Input {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Output {}

#[bincode_grpc::service]
trait TestService {
    fn rpc_method1(&mut self, input: Input) -> Output;
    fn rpc_method2(&mut self, input: Input) -> Output;
    fn rpc_method3(&mut self, forward_id: u64, forward_only: bool) -> Result<Output, ()>;
}

#[derive(Clone)]
struct TestServer;

#[bincode_grpc::server]
impl TestService for TestServer {
    fn rpc_method1(&mut self, input: Input) -> Output {
        Output {}
    }

    fn rpc_method2(&mut self, input: Input) -> Output {
        Output {}
    }

    fn rpc_method3(&mut self, forward_id: u64, forward_only: bool) -> Result<Output, ()> { Ok(Output {}) }
}

fn main() {
    // start server
    let service = create_test_service(TestServer {});
    let env = std::sync::Arc::new(bincode_grpc::grpcio::Environment::new(8));
    let channel_builder = bincode_grpc::grpcio::ChannelBuilder::new(env.clone());
    let mut server = bincode_grpc::grpcio::ServerBuilder::new(env)
        .register_service(service)
        .bind("0.0.0.0", 9999)
        .channel_args(channel_builder.build_args())
        .build().unwrap();
    server.start();

    // client
    let env = std::sync::Arc::new(bincode_grpc::grpcio::Environment::new(8));
    let client_channel = bincode_grpc::grpcio::ChannelBuilder::new(env)
        .connect("127.0.0.1:9999");
    let client = TestServiceClient::new(client_channel);
    println!("{:?}", client.rpc_method1(&(Input {},)));
}

// The following are for other testing purposes.

#[bincode_grpc::service]
trait TestService3 {
    fn rpc_method4(&mut self) -> Output;
}

#[bincode_grpc::service]
pub trait TestService2 {
    fn rpc_method1(&mut self, input: Input) -> Output;
    fn rpc_method2(&mut self, input: Input) -> Output;
}

#[bincode_grpc::server]
impl TestService3 for TestServer {
    fn rpc_method4(&mut self) -> Output {
        Output {}
    }
}


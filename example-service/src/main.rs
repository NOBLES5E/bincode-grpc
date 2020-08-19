use futures::{FutureExt, TryFutureExt};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Input {}

#[derive(Serialize, Deserialize)]
pub struct Output {}

#[bincode_grpc::service]
trait TestService {
    fn rpc_method1(&mut self, input: Input) -> Output;
    fn rpc_method2(&mut self, input: Input) -> Output;
}

#[bincode_grpc::service]
trait TestService3 {
    fn rpc_method4(&mut self) -> Output;
}

#[bincode_grpc::service]
pub trait TestService2 {
    fn rpc_method1(&mut self, input: Input) -> Output;
    fn rpc_method2(&mut self, input: Input) -> Output;
}

struct TestServer;

#[bincode_grpc::server]
impl TestService3 for TestServer {
    fn rpc_method4(&mut self) -> Output {
        Output {}
    }
}

#[bincode_grpc::server]
impl TestService for TestServer {
    fn rpc_method1(&mut self, input: Input) -> Output {
        Output {}
    }

    fn rpc_method2(&mut self, input: Input) -> Output {
        Output {}
    }
}

fn main() {
    println!("Hello, world!");
}

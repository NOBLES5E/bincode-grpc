extern crate self as bincode_grpc;

pub mod bi_codec {
    use bytes::buf::{BufMut, BufMutExt};
    use grpcio::MessageReader;
    use grpcio::Result;
    use serde::de::DeserializeOwned;
    use serde::Serialize;

    #[inline]
    pub fn ser<M: Serialize, B: BufMut>(msg: &M, buf: &mut B) {
        bincode::serialize_into(buf.writer(), msg).expect("Writing message to buffer failed");
    }

    #[inline]
    pub fn de<M: DeserializeOwned>(reader: MessageReader) -> Result<M> {
        Ok(bincode::deserialize_from(reader).expect("Reading message from buffer failed"))
    }
}

pub use ::grpcio;
pub use bincode_grpc_macro::{server, service};

mod test {
    use super::{server, service};
    use futures::{FutureExt, TryFutureExt};
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize)]
    pub struct Input {}

    #[derive(Serialize, Deserialize)]
    pub struct Output {}

    #[service]
    trait TestService {
        fn rpc_method1(&mut self, input: Input) -> Output;
        fn rpc_method2(&mut self, input: Input) -> Output;
    }

    #[service]
    trait TestService3 {
        fn rpc_method1(&mut self) -> Output;
    }

    #[service]
    pub trait TestService2 {
        fn rpc_method1(&mut self, input: Input) -> Output;
        fn rpc_method2(&mut self, input: Input) -> Output;
    }

    struct TestServer;

    #[server]
    impl TestService for TestServer {
        fn rpc_method1(&mut self, input: Input) -> Output {
            Output {}
        }

        fn rpc_method2(&mut self, input: Input) -> Output {
            Output {}
        }
    }
}

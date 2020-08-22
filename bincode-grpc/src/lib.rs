pub extern crate grpcio;
extern crate self as bincode_grpc;
pub extern crate tracing;

pub mod bi_codec {
    use bytes::buf::BufMutExt;
    use grpcio::MessageReader;
    use grpcio::Result;
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use std::io::{BufReader};

    pub fn ser<M: Serialize>(msg: &M, buf: &mut Vec<u8>) {
        let serialized = bincode::serialize(msg).expect("serialize message failed");
        assert_eq!(std::mem::replace(buf, serialized).len(), 0);
    }

    pub fn de<M: DeserializeOwned>(reader: MessageReader) -> Result<M> {
        Ok(bincode::deserialize_from(BufReader::with_capacity(1024 * 512, reader)).expect("Reading message from buffer failed"))
    }
}

pub use bincode_grpc_macro::{server, service};

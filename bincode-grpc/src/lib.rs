pub extern crate grpcio;
extern crate self as bincode_grpc;
pub extern crate tracing;

pub mod bi_codec {
    use grpcio::MessageReader;
    use grpcio::Result;
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use std::io::{BufReader, Read};

    pub fn ser<M: Serialize>(msg: &M, buf: &mut Vec<u8>) {
        let serialized = bincode::serialize(msg).expect("serialize message failed");
        assert_eq!(std::mem::replace(buf, serialized).len(), 0);
    }

    pub fn de<M: DeserializeOwned>(mut reader: MessageReader) -> Result<M> {
        let mut buf = Vec::with_capacity(reader.len());
        reader.read_to_end(&mut buf).expect("Reading message from buffer failed");
        Ok(bincode::deserialize(&mut buf).expect("Deserializing message from buffer failed"))
    }
}

pub use bincode_grpc_macro::{server, service};

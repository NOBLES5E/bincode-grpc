#![feature(type_name_of_val)]
pub extern crate grpcio;
extern crate self as bincode_grpc;
pub extern crate tracing;

pub mod bi_codec {
    use grpcio::MessageReader;
    use grpcio::Result;
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use std::io::{BufReader, Read};
    use std::time::Instant;

    pub fn ser<M: Serialize>(msg: &M, buf: &mut Vec<u8>) {
        let span = tracing::span!(tracing::Level::DEBUG, "serialize");
        let _guard = span.enter();
        let start_time = Instant::now();
        let serialized = bincode::serialize(msg).expect("serialize message failed");
        assert_eq!(std::mem::replace(buf, serialized).len(), 0);
        tracing::debug!("serialize {:?} time cost {:?} on thread {:?}", std::any::type_name_of_val(&msg), start_time.elapsed(), std::thread::current().id());
    }

    pub fn de<M: DeserializeOwned>(mut reader: MessageReader) -> Result<M> {
        let span = tracing::span!(tracing::Level::DEBUG, "deserialize");
        let _guard = span.enter();
        let start_time = Instant::now();
        let mut buf = Vec::with_capacity(reader.len());
        reader.read_to_end(&mut buf).expect("Reading message from buffer failed");
        let result = bincode::deserialize(&mut buf).expect("Deserializing message from buffer failed");
        tracing::debug!("deserialize {:?} time cost {:?} on thread {:?}", std::any::type_name_of_val(&result), start_time.elapsed(), std::thread::current().id());
        Ok(result)
    }
}

pub use bincode_grpc_macro::{server, service};

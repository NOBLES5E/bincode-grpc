pub extern crate grpcio;
extern crate self as bincode_grpc;
pub extern crate tracing;

pub mod bi_codec {
    use bytes::buf::{BufMut, BufMutExt};
    use grpcio::MessageReader;
    use grpcio::Result;
    use serde::de::DeserializeOwned;
    use serde::Serialize;
    use std::io::{BufReader, BufWriter};

    #[inline]
    pub fn ser<M: Serialize, B: BufMut>(msg: &M, buf: &mut B) {
        bincode::serialize_into(BufWriter::new(buf.writer()), msg).expect("Writing message to buffer failed");
    }

    #[inline]
    pub fn de<M: DeserializeOwned>(reader: MessageReader) -> Result<M> {
        Ok(bincode::deserialize_from(BufReader::new(reader)).expect("Reading message from buffer failed"))
    }
}

pub use bincode_grpc_macro::{server, service};

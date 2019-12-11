use bytes::{BufMut, BytesMut};
use std::io;
use tokio::codec::{Decoder, Encoder};

/// A simple `Codec` implementation that just ships bytes around.
///
/// This type is used for "framing" a TCP/UDP stream of bytes but it's really
/// just a convenient method for us to work with streams/sinks for now.
/// This'll just take any data read and interpret it as a "frame" and
/// conversely just shove data into the output location without looking at
/// it.
pub struct Bytes;

impl Decoder for Bytes {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn decode(&mut self, buf: &mut BytesMut) -> io::Result<Option<Vec<u8>>> {

        if buf.len() > 0 {
            let len = buf.len();
            Ok(Some(buf.split_to(len).into_iter().collect()))
        } else {
            Ok(None)
        }
    }
}

impl Encoder for Bytes {
    type Item = Vec<u8>;
    type Error = io::Error;

    fn encode(&mut self, data: Vec<u8>, buf: &mut BytesMut) -> io::Result<()> {
        buf.put(&data[..]);
        Ok(())
    }
}
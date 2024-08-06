use serde_json::{from_slice as from_json_slice, to_vec as to_json_vec};
use tokio::io;
use bytes::{BytesMut, BufMut, Buf};
use tokio_util::codec::{Decoder, Encoder};
use std::time::Instant;

mod message;
pub use message::{Message, Protocol, Compression};

pub struct MessageCodec;

impl Encoder<Message> for MessageCodec {
    type Error = io::Error;

    fn encode(&mut self, item: Message, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let start = Instant::now();
        let payload = item.encode_data()?;

        let metadata = to_json_vec(&item).unwrap();
        let metadata_len = metadata.len() as u64;
        let payload_len = payload.len() as u64;

        dst.put_u64_le(metadata_len);
        dst.extend_from_slice(&metadata);
        dst.put_u64_le(payload_len);
        dst.extend_from_slice(&payload);

        let duration = start.elapsed();
        println!("Packed message: length {} bytes, time taken: {:?}", metadata_len + payload_len, duration);
        Ok(())
    }
}

impl Decoder for MessageCodec {
    type Item = Message;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Message>, Self::Error> {
        let start = Instant::now();

        if src.len() < 16 {
            return Ok(None);
        }

        let metadata_len = {
            let mut length_buf = &src[..8];
            length_buf.get_u64_le() as usize
        };

        if src.len() < 8 + metadata_len + 8 {
            return Ok(None);
        }

        let payload_len = {
            let mut length_buf = &src[8 + metadata_len..16 + metadata_len];
            length_buf.get_u64_le() as usize
        };

        if src.len() < 16 + metadata_len + payload_len {
            return Ok(None);
        }

        src.advance(8);
        let metadata_buf = src.split_to(metadata_len);
        let message: Message = from_json_slice(&metadata_buf).unwrap();

        src.advance(8);
        let payload_buf = src.split_to(payload_len);

        let payload = Message::decode_data(&payload_buf, &message.compression, &message.payload)?;

        let duration = start.elapsed();
        println!("Unpacked message: length {} bytes, time taken: {:?}", metadata_len + payload_len, duration);
        Ok(Some(Message { compression: message.compression, payload: match message.payload {
            Protocol::Json(_) => Protocol::Json(payload),
        } }))
    }
}

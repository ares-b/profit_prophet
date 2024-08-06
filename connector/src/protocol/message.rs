use serde::{Deserialize, Serialize};
use serde_json::{Value as JsonValue, from_slice as from_json_slice, to_vec as to_json_vec};
use zstd::stream::{encode_all as zstd_encode_all, decode_all as zstd_decode_all};
use tokio::io;
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Protocol {
    Json(JsonValue),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Compression {
    None,
    Zstd(u32),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub compression: Compression,
    pub payload: Protocol,
}

impl Message {
    pub fn encode_data(&self) -> Result<Vec<u8>, io::Error> {
        let data = match &self.payload {
            Protocol::Json(value) => to_json_vec(value),
        }.map_err(|e| {
            println!("Failed to serialize data: {:?}", e);
            io::Error::new(io::ErrorKind::Other, e)
        })?;

        let compressed_data = match self.compression {
            Compression::None => data,
            Compression::Zstd(level) => {
                let start = Instant::now();
                let result = zstd_encode_all(&data[..], level as i32).unwrap();
                let duration = start.elapsed();
                println!("Compression time: {:?}", duration);
                result
            }
        };

        Ok(compressed_data)
    }

    pub fn decode_data(data: &[u8], compression: &Compression, protocol: &Protocol) -> Result<JsonValue, io::Error> {
        let decompressed_data = match compression {
            Compression::None => data.to_vec(),
            Compression::Zstd(_) => {
                let start = Instant::now();
                let result = zstd_decode_all(data).unwrap();
                let duration = start.elapsed();
                println!("Decompression time: {:?}", duration);
                result
            }
        };

        match protocol {
            Protocol::Json(_) => from_json_slice(&decompressed_data).map_err(|e| {
                println!("Failed to deserialize JSON: {:?}", e);
                io::Error::new(io::ErrorKind::Other, e)
            }),
        }
    }
}
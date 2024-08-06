use metaflow::MetaFlow;
use std::env;
use log::info;

use metaflow::transformation::{TransformationConfig, get_value, register};
use connector::{Message, Protocol};

pub fn select(transformation_config: &TransformationConfig, message: Message) -> Message {
    info!("Original Message: {:?}", message);

    let payload = message.payload;

    let mut data = serde_json::Map::new();
    for (key, source_config) in &transformation_config.message.data {
        let Protocol::Json(value) = get_value(source_config, &payload);
        data.insert(key.clone(), value);
    }

    let transformed_message = Message {
        compression: message.compression,
        payload: Protocol::Json(serde_json::Value::Object(data)),
    };

    info!("Transformed Message: {:?}", transformed_message.payload);

    transformed_message
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    register("select", select);
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <config_file>", args[0]);
        std::process::exit(1);
    }
    let config_path = &args[1];

    info!("Loading configuration from {}", config_path);

    let metaflow = MetaFlow::new(config_path)?;

    metaflow.start().await?;

    Ok(())
}
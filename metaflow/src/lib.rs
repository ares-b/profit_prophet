use crate::config::data_source::DataSourceConfig;
use crate::config::data_sink::DataSinkConfig;
use std::fs;
use std::env;
use regex::Regex;

use log::info;
use serde::Deserialize;

mod config;
pub use config::transformation;

#[derive(Debug, Deserialize, Clone)]
pub struct MetaFlow {
    data_sources: Option<Vec<DataSourceConfig>>,
    data_sinks: Option<Vec<DataSinkConfig>>,
}

impl MetaFlow {
    pub fn new(config_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_config(config_path)
    }

    fn load_config(file_path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let config_content = fs::read_to_string(file_path)?;
        let updated_content = Self::replace_env_vars(&config_content)?;
        let config: Self = serde_yaml::from_str(&updated_content)?;
        Ok(config)
    }

    fn replace_env_vars(content: &str) -> Result<String, Box<dyn std::error::Error>> {
        let re = Regex::new(r"\$\{([^}]+)\}")?;
        let result = re.replace_all(content, |caps: &regex::Captures| {
            let var_name = &caps[1];
            env::var(var_name).unwrap_or_else(|_| format!("${{{}}}", var_name))
        });
        Ok(result.into_owned())
    }
    
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {

        if let Some(data_sinks) = &self.data_sinks {
            if !data_sinks.is_empty() {
                info!("Starting data sinks...");
                for data_sink_config in data_sinks {
                    data_sink_config.start().await?;
                }
            }
        }

        if let Some(data_sources) = &self.data_sources {
            if !data_sources.is_empty() {
                info!("Starting data sources...");
                for data_source_config in data_sources {
                    data_source_config.start().await?;
                }
            }
        }

        Ok(())
    }
}

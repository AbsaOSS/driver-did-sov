/*
 * Copyright 2023 ABSA Group Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use ::config as configrs;
use anyhow::Context;
use serde::{de::DeserializeOwned, Deserialize, Deserializer};

#[derive(Debug)]
pub struct LogLevel(pub tracing::Level);

#[derive(Debug, Deserialize)]
pub struct WalletConfig {
    pub kdf: String,
    pub key: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct PoolConfig {
    pub name: String,
    pub network: String,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationConfig {
    pub log_level: LogLevel,
    pub port: u16,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub pool: PoolConfig,
    pub wallet: WalletConfig,
    pub application: ApplicationConfig,
}

impl<'de> Deserialize<'de> for LogLevel {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.as_str() {
            "error" => Ok(LogLevel(tracing::Level::ERROR)),
            "warn" => Ok(LogLevel(tracing::Level::WARN)),
            "info" => Ok(LogLevel(tracing::Level::INFO)),
            "debug" => Ok(LogLevel(tracing::Level::DEBUG)),
            "trace" => Ok(LogLevel(tracing::Level::TRACE)),
            _ => Err(serde::de::Error::custom(format!(
                "Invalid log level: {}",
                s
            ))),
        }
    }
}

fn load_config<T: DeserializeOwned>() -> Result<T, anyhow::Error> {
    let base_path = std::env::current_dir().context("Failed to determine the current directory")?;
    let configuration_directory = base_path.join("config");

    let config = match std::env::var("APP_CONFIG").ok() {
        Some(env) => {
            let environment_filename = format!("{}.toml", env.as_str());
            info!("Configuration will be loaded from {}", environment_filename);
            configrs::Config::builder()
                .add_source(configrs::File::from(
                    configuration_directory.join(&environment_filename),
                ))
                .build()?
        }
        None => {
            info!("Configuration will be loaded from environment variables");
            configrs::Config::builder()
                .add_source(configrs::Environment::default().separator("::"))
                .build()?
        }
    };

    config
        .try_deserialize::<T>()
        .context("Failed to deserialize configuration")
}

impl Config {
    pub fn new() -> Result<Self, anyhow::Error> {
        load_config()
    }
}

use anyhow::Result;
use config::{Config, Environment, File};
use serde::{Deserialize, Serialize};
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CacheSettings {
    // In seconds, default = 3600 (1 hour)
    #[serde(default = "default_cache_expiration")]
    pub expiration: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    // Configure logging level = debug
    #[serde(default = "default_log")]
    pub log: String,
    // Configure host
    #[serde(default = "default_host")]
    pub host: String,
    // Configure port
    #[serde(default = "default_port")]
    pub port: u16,
    // Configure region dir containing H3 indices
    pub region_dir: PathBuf,
    // Configure cache settings
    pub cache: CacheSettings,
}

pub fn default_cache_expiration() -> u64 {
    3600
}

pub fn default_host() -> String {
    "127.0.0.1".to_string()
}

pub fn default_port() -> u16 {
    3000
}

pub fn default_log() -> String {
    "hextree_api=debug".to_string()
}

impl Settings {
    pub fn new<P: AsRef<Path>>(path: Option<P>) -> Result<Self, config::ConfigError> {
        let mut builder = Config::builder();

        if let Some(file) = path {
            builder = builder
                .add_source(File::with_name(&file.as_ref().to_string_lossy()).required(false));
        }
        builder
            .add_source(Environment::with_prefix("hextree_api").separator("_"))
            .build()
            .and_then(|config| config.try_deserialize())
    }

    pub fn socket_addr(&self) -> Result<SocketAddr> {
        let ip: std::net::IpAddr = self.host.parse()?;
        let addr = SocketAddr::new(ip, self.port);
        Ok(addr)
    }
}

impl CacheSettings {
    pub fn expiration(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.expiration)
    }
}

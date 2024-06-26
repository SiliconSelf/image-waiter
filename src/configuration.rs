//! Contains global program configuration

use figment::{
    providers::{Format, Toml},
    Figment,
};
use once_cell::sync::Lazy;
use serde::Deserialize;

/// The single source of truth for all application configuration
pub(crate) static APP_CONFIG: Lazy<Config> = Lazy::new(Config::new);

/// Represents a program configuration
#[derive(Deserialize)]
pub(crate) struct Config {
    /// The path to the postgres database
    database_url: String,
    /// How large the bounded channel between image actor and web server should
    /// be
    channel_size: usize,
}

impl Config {
    /// Create a new program configuration from the environment
    pub(crate) fn new() -> Self {
        let config: Config = Figment::from(Toml::file("waiter_config.toml"))
            .extract()
            .expect("Failed to configure program");
        config
    }

    /// Get a reference to the internal `database_path`
    pub(crate) fn get_database_url(&self) -> &String {
        &self.database_url
    }

    /// Get a copy of the internal `channel_size`
    pub(crate) fn get_channel_size(&self) -> usize {
        self.channel_size
    }
}

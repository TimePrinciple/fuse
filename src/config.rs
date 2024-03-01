//! `config` mod contains structs needed by core to start.
//! The workflow of generation process of `ValidatedConfig`:
//! 1. `cli::Args` read from command line are filled to `Config`.
//! 2. `Config` are filled with contents read from **configuration file**, which
//!    means the presented command line fields will override the configuration.
//! 3. `ValidatedConfig` is generated from `Config`, with all necessary fields
//!    checked to be valid to get `core` to work.
//! Configuration preparation before `core` starts.
use std::{convert::From, path::PathBuf};

use anyhow::Result;

use crate::cli::Args;

/// Configurations are read from config files and then can be override by the
/// supplied fields from command line. This config is a super set of `Args` read
/// from cli.
#[derive(Debug)]
pub struct Config {
    /// Mount point
    mount_point: Option<PathBuf>,
    /// Cache directory: where the data pulled from remote actually resides
    cache_dir: Option<PathBuf>,
    /// Log directory
    log_dir: Option<PathBuf>,
    /// Mega server Host
    mega_host: Option<String>,
    /// Mega server Port
    mega_port: Option<u16>,
}

impl Config {
    fn validate_mount_point(&mut self) -> Result<PathBuf, ()> {
        if self.mount_point.is_some() && self.mount_point.as_ref().unwrap().is_dir() {
            Ok(self.mount_point.take().unwrap())
        } else {
            Err(())
        }
    }

    fn validate_cache_dir(&mut self) -> Result<PathBuf, ()> {
        if self.cache_dir.is_some() && self.cache_dir.as_ref().unwrap().is_dir() {
            Ok(self.cache_dir.take().unwrap())
        } else {
            Err(())
        }
    }

    fn validate_log_dir(&mut self) -> Result<PathBuf, ()> {
        if self.log_dir.is_some() && self.log_dir.as_ref().unwrap().is_dir() {
            Ok(self.log_dir.take().unwrap())
        } else {
            Err(())
        }
    }

    fn validate_mega_url(&mut self) -> Result<String, ()> {
        let host = self.mega_host.take().unwrap();
        let port = self.mega_port.unwrap();
        Ok(format!("{}:{}", host, port))
    }
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Config {
            mount_point: args.mount_point,
            cache_dir: args.cache_dir,
            log_dir: args.log_dir,
            mega_host: args.mega_host,
            mega_port: args.mega_port,
        }
    }
}

/// `ValidatedConfig` can only be generated from `Config`.
#[derive(Debug)]
pub struct ValidatedConfig {
    /// Mount point
    pub mount_point: PathBuf,
    /// Cache directory: where the data pulled from remote actually resides
    pub cache_dir: PathBuf,
    /// Log directory
    pub log_dir: PathBuf,
    /// Joined by Mega server URL and API version, must be dialed and then check
    /// its response to make sure the server's `object services` are ready
    pub server_url: String,
}

impl From<Config> for ValidatedConfig {
    fn from(args: Config) -> Self {
        let mut args = args;
        ValidatedConfig {
            mount_point: args.validate_mount_point().unwrap(),
            cache_dir: args.validate_cache_dir().unwrap(),
            log_dir: args.validate_log_dir().unwrap(),
            server_url: args.validate_mega_url().unwrap(),
        }
    }
}

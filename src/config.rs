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
    pub mount_point: Option<PathBuf>,
    /// Cache directory: where the data pulled from remote actually resides
    cache_dir: Option<PathBuf>,
    /// Log directory
    log_dir: Option<PathBuf>,
    /// Mega server URL
    mega_url: Option<String>,
}

impl From<Args> for Config {
    fn from(args: Args) -> Self {
        Config {
            mount_point: args.mount_point,
            cache_dir: args.cache_dir,
            log_dir: args.log_dir,
            mega_url: args.mega_url,
        }
    }
}

/// `ValidatedConfig` can only be generated from `Config`.
pub struct ValidatedConfig {
    /// Mount point
    mount_point: PathBuf,
    /// Cache directory: where the data pulled from remote actually resides
    cache_dir: PathBuf,
    /// Log directory
    log_dir: PathBuf,
    /// Joined by Mega server URL and API version, must be dialed and then check
    /// its response to make sure the server's `object services` are ready
    server_url: String,
}

impl ValidatedConfig {
    fn from_config(config: Config) {
        // ValidatedConfig {}
    }
    fn is_valid(&self) -> bool {
        self.mount_point.is_dir() && self.cache_dir.is_dir() && self.log_dir.is_dir()
    }
}

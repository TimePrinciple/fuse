//! `cli` mod is used to read and parse command line arguments. These arguments
//! are required by `config` mod to produce a valid configuration before `core`
//! starts.
use std::path::PathBuf;

use clap::Parser;

/// Command line parser definition
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Mount point
    #[arg(long)]
    pub mount_point: Option<PathBuf>,
    /// Cache directory: where the data pulled from remote actually resides
    #[arg(long, default_value = None/* TODO */)]
    pub cache_dir: Option<PathBuf>,
    /// Log directory
    #[arg(long, default_value = None/* TODO */)]
    pub log_dir: Option<PathBuf>,
    /// Mega server URL
    #[arg(long)]
    pub mega_url: Option<String>,
}

/// Export parse() to main
pub fn parse() -> Args {
    Args::parse()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parsing() {
        let input = "fuse --mount-point path/to/mount-point --cache-dir path/to/cache --log-dir path/to/log --mega-url http://mega.com";
        let input: Vec<&str> = input.split_whitespace().collect();
        let args = Args::parse_from(&input);
        assert_eq!(args.mount_point.unwrap().as_os_str(), input[2]);
        assert_eq!(args.cache_dir.unwrap().as_os_str(), input[4]);
        assert_eq!(args.log_dir.unwrap().as_os_str(), input[6]);
        assert_eq!(args.mega_url.unwrap().as_str(), input[8]);
    }
}

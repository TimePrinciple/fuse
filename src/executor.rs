use tracing::info;

use crate::{cli::parse, config, core::mega_client};

/// Executor contains the actual logic of `mega-fuse`
/// The following steps are subjected to errors encountered, they will directly
/// abort on failure:
/// 1. Parse command line arguments, make sure the arguments supplied are at
///    least of the correct type.
/// 2. Construct a `Config` from `Args` parse from command line, validate the
///    arguments (e.g. directory specified are truly exist)
/// 3. Read configuration from configuration file.
/// 4. Validate `Config`, looking for missing but mandatory fields, return a
///    `ValidatedConfig` with fields in place (if the missing field could be
///    found in configuration file).
/// 5. Construct `MegaClient` upon `ValidatedConfig`.
/// 6. Mount the FS.
pub struct Executor {}

impl Executor {
    /// Executor entrance
    pub fn start() {
        // Initialize tracing subscriber
        tracing_subscriber::fmt().init();
        info!("Tracing subscriber initialized");

        // Parse command line arguments
        let cli = parse();
        info!("Command line arguments parsed: {:?}", cli);

        // Construct `Config` from `Args`
        let config = config::Config::from(cli);
        info!("`Config` generated from cli: {:?}", config);

        // Read configuration from configuration file
        // TODO: Currently skipped

        // Validate `Config`
        // The `ValidatedConfig::from()` method will use methods in `Config` to validate
        // all fields in `Config`
        let validated_config = config::ValidatedConfig::from(config);
        info!(
            "`Config` validated to `ValidatedConfig`: {:?}",
            validated_config
        );

        // Construct `MegaClient`
        let mut mega_client = mega_client::MegaClient::from_default_runtime(&validated_config)
            .expect("The MegaClient should be construct faultlessly with validated config");
        // If construction went successfully, the remote is alive at least this
        // moment, because the mega_client is running on a long held TcpStream
        info!(
            "MegaClient connection established to mage server at {}",
            &validated_config.server_url
        );

        // Mount FS
    }
}

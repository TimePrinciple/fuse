use fuse::{cli, config, core};
use fuser::{spawn_mount2, MountOption};
use tracing::info;

fn main() {
    // Initialize tracing subscriber
    tracing_subscriber::fmt().init();
    info!("Tracing subscriber initialized");

    let cli = cli::parse();
    info!("Command line arguments parsed: {:?}", cli);
    let config = config::Config::from(cli);
    info!("`Config` generated from cli: {:?}", config);

    let fs = core::FS {};
    let bs = spawn_mount2(
        fs,
        config.mount_point.unwrap(),
        &vec![MountOption::RO, MountOption::FSName("testFS".to_string())],
    )
    .unwrap();
    // TODO: use signal to terminal program and unmount file system
    info!("CTRL-C received, shutting down fuse");
    drop(bs);
    info!("Exitting");
}

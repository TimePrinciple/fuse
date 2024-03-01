use fuse::{cli, config, core};
use fuser::{spawn_mount2, MountOption};
use tracing::info;

fn main() {
    fuse::executor::Executor::start();
}

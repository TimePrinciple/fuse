// #![forbid(unsafe_code)]
#![deny(missing_docs)]
#![warn(clippy::all)]

//! A working FUSE filesystem consisis of three parts:
//! 1. The `kernel driver` that registers as a filesystem and forwards
//!    operations into a communication channel to a userspace process that
//!    handles them.
//! 2. The `userspace library` (libfuse) that helps the userspace process to
//!    establish and run communication with the kernel driver.
//! 3. The `userspace implemetation` that actually processes the file system
//!    operations.
//!
//! Modules of this project is listed below:

/// Command line parsing module
pub mod cli;
/// Configuration module
pub mod config;
/// Core logic
pub mod core;
/// Executor for cli module
pub mod executor;

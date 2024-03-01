mod inode;
/// MegaClient used to dial and communicate with remote mega server
pub mod mega_client;
mod request;
mod response;

use std::{
    collections::HashMap,
    sync::atomic::{AtomicU32, AtomicUsize, Ordering},
    time::{Duration, UNIX_EPOCH},
};

use fuser::{FileAttr, FileType, FUSE_ROOT_ID};
use libc::{ENOENT, ENONET};
use tracing::info;

use crate::core::mega_client::MegaClient;

/// Actually FUSE implementation
pub struct MegaFUSE {
    mega_client: MegaClient,
    // inodes: HashMap<usize, Inode>,
}

impl fuser::Filesystem for MegaFUSE {
    fn init(
        &mut self,
        _req: &fuser::Request<'_>,
        _config: &mut fuser::KernelConfig,
    ) -> Result<(), libc::c_int> {
        // Retrieve the basic `Tree` from the specified remote repository,
        // recursively initialize the directory

        // Request repo with `repo_name` specified to get the basic layout
        Ok(())
    }
    // fn getattr(&mut self, _req: &fuser::Request<'_>, ino: u64, reply:
    // fuser::ReplyAttr) {     info!("[getattr] called with ino: {}", ino);
    //     match ino {
    //         1 => reply.attr(&TTL, &SAMPLE_DIR_ATTR),
    //         2 => reply.attr(&TTL, &SAMPLE_FILE_ATTR),
    //         _ => reply.error(ENOENT),
    //     }
    // }

    // fn readdir(
    //     &mut self,
    //     _req: &fuser::Request<'_>,
    //     ino: u64,
    //     _fh: u64,
    //     offset: i64,
    //     mut reply: fuser::ReplyDirectory,
    // ) {
    //     info!("[readdir] called with ino: {}, offset: {}", ino, offset);
    //     if ino != 1 {
    //         reply.error(ENOENT);
    //         return;
    //     }

    //     let entries = vec![
    //         (1, FileType::Directory, "."),
    //         (1, FileType::Directory, ".."),
    //         (2, FileType::RegularFile, "sample-file"),
    //     ];

    //     for (i, entry) in entries.into_iter().enumerate().skip(offset as usize) {
    //         if reply.add(entry.0, (i + 1) as i64, entry.1, entry.2) {
    //             break;
    //         }
    //     }
    //     reply.ok();
    // }

    // fn lookup(
    //     &mut self,
    //     _req: &fuser::Request<'_>,
    //     parent: u64,
    //     name: &std::ffi::OsStr,
    //     reply: fuser::ReplyEntry,
    // ) {
    //     info!("[lookup] called with parent: {}, name: {:?}", parent, name);
    //     if parent == 1 && name.to_str() == Some("sample-file") {
    //         reply.entry(&TTL, &SAMPLE_FILE_ATTR, 0);
    //     } else {
    //         reply.error(ENOENT);
    //     }
    // }

    // fn read(
    //     &mut self,
    //     _req: &fuser::Request<'_>,
    //     ino: u64,
    //     _fh: u64,
    //     offset: i64,
    //     _size: u32,
    //     _flags: i32,
    //     _lock_owner: Option<u64>,
    //     reply: fuser::ReplyData,
    // ) {
    //     if ino == 2 {
    //         reply.data(&SAMPLE_FILE_CONTENT.as_bytes()[offset as usize..]);
    //     } else {
    //         reply.error(ENONET);
    //     }
    // }
}

#[cfg(test)]
mod tests {
    // use crate::core::SAMPLE_FILE_CONTENT;

    // #[test]
    // fn get_sample_length() {
    //     println!("{}", SAMPLE_FILE_CONTENT.len());
    // }
}

/// MegaClient used to dial and communicate with remote mega server
pub mod mega_client;
mod request;
mod response;

use std::time::{Duration, UNIX_EPOCH};

use fuser::{FileAttr, FileType};
use libc::{ENOENT, ENONET};
use tracing::info;

// const TTL: Duration = Duration::from_secs(1);
// const SAMPLE_DIR_ATTR: FileAttr = FileAttr {
//     ino: 1,
//     size: 0,
//     blocks: 0,
//     atime: UNIX_EPOCH, // 1970-01-01 00:00:00
//     mtime: UNIX_EPOCH,
//     ctime: UNIX_EPOCH,
//     crtime: UNIX_EPOCH,
//     kind: FileType::Directory,
//     perm: 0o755,
//     nlink: 2,
//     uid: 501,
//     gid: 20,
//     rdev: 0,
//     flags: 0,
//     blksize: 512,
// };
// const SAMPLE_FILE_CONTENT: &str = "This is a sample file from fuser rust\n";
// const SAMPLE_FILE_ATTR: FileAttr = FileAttr {
//     ino: 2,
//     size: 38,
//     blocks: 1,
//     atime: UNIX_EPOCH, // 1970-01-01 00:00:00
//     mtime: UNIX_EPOCH,
//     ctime: UNIX_EPOCH,
//     crtime: UNIX_EPOCH,
//     kind: FileType::RegularFile,
//     perm: 0o644,
//     nlink: 1,
//     uid: 501,
//     gid: 20,
//     rdev: 0,
//     flags: 0,
//     blksize: 512,
// };

/// Actually FUSE implementation
pub struct FS;

impl fuser::Filesystem for FS {
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

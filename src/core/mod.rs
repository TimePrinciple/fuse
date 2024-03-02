mod inode;
/// MegaClient used to dial and communicate with remote mega server
pub mod mega_client;
mod request;

use std::{
    borrow::BorrowMut,
    collections::{HashMap, LinkedList},
    fs::File,
    mem::replace,
    os::fd::AsRawFd,
    sync::{
        atomic::{AtomicU32, AtomicUsize, Ordering},
        Mutex,
    },
    time::{Duration, UNIX_EPOCH},
};

use clap::error;
use fuser::{consts::FOPEN_DIRECT_IO, FileAttr, FileType, FUSE_ROOT_ID};
use libc::{ENOENT, ENONET};
use tokio::io::join;
use tracing::{debug, error, info};

use crate::core::{
    inode::{ContentType, Inode, InodeAttributes},
    mega_client::MegaClient,
};

const TTL: Duration = Duration::from_secs(1); // 1 second
const MAX_NAME_LENGTH: u32 = 255;

/// Actually FUSE implementation
pub struct MegaFUSE {
    target_repo: String,
    mega_client: MegaClient,
    guard: Mutex<()>,
    inodes: HashMap<u64, Inode>,
}

impl MegaFUSE {
    /// Construct MegaFUSE using specified target repo and pre constructed
    /// MegaClient
    pub fn from(target_repo: String, mega_client: MegaClient) -> MegaFUSE {
        MegaFUSE {
            target_repo,
            mega_client,
            guard: Mutex::new(()),
            inodes: HashMap::<u64, Inode>::new(),
        }
    }

    /// lookup utility
    pub fn lookup_name(&self, parent: u64, name: &str) -> Option<u64> {
        let parent_inode = self.inodes.get(&parent).unwrap();
        for ino in parent_inode.children_ino.iter() {
            let inode = self.inodes.get(ino).unwrap();
            if inode.attr.name.eq(name) {
                return Some(*ino);
            }
        }
        None
    }
}

impl fuser::Filesystem for MegaFUSE {
    fn init(
        &mut self,
        _req: &fuser::Request<'_>,
        _config: &mut fuser::KernelConfig,
    ) -> Result<(), libc::c_int> {
        // Retrieve the basic `Tree` from the specified remote repository,
        // recursively initialize the directory
        info!(
            "Initialize filesystem of target {} repository",
            &self.target_repo
        );
        let guard = self.guard.lock().unwrap();
        self.inodes
            .insert(FUSE_ROOT_ID, Inode::root_node(&self.target_repo));
        // Used to iterate the objects level by level (request by request)
        let mut queue = LinkedList::from([FUSE_ROOT_ID]);
        while let Some(ino) = queue.pop_front() {
            let inode = self.inodes.get_mut(&ino).unwrap();
            let path = &inode.attr.path;

            let objects = match ino {
                // First time through, constructing basic tree
                FUSE_ROOT_ID => self.mega_client.request_base_tree(&self.target_repo),
                _ => {
                    // Request sub-directories
                    self.mega_client
                        .request_sub_tree_with_id(&self.target_repo, &inode.attr.id)
                }
            };
            let new_inodes: Vec<Inode> = objects
                .data
                .into_iter()
                .map(|object| {
                    // retrieve attributes from object
                    let attr = InodeAttributes::from(object);
                    let kind = attr.kind.clone();
                    let new_inode = Inode::new(ino, attr);
                    if kind == ContentType::Dir {
                        queue.push_back(new_inode.ino);
                    }
                    inode.insert_child(new_inode.ino);
                    new_inode
                })
                .collect();
            new_inodes.into_iter().for_each(|inode| {
                info!("Constructing {:?}", inode);
                self.inodes.insert(inode.ino, inode);
            });
        }
        drop(guard);
        info!("File system init success.");
        // Request repo with `repo_name` specified to get the basic layout
        Ok(())
    }

    fn getattr(&mut self, req: &fuser::Request<'_>, ino: u64, reply: fuser::ReplyAttr) {
        match self.inodes.get(&ino) {
            Some(inode) => {
                debug!("getattr(file at inode: {})", ino);
                reply.attr(&TTL, &inode.file_attr(req.uid(), req.gid()));
            }
            None => reply.error(ENOENT),
        }
    }

    fn readdir(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        mut reply: fuser::ReplyDirectory,
    ) {
        let inode = self.inodes.get(&ino).unwrap();
        let mut entries = vec![
            (ino, FileType::Directory, ".".to_owned()),
            (ino, FileType::Directory, "..".to_owned()),
        ];
        let children: Vec<(u64, FileType, String)> = inode
            .children_ino
            .iter()
            .map(|ino| self.inodes.get(ino).unwrap())
            .map(|inode| {
                (
                    inode.ino,
                    FileType::from(&inode.attr.kind),
                    inode.attr.name.clone(),
                )
            })
            .collect();
        entries.extend(children);

        for (index, (ino, kind, name)) in entries.into_iter().enumerate().skip(offset as usize) {
            if reply.add(ino, (index + 1) as i64, kind, name) {
                break;
            }
        }
        reply.ok();
    }

    fn lookup(
        &mut self,
        req: &fuser::Request<'_>,
        parent: u64,
        name: &std::ffi::OsStr,
        reply: fuser::ReplyEntry,
    ) {
        if name.len() > MAX_NAME_LENGTH as usize {
            reply.error(libc::ENAMETOOLONG);
            return;
        }
        let name = name.to_str().unwrap().to_owned();
        debug!("lookup({} at inode)", name);
        match self.lookup_name(parent, &name) {
            Some(ino) => {
                let inode = self.inodes.get(&ino).unwrap();
                reply.entry(&TTL, &inode.file_attr(req.uid(), req.gid()), 0)
            }
            None => reply.error(libc::ENOENT),
        }
    }

    fn open(&mut self, _req: &fuser::Request<'_>, ino: u64, _flags: i32, reply: fuser::ReplyOpen) {
        let inode = self.inodes.get_mut(&ino).unwrap();
        debug!("open({})", inode.attr.name);
        // Instantiate the file at inode `ino`

        let file_content = self
            .mega_client
            .request_file_content(&self.target_repo, &inode.attr.id);
        inode.attr.size = file_content.len() as u64;
        inode.content = Some(file_content);
        debug!("{:?}", inode);

        reply.opened(ino, FOPEN_DIRECT_IO);
    }

    fn flush(
        &mut self,
        _req: &fuser::Request<'_>,
        _ino: u64,
        _fh: u64,
        _lock_owner: u64,
        reply: fuser::ReplyEmpty,
    ) {
        reply.ok()
    }

    fn read(
        &mut self,
        _req: &fuser::Request<'_>,
        ino: u64,
        fh: u64,
        offset: i64,
        size: u32,
        flags: i32,
        lock_owner: Option<u64>,
        reply: fuser::ReplyData,
    ) {
        assert!(offset >= 0);
        // if (fh & ino) != 0 {
        //     reply.error(libc::EACCES);
        //     return;
        // }
        // Instantiate the file at inode `ino`
        let file_content = match self.inodes.get(&ino) {
            Some(inode) => {
                debug!("read({})", inode.attr.name);
                inode.content.as_ref().unwrap()
            }
            None => {
                reply.error(libc::ENOENT);
                error!("content at inode: {} not found, aborting", ino);
                return;
            }
        };

        reply.data(file_content[offset as usize..].as_bytes());
    }
}

#[cfg(test)]
mod tests {
    // use crate::core::SAMPLE_FILE_CONTENT;

    // #[test]
    // fn get_sample_length() {
    //     println!("{}", SAMPLE_FILE_CONTENT.len());
    // }
}

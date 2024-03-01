use std::{
    fs::File,
    path::PathBuf,
    sync::atomic::{AtomicU32, AtomicU64, Ordering},
    time::SystemTime,
};

use fuser::{FileAttr, FileType, FUSE_ROOT_ID};
use serde::Deserialize;

const BLOCK_SIZE: u32 = 4096;
const RDEV: u32 = 0;
const FLAGS: u32 = 0;
const DEFAULT_HARD_LINKS: u32 = 1;
const DEFAULT_PERMISSIONS: u16 = 600;
static GID: AtomicU32 = AtomicU32::new(1000);
static UID: AtomicU32 = AtomicU32::new(1000);
pub fn init_gu_id(gid: u32, uid: u32) {
    GID.store(gid, std::sync::atomic::Ordering::SeqCst);
    UID.store(uid, std::sync::atomic::Ordering::SeqCst);
}

pub fn gid() -> u32 {
    GID.load(std::sync::atomic::Ordering::Acquire)
}

pub fn uid() -> u32 {
    UID.load(std::sync::atomic::Ordering::Acquire)
}

#[derive(Debug, Deserialize)]
pub struct Object {
    id: String,
    name: String,
    path: PathBuf,
    content_type: ContentType,
    under_repo: bool,
    commit_msg: String,
    commit_date: String,
    commit_id: String,
}

#[derive(Debug, Deserialize)]
enum ContentType {
    #[serde(rename(deserialize = "file"))]
    File,
    #[serde(rename(deserialize = "directory"))]
    Dir,
}

#[derive(Debug, Deserialize)]
struct Objects {
    #[serde(rename(deserialize = "items"))]
    data: Vec<Objects>,
}
static INO_ALLOCATOR: AtomicU64 = AtomicU64::new(FUSE_ROOT_ID + 1);

fn alloc_ino() -> u64 {
    INO_ALLOCATOR.fetch_add(1, Ordering::SeqCst)
}
pub struct Inode {
    pub ino: u64,
    pub parent_ino: u64,
    pub children_ino: Vec<u64>,
    pub attr: InodeAttributes,
}

impl Inode {
    pub fn new(parent_ino: u64, attr: InodeAttributes) -> Self {
        // Register self to parent inode
        Self {
            ino: alloc_ino(),
            parent_ino,
            children_ino: Vec::new(),
            attr,
        }
    }

    pub fn insert_child(&mut self, child: u64) {
        self.children_ino.push(child);
    }
    pub fn remove_child(&mut self, child: u64) {
        let mut index = 0;
        for ele in self.children_ino.iter() {
            if *ele == child {
                break;
            }
            index += 1;
        }
        self.children_ino.remove(index);
    }

    pub fn file_attr(&self) -> FileAttr {
        let attrs = &self.attr;
        FileAttr {
            ino: self.ino,
            size: attrs.size,
            blocks: attrs.size / (BLOCK_SIZE as u64) + 1,
            atime: attrs.mtime,
            mtime: attrs.mtime,
            ctime: attrs.ctime,
            crtime: attrs.ctime,
            kind: attrs.kind.into(),
            perm: attrs.permissions,
            nlink: DEFAULT_HARD_LINKS,
            uid: uid(),
            gid: gid(),
            rdev: RDEV,
            blksize: BLOCK_SIZE,
            flags: FLAGS,
        }
    }
    pub fn root_node(fs_name: &str) -> Inode {
        let attr = InodeAttributes {
            id: fs_name.to_string(),
            size: BLOCK_SIZE as u64,
            name: fs_name.to_string(),
            path: "".to_owned(),
            kind: FileType::Directory,
            mtime: SystemTime::now(),
            ctime: SystemTime::now(),
            permissions: DEFAULT_PERMISSIONS,
        };
        Inode {
            ino: FUSE_ROOT_ID,
            parent_ino: FUSE_ROOT_ID,
            children_ino: Vec::new(),
            attr,
        }
    }
}

pub struct InodeAttributes {
    pub id: String,
    pub size: u64,
    pub name: String,
    pub kind: FileType,
    pub path: String,
    pub mtime: SystemTime,
    pub ctime: SystemTime,
    pub permissions: u16,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_tree() {
        let tree_response = r#"
{
   "items":[
      {
         "id":"d2c73088bc71e8b6ce07ec2e95087b57c42286d4",
         "name":"deny.toml",
         "path":"/projects/fuser/deny.toml",
         "content_type":"file",
         "under_repo":true,
         "commit_msg":"",
         "commit_date":"1701057603",
         "commit_id":"b6eb9ec1046d0e64adbcfdebe09d28eab43a94f9"
      },
      {
         "id":"75db394838a7ef17741a7cc5247763b9dea095b9",
         "name":".github",
         "path":"/projects/fuser/.github",
         "content_type":"directory",
         "under_repo":true,
         "commit_msg":"",
         "commit_date":"1701057603",
         "commit_id":"be820a8080f229301028546e819b4997af26cf47"
      }
   ]
}
"#;
        let objects: Objects = serde_json::from_str(tree_response).unwrap();
        dbg!(objects);
    }
}

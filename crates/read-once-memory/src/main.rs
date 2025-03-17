use fuser::{
    FileAttr, FileType, Filesystem, ReplyAttr, ReplyCreate, ReplyData, ReplyDirectory, ReplyEntry,
    ReplyStatfs, ReplyWrite, Request, TimeOrNow,
};
use libc::{EEXIST, ENOENT, ENOSPC};
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::create_dir_all;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

const TTL: Duration = Duration::from_secs(1);

#[derive(Clone)]
struct MemoryFile {
    data: Vec<u8>,
    attr: FileAttr,
}

struct ReadOnceFS {
    files: Arc<Mutex<HashMap<String, MemoryFile>>>,
    next_inode: u64,
}

impl ReadOnceFS {
    fn new() -> Self {
        Self {
            files: Arc::new(Mutex::new(HashMap::new())),
            next_inode: 2,
        }
    }
}

impl Filesystem for ReadOnceFS {
    fn lookup(&mut self, _req: &Request, parent: u64, name: &OsStr, reply: ReplyEntry) {
        if parent != 1 {
            reply.error(ENOENT);
            return;
        }

        let files = self.files.lock().unwrap();
        let path = name.to_str().unwrap();
        if let Some(file) = files.get(path) {
            reply.entry(&TTL, &file.attr, 0);
        } else {
            reply.error(ENOENT);
        }
    }

    fn create(
        &mut self,
        _req: &Request<'_>,
        parent: u64,
        name: &OsStr,
        mode: u32,
        umask: u32,
        flags: i32,
        reply: ReplyCreate,
    ) {
        if name.to_str().unwrap_or_default().starts_with(".") {
            reply.error(EEXIST);
            return;
        }
        if parent != 1 {
            reply.error(ENOENT);
            return;
        }

        let name = name.to_str().unwrap();
        let files = self.files.lock().unwrap();

        if files.contains_key(name) {
            reply.error(EEXIST);
            return;
        }

        self.next_inode += 1;

        let attr = FileAttr {
            ino: self.next_inode,
            size: 0,
            blocks: 0,
            atime: SystemTime::now(),
            mtime: SystemTime::now(),
            ctime: SystemTime::now(),
            crtime: SystemTime::now(),
            kind: FileType::RegularFile,
            perm: 0o777,
            nlink: 1,
            uid: 0,
            gid: 0,
            rdev: 0,
            flags: 0,
            blksize: 4096,
        };

        // files.insert(
        //     name.to_string(),
        //     MemoryFile {
        //         data: Vec::new(),
        //         attr,
        //     },
        // );

        reply.created(&TTL, &attr, 0, self.next_inode, 0);
    }

    fn write(
        &mut self,
        _req: &Request<'_>,
        _ino: u64,
        _fh: u64,
        _offset: i64,
        data: &[u8],
        _write_flags: u32,
        _flags: i32,
        _lock_owner: Option<u64>,
        reply: ReplyWrite,
    ) {
        reply.written(data.len() as u32);
    }

    fn setattr(
        &mut self,
        _req: &Request<'_>,
        ino: u64,
        _mode: Option<u32>,
        _uid: Option<u32>,
        _gid: Option<u32>,
        size: Option<u64>,
        atime: Option<TimeOrNow>,
        mtime: Option<TimeOrNow>,
        _ctime: Option<SystemTime>,
        _fh: Option<u64>,
        _crtime: Option<SystemTime>,
        _chgtime: Option<SystemTime>,
        _bkuptime: Option<SystemTime>,
        _flags: Option<u32>,
        reply: ReplyAttr,
    ) {
        let mut files = self.files.lock().unwrap();

        if let Some(file) = files.values_mut().find(|f| f.attr.ino == ino) {
            // Update the file size if necessary
            if let Some(new_size) = size {
                file.attr.size = new_size;
                file.data.resize(new_size as usize, 0);
            }

            // Update the access and modification times
            if let Some(new_atime) = atime {
                file.attr.atime = match new_atime {
                    TimeOrNow::SpecificTime(system_time) => system_time,
                    TimeOrNow::Now => SystemTime::now(),
                }
            }
            if let Some(new_mtime) = mtime {
                file.attr.mtime = match new_mtime {
                    TimeOrNow::SpecificTime(system_time) => system_time,
                    TimeOrNow::Now => SystemTime::now(),
                };
            }

            // Respond with updated file attributes
            reply.attr(&TTL, &file.attr);
        } else {
            reply.error(ENOENT);
        }
    }

    fn getattr(&mut self, _req: &Request<'_>, ino: u64, _fh: Option<u64>, reply: ReplyAttr) {
        if ino == 1 {
            // Root directory attributes
            let attr = FileAttr {
                ino: 1,
                size: 0,
                blocks: 0,
                atime: SystemTime::now(),
                mtime: SystemTime::now(),
                ctime: SystemTime::now(),
                crtime: SystemTime::now(),
                kind: FileType::Directory,
                perm: 0o777,
                nlink: 2,
                uid: 1000,
                gid: 1000,
                rdev: 0,
                flags: 0,
                blksize: 4096,
            };
            reply.attr(&TTL, &attr);
            return;
        }
        let files = self.files.lock().unwrap();
        if let Some(file) = files.values().find(|f| f.attr.ino == ino) {
            reply.attr(&TTL, &file.attr);
        } else {
            reply.error(ENOENT);
        }
    }

    fn read(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        size: u32,
        _flags: i32,
        _lock: Option<u64>,
        reply: ReplyData,
    ) {
        let mut files = self.files.lock().unwrap();
        if let Some(file) = files.values().find(|f| f.attr.ino == ino) {
            let data = &file.data;
            let offset = offset as usize;
            let size = size as usize;

            if offset >= data.len() {
                reply.data(&[]);
            } else {
                let end = std::cmp::min(offset + size, data.len());
                reply.data(&data[offset..end]);
            }

            // Delete the file after reading
            files.retain(|_, f| f.attr.ino != ino);
        } else {
            reply.error(ENOENT);
        }
    }

    fn readdir(
        &mut self,
        _req: &Request,
        ino: u64,
        _fh: u64,
        offset: i64,
        mut reply: ReplyDirectory,
    ) {
        if ino != 1 {
            reply.error(ENOENT);
            return;
        }

        let files = self.files.lock().unwrap();
        if offset == 0 {
            for (name, file) in files.iter() {
                let _ = reply.add(
                    file.attr.ino,
                    offset as i64 + 1,
                    FileType::RegularFile,
                    name,
                );
            }
        }
        reply.ok();
    }
}

fn main() {
    env_logger::init();
    let mut fs = ReadOnceFS::new();
    //fs.create_file("hello.txt", b"Hello, world!".to_vec());

    let mountpoint = "/tmp/memdisk-once";
    create_dir_all(mountpoint);
    fuser::mount2(fs, &mountpoint, &[]).unwrap();
}

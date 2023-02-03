use alloc::{string::String, vec::Vec};

type Inode = usize;

type read_fs = fn(&VFS_Node, usize, usize, &mut [u8]) -> Option<usize>;
type write_fs = fn(&mut VFS_Node, usize, usize, &[u8]) -> Option<usize>;
type readdir_fs = fn(&VFS_Node) -> Option<Vec<Inode>>;

#[derive(PartialEq, Debug)]
pub enum Type {
    File,
    Dir,
    CharDev,
    BlockDev,
    Mountpoint,
}

pub struct VFS_Node {
    pub name: String,
    pub kind: Type,
    pub inode: Inode,
    pub size: usize,
    pub read: Option<read_fs>,
    pub write: Option<write_fs>,
    pub readdir: Option<readdir_fs>,
}

impl VFS_Node {
    pub fn read(&self, offset: usize, size: usize, buffer: &mut [u8]) -> Option<usize> {
        if let Some(readfn) = self.read {
            return readfn(self, offset, size, buffer);
        }
        None
    }

    pub fn write(&mut self, offset: usize, size: usize, buffer: &[u8]) -> Option<usize> {
        if let Some(writefn) = self.write {
            return writefn(self, offset, size, buffer);
        }
        None
    }

    /// Returns FS indexes of nodes inside the directory
    pub fn readdir(&self) -> Option<Vec<usize>> {
        if let Some(readdirfn) = self.readdir {
            return readdirfn(self);
        }
        None
    }
}

pub struct Filesystem {
    pub nodes: Vec<VFS_Node>,
}

impl Filesystem {
    pub fn open(&self, pathname: &str) -> usize {
        self.nodes.iter().position(|n| n.name == pathname).unwrap()
    }
}

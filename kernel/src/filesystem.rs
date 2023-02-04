use crate::drivers::initrd::initialize_initrd;
use crate::logging;
use crate::{arch::addressing::KERNEL_BASE, multiboot::MultibootInfo};
use alloc::{string::String, vec::Vec};

static mut FS_root: Option<*mut VFS_Node> = None;

pub type Inode = usize;

type read_fs = fn(&VFS_Node, usize, usize, &mut [u8]) -> Option<usize>;
type write_fs = fn(&mut VFS_Node, usize, usize, &[u8]) -> Option<usize>;
type readdir_fs = fn(&VFS_Node) -> Option<Vec<DirEnt>>;
type finddir_fs = fn(&VFS_Node, name: &str) -> Option<*mut VFS_Node>;

#[derive(PartialEq, Debug, Clone)]
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
    pub finddir: Option<finddir_fs>,
    pub mount_point: Option<*mut VFS_Node>,
}

pub struct DirEnt {
    pub name: String,
    pub inode: Inode,
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
    pub fn readdir(&self) -> Option<Vec<DirEnt>> {
        let mut which = self;
        // Passthrough mounted directory if needed
        if let Some(mounted) = self.mount_point {
            which = unsafe { &*mounted };
        }

        if let Some(readdirfn) = which.readdir {
            return readdirfn(self);
        }
        None
    }

    /// Returns FS indexes of nodes inside the directory
    pub fn finddir(&self) -> Option<Vec<DirEnt>> {
        let mut which = self;
        // Passthrough mounted directory if needed
        if let Some(mounted) = self.mount_point {
            which = unsafe { &*mounted };
        }

        if let Some(finddir) = which.readdir {
            return finddir(self);
        }
        None
    }
}

pub fn initialize_fs(mb_info: &'static MultibootInfo) {
    //TODO: check if flag is set
    if mb_info.mods_count != 1 {
        panic!("GRUB modules not loaded");
    }
    let initrd_location;
    let size;
    unsafe {
        initrd_location = *((mb_info.mods_addr as u64 + KERNEL_BASE) as *const u32);
        let initrd_end = *((mb_info.mods_addr as u64 + KERNEL_BASE) as *const u32).add(1);
        size = initrd_end - initrd_location;

        log!("{} {} size: {}", initrd_location, initrd_end, size);
    }

    let root = initialize_initrd(initrd_location as u64, size as usize);

    unsafe {
        FS_root = Some(root);
    }
}

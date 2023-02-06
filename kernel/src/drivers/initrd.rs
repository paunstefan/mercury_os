use core::slice;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    arch::addressing::KERNEL_BASE,
    filesystem::{DirEnt, Type, VFS_Node},
};

use crate::logging;

static mut INIT_RD_FS: Option<InitRD> = None;

/// Filesystem header with the number of files
#[derive(Debug)]
pub struct Header {
    pub nfiles: u8,
}

/// InitRD file header.
/// Contains the name, size and location in the FS
#[derive(Debug, Clone)]
#[repr(C, packed(8))]
pub struct FileHeader {
    name: [u8; 64],
    pub size: usize,
    pub offset: usize,
}

impl FileHeader {
    /// The name in the structure is a null terminated array of bytes,
    /// this function converets it to a Rust string
    pub fn filename(&self) -> String {
        self.name
            .iter()
            .take_while(|b| **b != 0)
            .map(|b| *b as char)
            .collect()
    }
}

/// Initial RAMDisk filesystem structure
pub struct InitRD {
    address: *const u8,
    size: usize,
    header: Header,
    files: Vec<FileHeader>,
    root: VFS_Node,
    dev_dir: VFS_Node,
    file_nodes: Vec<VFS_Node>,
}

pub fn initialize_initrd(fs_location: u64, size: usize) -> *const VFS_Node {
    let address = fs_location + KERNEL_BASE;
    let location = unsafe { slice::from_raw_parts(address as *const u8, size) };
    let header = Header {
        nfiles: location[0],
    };

    let root = VFS_Node {
        name: "initrd".to_string(),
        kind: Type::Dir,
        inode: 0,
        size: 0,
        read: None,
        write: None,
        readdir: Some(readdir),
        finddir: Some(finddir),
        mount_point: None,
    };

    let dev_dir = VFS_Node {
        name: "dev".to_string(),
        kind: Type::Mountpoint,
        inode: 0,
        size: 0,
        read: None,
        write: None,
        readdir: None,
        finddir: None,
        mount_point: None,
    };

    let mut files = Vec::new();
    let mut file_nodes = Vec::new();

    for i in 0..header.nfiles {
        let file_header: FileHeader =
            unsafe { (*((address + 1) as *const FileHeader).add(i as usize)).clone() };

        let file_node = VFS_Node {
            name: file_header.filename(),
            kind: Type::File,
            inode: i as usize,
            size: file_header.size,
            read: Some(initrd_read),
            write: None,
            readdir: None,
            finddir: None,
            mount_point: None,
        };
        files.push(file_header);
        file_nodes.push(file_node);
    }

    log!("{:?}", files);

    let initrd_struct = InitRD {
        address: address as *const u8,
        size,
        header,
        files,
        root,
        dev_dir,
        file_nodes,
    };

    unsafe {
        INIT_RD_FS = Some(initrd_struct);
    }

    unsafe { &INIT_RD_FS.as_ref().unwrap().root as *const VFS_Node }
}

pub fn initrd_read(
    node: &VFS_Node,
    offset: usize,
    mut size: usize,
    buffer: &mut [u8],
) -> Option<usize> {
    if node.kind == Type::File {
        let fs = unsafe { &INIT_RD_FS.as_ref().unwrap() };
        let header = &fs.files[node.inode];
        let location = unsafe { slice::from_raw_parts(fs.address, fs.size) };

        if offset >= header.size {
            return None;
        }
        if offset + size > header.size {
            size = header.size - offset;
        }

        buffer[..size].copy_from_slice(&location[header.offset..(header.offset + size)]);
        return Some(size);
    }
    None
}

pub fn readdir(node: &VFS_Node) -> Option<Vec<DirEnt>> {
    if node.kind != Type::Dir {
        return None;
    }
    let mut ret = Vec::new();
    let fs = unsafe { &INIT_RD_FS.as_ref().unwrap() };

    ret.push(DirEnt {
        name: "dev".to_string(),
        inode: 0,
    });

    for node in &fs.file_nodes {
        ret.push(DirEnt {
            name: node.name.clone(),
            inode: node.inode,
        });
    }

    Some(ret)
}

pub fn finddir(node: &VFS_Node, name: &str) -> Option<*mut VFS_Node> {
    if node.kind != Type::Dir {
        return None;
    }
    let fs = unsafe { &mut INIT_RD_FS.as_mut().unwrap() };

    if name == "dev" {
        return Some(&mut fs.dev_dir);
    }

    fs.file_nodes
        .iter_mut()
        .find(|node| node.name == name)
        .map(|node| node as *mut VFS_Node)
}

use core::slice;

use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use crate::{
    arch::addressing::KERNEL_BASE,
    filesystem::{DirEnt, Inode, Type, VFS_Node},
};

static mut init_rd_fs: Option<InitRD> = None;

#[derive(Debug)]
pub struct Header {
    nfiles: u8,
}

#[derive(Debug)]
pub struct FileHeader {
    name: [u8; 64],
    offset: usize,
    size: usize,
}

pub struct InitRD {
    location: &'static [u8],
    header: Header,
    files: Vec<FileHeader>,
    root: VFS_Node,
    dev_dir: VFS_Node,
    file_nodes: Vec<VFS_Node>,
}

pub fn initialize_initrd(fs_location: u64, size: usize) -> *mut VFS_Node {
    let address = fs_location + KERNEL_BASE;
    let location = unsafe { slice::from_raw_parts(address as *const u8, size) };
    todo!()
}

pub fn initrd_read(
    node: &VFS_Node,
    offset: usize,
    mut size: usize,
    buffer: &mut [u8],
) -> Option<usize> {
    if node.kind == Type::File {
        let fs = unsafe { &init_rd_fs.as_ref().unwrap() };
        let header = &fs.files[node.inode];

        if offset >= header.size {
            return None;
        }
        if offset + size > header.size {
            size = header.size - offset;
        }

        buffer[..size].copy_from_slice(&fs.location[header.offset..(header.offset + size)]);
        return Some(size);
    }
    None
}

pub fn readdir(_node: &VFS_Node) -> Vec<DirEnt> {
    let mut ret = Vec::new();
    let fs = unsafe { &init_rd_fs.as_ref().unwrap() };

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

    ret
}

pub fn finddir(_node: &VFS_Node, name: &str) -> Option<*mut VFS_Node> {
    let fs = unsafe { &mut init_rd_fs.as_mut().unwrap() };

    if name == "dev" {
        return Some(&mut fs.dev_dir);
    }

    fs.file_nodes
        .iter_mut()
        .find(|node| node.name == name)
        .map(|node| node as *mut VFS_Node)
}

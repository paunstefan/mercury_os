use alloc::vec::Vec;

use crate::filesystem::VFS_Node;

#[derive(Debug)]
pub struct Header {
    nfiles: u64,
}

#[derive(Debug)]
pub struct FileHeader {
    name: [i8; 64],
    offset: u64,
    size: u64,
}

pub struct InitRD {
    location: &'static [u8],
    header: Header,
    files: Vec<FileHeader>,
    root: VFS_Node,
}

pub fn initrd_read(
    node: &VFS_Node,
    offset: usize,
    size: usize,
    buffer: &mut [u8],
) -> Option<usize> {
    todo!()
}

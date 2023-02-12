use crate::filesystem::{DirEnt, Type, VFS_Node};
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec::Vec;

use super::chardev::CharDev;

static mut DEV_FS: Option<DevFilesystem> = None;

pub struct DevFilesystem {
    root: VFS_Node,
    file_nodes: Vec<VFS_Node>,
    devices: Vec<Box<dyn CharDev>>,
}

pub fn initialize_devfs() -> *mut VFS_Node {
    let root = VFS_Node {
        name: "dev".to_string(),
        kind: Type::Dir,
        inode: 0,
        size: 0,
        read: None,
        write: None,
        readdir: Some(devfs_readdir),
        finddir: Some(devfs_finddir),
        mount_point: None,
    };
    let serial = VFS_Node {
        name: "serial".to_string(),
        kind: Type::CharDev,
        inode: 0,
        size: 0,
        read: Some(devfs_read),
        write: Some(devfs_write),
        readdir: None,
        finddir: None,
        mount_point: None,
    };

    let mut file_nodes = Vec::new();
    file_nodes.push(serial);

    let serial_dev = Box::new(super::serial::Serial);
    let mut devices: Vec<Box<dyn CharDev>> = Vec::new();
    devices.push(serial_dev);

    let dev_fs = DevFilesystem {
        root,
        file_nodes,
        devices,
    };

    unsafe {
        DEV_FS = Some(dev_fs);
    }

    unsafe { &mut DEV_FS.as_mut().unwrap().root as *mut VFS_Node }
}

pub fn devfs_read(
    node: &VFS_Node,
    _offset: usize,
    size: usize,
    buffer: &mut [u8],
) -> Option<usize> {
    let fs = unsafe { &DEV_FS.as_ref().unwrap() };
    let device = &fs.devices[node.inode];

    device.read(size, buffer)
}

pub fn devfs_write(
    node: &mut VFS_Node,
    _offset: usize,
    size: usize,
    buffer: &[u8],
) -> Option<usize> {
    let fs = unsafe { &mut DEV_FS.as_mut().unwrap() };
    let device = &mut fs.devices[node.inode];

    device.write(size, buffer)
}

pub fn devfs_readdir(node: &VFS_Node) -> Option<Vec<DirEnt>> {
    if node.kind != Type::Dir {
        return None;
    }
    let mut ret = Vec::new();
    let fs = unsafe { &DEV_FS.as_ref().unwrap() };

    for node in &fs.file_nodes {
        ret.push(DirEnt {
            name: node.name.clone(),
            inode: node.inode,
        });
    }

    Some(ret)
}

pub fn devfs_finddir(node: &VFS_Node, name: &str) -> Option<*mut VFS_Node> {
    if node.kind != Type::Dir {
        return None;
    }
    let fs = unsafe { &mut DEV_FS.as_mut().unwrap() };

    fs.file_nodes
        .iter_mut()
        .find(|node| node.name == name)
        .map(|node| node as *mut VFS_Node)
}

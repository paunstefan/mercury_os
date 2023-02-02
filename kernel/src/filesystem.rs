use alloc::vec::Vec;

type Inode = usize;

#[derive(Debug)]
pub struct VFS_Node<T> {
    pub name: [char; 64],
    pub typ: Type,
    pub inode: Inode,
    pub size: usize,
    pub driver: Driver,
    pub internal: T,
}

impl<T> File for VFS_Node<T>
where
    T: File,
{
    fn read(&self, offset: usize, size: usize, buffer: &mut [u8]) -> Option<usize> {
        self.internal.read(offset, size, buffer)
    }
    fn write(&mut self, offset: usize, size: usize, buffer: &[u8]) -> Option<usize> {
        self.internal.write(offset, size, buffer)
    }
    fn readdir(&self) -> Option<Vec<Inode>> {
        if self.typ == Type::Dir {
            todo!()
        } else {
            None
        }
    }
}

pub trait File {
    fn read(&self, offset: usize, size: usize, buffer: &mut [u8]) -> Option<usize>;
    fn write(&mut self, offset: usize, size: usize, buffer: &[u8]) -> Option<usize>;
    fn readdir(&self) -> Option<Vec<Inode>>;
}

#[derive(PartialEq, Debug)]
pub enum Type {
    File,
    Dir,
    CharDev,
    BlockDev,
    Mountpoint,
}

#[derive(PartialEq, Debug)]
pub enum Driver {
    InitRD,
    Dev,
}

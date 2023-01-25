/// A physical memory frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(C)]
pub struct PhysFrame {
    pub start_address: u64,
}

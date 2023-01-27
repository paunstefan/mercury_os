use core::fmt;

// TODO: table should actually be an union
#[derive(Clone, Copy, Debug)]
#[repr(C, packed(4))]
pub struct MultibootInfo {
    pub flags: u32,
    pub mem_lower: u32,
    pub mem_upper: u32,
    pub boot_device: u32,
    pub cmdline: u32,
    pub mods_count: u32,
    pub mods_addr: u32,
    pub table: ELF_Section_Header_Table,
    pub mmap_length: u32,
    pub mmap_addr: u32,
}

impl MultibootInfo {
    /// Read the Multiboot info using the address left in RBX
    /// Safety:
    /// The address given must point to a valid Multiboot structure
    pub const unsafe fn read(info_address: u64) -> &'static Self {
        //TODO: check magic numbers and flags
        &*((info_address + crate::arch::addressing::KERNEL_BASE) as *const Self) as _
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct ELF_Section_Header_Table {
    pub num: u32,
    pub size: u32,
    pub addr: u32,
    pub shndx: u32,
}

#[derive(Clone, Copy)]
#[repr(C, packed)]
pub struct MmapEntry {
    pub size: u32,
    pub addr: u64,
    pub len: u64,
    pub typ: u32,
}

#[derive(Debug)]
#[repr(u32)]
pub enum MmapType {
    AVAILABLE = 1,
    RESERVED = 2,
    OTHER,
}

impl From<u32> for MmapType {
    fn from(value: u32) -> Self {
        match value {
            1 => MmapType::AVAILABLE,
            2 => MmapType::RESERVED,
            _ => MmapType::OTHER,
        }
    }
}

#[allow(unaligned_references)]
impl fmt::Debug for MmapEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = f.debug_struct("MmapEntry");
        f.field("size", &format_args!("{}", self.size));
        f.field("addr", &format_args!("0x{:X}", self.addr));
        f.field("len", &format_args!("{}", self.len));
        f.field(
            "type",
            &format_args!("{:?} ({})", MmapType::from(self.typ), self.typ),
        );
        f.finish()
    }
}

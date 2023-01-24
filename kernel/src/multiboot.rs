// TODO: table should actually be an union
#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
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

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct ELF_Section_Header_Table {
    pub num: u32,
    pub size: u32,
    pub addr: u32,
    pub shndx: u32,
}

#[derive(Clone, Copy, Debug)]
#[repr(C, packed)]
pub struct MmapEntry {
    pub size: u32,
    pub addr: u64,
    pub len: u64,
    pub typ: u32,
}

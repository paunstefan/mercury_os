use core::fmt;

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
    pub drives_length: u32,
    pub drives_addr: u32,
    pub config_table: u32,
    pub boot_loader_name: u32,
    pub apm_table: u32,
    pub vbe_control_info: u32,
    pub vbe_mode_info: u32,
    pub vbe_mode: u16,
    pub vbe_interface_seg: u16,
    pub vbe_interface_off: u16,
    pub vbe_interface_len: u16,
    pub framebuffer: MultibootFramebuffer,
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
pub struct MultibootFramebuffer {
    pub addr: u64,
    pub pitch: u32,
    pub width: u32,
    pub height: u32,
    pub bpp: u8,
    pub typ: u8,
    pub reserved: u8,
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

#[derive(Debug)]
#[repr(u32)]
pub enum MmapType {
    Available = 1,
    Reserved = 2,
    Other,
}

impl From<u32> for MmapType {
    fn from(value: u32) -> Self {
        match value {
            1 => MmapType::Available,
            2 => MmapType::Reserved,
            _ => MmapType::Other,
        }
    }
}

// impl fmt::Debug for MmapEntry {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         let mut f = f.debug_struct("MmapEntry");
//         f.field("size", &format_args!("{}", self.size));
//         f.field("addr", &format_args!("0x{:X}", self.addr));
//         f.field("len", &format_args!("{}", self.len));
//         f.field(
//             "type",
//             &format_args!("{:?} ({})", MmapType::from(self.typ), self.typ),
//         );
//         f.finish()
//     }
// }

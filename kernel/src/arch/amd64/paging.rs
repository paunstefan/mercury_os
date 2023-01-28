use super::addressing::{PhysAddr, VirtAddr, KERNEL_BASE};
use crate::logging;
use crate::multiboot::{MmapEntry, MultibootInfo};
use crate::utils;
use core::{
    fmt,
    mem::size_of,
    ops::{Index, IndexMut},
};

// Symbol from linker script
// Can't be accessed as variable, but can as function pointer
extern "C" {
    fn kernel_end();
}

/// I use 2MB pages
pub const PAGE_SIZE: u64 = 2 * 1024 * 1024;

/// The PageAllocator creates new page table entries to
/// allocate virtual memory to the requesting process
#[derive(Debug)]
pub struct PageAllocator {}

/// The PageFrameAllocator keeps track of physical memory usage using
/// a bitmap and can mark memory as free or used when requested
#[derive(Debug)]
pub struct PageFrameAllocator {
    bitmap: *mut u8,
    multiboot_info: &'static MultibootInfo,
    total_pages: u64,
}

impl PageFrameAllocator {
    /// Initialize the Page Frame Allocator
    /// # Safety
    /// The Multiboot structure must have a valid Mmap pointer.
    /// Kernel_end must point to the end of the kernel allocated memory.
    pub unsafe fn init(multiboot_info: &'static MultibootInfo) -> Self {
        // Find out how much memory and create a bitmap of 2MB frames
        let mmap_iter = (0..(multiboot_info.mmap_length as usize / size_of::<MmapEntry>()))
            .map(|i| &*((multiboot_info.mmap_addr as u64 + KERNEL_BASE) as *const MmapEntry).add(i))
            .filter(|entry| entry.typ == 1 && entry.len >= PAGE_SIZE);

        let total_pages = mmap_iter.map(|entry| entry.len).sum::<u64>() / PAGE_SIZE;

        // I reserve 1 more if case the total pages are not divisible by 8
        let bitmap_len = ((total_pages / 8) + 1) as isize;

        let bitmap: *mut u8 = (kernel_end as usize + 8) as _;

        for i in 0..bitmap_len {
            *(bitmap.offset(i)) = 0;
        }

        let mut pfa = PageFrameAllocator {
            bitmap,
            multiboot_info,
            total_pages,
        };

        // For now this works, the first pages are the kernel ones, but they might not be
        pfa.set_bit(0);
        pfa.set_bit(1);

        pfa
    }

    /// Returns an iterator over all the avaiable fame start addresses
    unsafe fn frame_iterator(&self) -> impl Iterator<Item = Frame> {
        // Range of indexes into the Mmap array
        (0..(self.multiboot_info.mmap_length as usize / size_of::<MmapEntry>()))
            // Take references to the Mmap struct
            .map(|i| {
                &*((self.multiboot_info.mmap_addr as u64 + KERNEL_BASE) as *const MmapEntry).add(i)
            })
            // Filter only the available ones with a size at least big enough for a frame
            .filter(|entry| entry.typ == 1 && entry.len >= PAGE_SIZE)
            // Map each entry to a range between the start and end address
            // and align up the beginning to a frame size
            .map(|r| {
                utils::align_up(r.addr, PAGE_SIZE)..(utils::align_down(r.addr + r.len, PAGE_SIZE))
            })
            // Transform all ranges into ranges of multipes of PAGE_SIZE
            .flat_map(|r| r.step_by(PAGE_SIZE as usize))
            // Finally create a Frame struct starting at each address
            .map(|addr| Frame::containing_address(PhysAddr::new(addr)))
    }

    #[inline]
    unsafe fn set_bit(&mut self, index: usize) {
        let byte_index = index / 8;
        let bit_index = index % 8;

        if byte_index >= self.get_bitmap_len() {
            return;
        }

        *(self.bitmap.add(byte_index)) |= 1 << (8 - bit_index - 1);
    }

    #[inline]
    unsafe fn unset_bit(&mut self, index: usize) {
        let byte_index = index / 8;
        let bit_index = index % 8;

        if byte_index >= self.get_bitmap_len() {
            return;
        }

        *(self.bitmap.add(byte_index)) &= !(1 << (8 - bit_index - 1));
    }

    #[inline]
    fn is_bit_set(&self, index: usize) -> bool {
        let byte_index = index / 8;
        let bit_index = index % 8;

        if byte_index >= self.get_bitmap_len() {
            return false;
        }

        unsafe { (*(self.bitmap.add(byte_index)) & 1 << (8 - bit_index - 1)) != 0 }
    }

    #[inline]
    pub fn get_bitmap_len(&self) -> usize {
        ((self.total_pages / 8) + 1) as usize
    }

    /// Allocates the first free frame
    /// Marks its location with a 1 in the bitmap
    pub fn alloc_next(&mut self) -> Option<Frame> {
        // Iterate over the bitmap to find first free frame
        for byte in 0..self.get_bitmap_len() {
            for bit in 0..8_usize {
                let index = byte * 8 + bit;
                if !self.is_bit_set(index) {
                    unsafe { self.set_bit(index) };
                    let frame = unsafe {
                        self.frame_iterator()
                            .nth(index)
                            .expect("No frame at given index")
                    };
                    return Some(frame);
                }
            }
        }

        None
    }

    /// Frees the given frame
    pub fn free(&mut self, frame: Frame) {
        let frame_index = unsafe {
            self.frame_iterator()
                .position(|e| e == frame)
                .expect("Frame not allocable")
        };

        unsafe { self.unset_bit(frame_index) }
    }
}

/// A virtual memory page.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(C)]
pub struct Page {
    start_address: VirtAddr,
}

impl Page {
    /// The page size in bytes.
    pub const SIZE: u64 = 2 * 1024 * 1024;

    /// Returns the page that starts at the given virtual address.
    ///
    /// Returns an error if the address is not correctly aligned (i.e. is not a valid page start).
    #[inline]
    pub fn from_start_address(address: VirtAddr) -> Option<Self> {
        if !address.is_aligned(Self::SIZE) {
            return None;
        }
        Some(Page::containing_address(address))
    }

    /// Returns the page that contains the given virtual address.
    #[inline]
    pub fn containing_address(address: VirtAddr) -> Self {
        Page {
            start_address: address.align_down(Self::SIZE),
        }
    }
}

/// A physical memory frame.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[repr(C)]
pub struct Frame {
    pub start_address: PhysAddr,
}

impl Frame {
    /// The frame size in bytes.
    pub const SIZE: u64 = 2 * 1024 * 1024;

    /// Returns the frame that starts at the given physical address.
    ///
    /// Returns an error if the address is not correctly aligned (i.e. is not a valid frame start).
    #[inline]
    pub fn from_start_address(address: PhysAddr) -> Option<Self> {
        if !address.is_aligned(Self::SIZE) {
            return None;
        }
        Some(Frame::containing_address(address))
    }

    /// Returns the page that contains the given virtual address.
    #[inline]
    pub fn containing_address(address: PhysAddr) -> Self {
        Frame {
            start_address: address.align_down(Self::SIZE),
        }
    }
}

/// Returns a mutable reference to the active level 4 table.
///
/// # Safety
/// This function is unsafe because the caller must guarantee that the
/// complete physical memory is mapped to virtual memory at the passed
/// `physical_memory_offset`. Also, this function must be only called once
/// to avoid aliasing `&mut` references (which is undefined behavior).
pub unsafe fn active_level_4_table(physical_memory_offset: u64) -> &'static mut PageTable {
    let (level_4_table_frame, _) = super::registers::Cr3::read();

    let phys = level_4_table_frame.start_address;
    let virt = VirtAddr::new(physical_memory_offset + phys.as_u64());
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    &mut *page_table_ptr // unsafe
}

/// The number of entries in a page table.
const ENTRY_COUNT: usize = 512;

/// A 64-bit page table entry.
#[derive(Clone)]
#[repr(transparent)]
pub struct PageTableEntry {
    entry: u64,
}

impl PageTableEntry {
    /// Creates an unused page table entry.
    #[inline]
    pub const fn new() -> Self {
        PageTableEntry { entry: 0 }
    }

    /// Returns the flags of this entry.
    #[inline]
    pub const fn flags(&self) -> u64 {
        self.entry & 0xfff
    }

    /// Returns whether this entry is zero.
    #[inline]
    pub const fn is_unused(&self) -> bool {
        self.entry == 0
    }

    /// Sets this entry to zero.
    #[inline]
    pub fn set_unused(&mut self) {
        self.entry = 0;
    }

    /// Returns the physical address mapped by this entry, might be zero.
    #[inline]
    pub fn addr(&self) -> PhysAddr {
        PhysAddr::new(self.entry & 0x000f_ffff_ffff_f000)
    }

    /// Map the entry to the specified physical address with the specified flags.
    #[inline]
    pub fn set_addr(&mut self, addr: u64, flags: u64) {
        self.entry = addr | flags;
    }

    /// Sets the flags of this entry.
    #[inline]
    pub fn set_flags(&mut self, flags: u64) {
        self.entry = self.addr().as_u64() | flags;
    }
}

impl fmt::Debug for PageTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut f = f.debug_struct("PageTableEntry");
        f.field("addr", &format_args!("0x{:X}", self.addr().as_u64()));
        f.field("flags", &format_args!("0x{:X}", self.flags()));
        f.finish()
    }
}

#[allow(non_snake_case)]
pub mod PageTableFlags {
    /// Specifies whether the mapped frame or page table is loaded in memory.
    pub const PRESENT: u64 = 1;
    /// Controls whether writes to the mapped frames are allowed.
    ///
    /// If this bit is unset in a level 1 page table entry, the mapped frame is read-only.
    /// If this bit is unset in a higher level page table entry the complete range of mapped
    /// pages is read-only.
    pub const WRITABLE: u64 = 1 << 1;
    /// Controls whether accesses from userspace (i.e. ring 3) are permitted.
    pub const USER_ACCESSIBLE: u64 = 1 << 2;
    /// If this bit is set, a “write-through” policy is used for the cache, else a “write-back”
    /// policy is used.
    pub const WRITE_THROUGH: u64 = 1 << 3;
    /// Disables caching for the pointed entry is cacheable.
    pub const NO_CACHE: u64 = 1 << 4;
    /// Set by the CPU when the mapped frame or page table is accessed.
    pub const ACCESSED: u64 = 1 << 5;
    /// Set by the CPU on a write to the mapped frame.
    pub const DIRTY: u64 = 1 << 6;
    /// Specifies that the entry maps a huge frame instead of a page table. Only allowed in
    /// P2 or P3 tables.
    pub const HUGE_PAGE: u64 = 1 << 7;
    /// Indicates that the mapping is present in all address spaces, so it isn't flushed from
    /// the TLB on an address space switch.
    pub const GLOBAL: u64 = 1 << 8;
    /// Forbid code execution from the mapped frames.
    ///
    /// Can be only used when the no-execute page protection feature is enabled in the EFER
    /// register.
    pub const NO_EXECUTE: u64 = 1 << 63;
}

/// Represents a page table.
///
/// Always page-sized.
///
/// This struct implements the `Index` and `IndexMut` traits, so the entries can be accessed
/// through index operations. For example, `page_table[15]` returns the 15th page table entry.
#[repr(align(4096))]
#[repr(C)]
#[derive(Debug)]
pub struct PageTable {
    entries: [PageTableEntry; ENTRY_COUNT],
}

impl PageTable {
    /// Creates an empty page table.
    #[inline]
    pub const fn new() -> Self {
        const EMPTY: PageTableEntry = PageTableEntry::new();
        PageTable {
            entries: [EMPTY; ENTRY_COUNT],
        }
    }

    /// Returns an iterator over the entries of the page table.
    #[inline]
    pub fn iter(&self) -> impl Iterator<Item = &PageTableEntry> {
        self.entries.iter()
    }

    /// Returns an iterator that allows modifying the entries of the page table.
    #[inline]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut PageTableEntry> {
        self.entries.iter_mut()
    }
}

impl Index<usize> for PageTable {
    type Output = PageTableEntry;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl IndexMut<usize> for PageTable {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

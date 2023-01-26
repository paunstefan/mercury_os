use core::{
    fmt,
    ops::{Index, IndexMut},
};

use super::paging::PageTable;

pub const KERNEL_BASE: u64 = 0xFFFFFFFF80000000;

/// Utility function to wrap the `translate_address` method
/// by adding the hardcoded KERNEL_BASE address
pub fn translate_virtual_address(addr: VirtAddr) -> Option<PhysAddr> {
    addr.translate_address(KERNEL_BASE)
}
/// A canonical 64-bit virtual memory address.
///
/// On `x86_64`, only the 48 lower bits of a virtual address can be used. The top 16 bits need
/// to be copies of bit 47, i.e. the most significant bit. Addresses that fulfil this criterium
/// are called “canonical”. This type guarantees that it always represents a canonical address.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct VirtAddr(u64);

impl VirtAddr {
    pub fn translate_address(&self, physical_memory_offset: u64) -> Option<PhysAddr> {
        // read the active level 4 frame from the CR3 register
        let (level_4_table_frame, _) = super::registers::Cr3::read();

        let table_indexes = [self.p4_index(), self.p3_index(), self.p2_index()];
        let mut frame = level_4_table_frame.start_address();

        // traverse the multi-level page table
        for &index in &table_indexes {
            // convert the frame into a page table reference
            let virt = VirtAddr::new(physical_memory_offset + frame.as_u64());
            let table_ptr: *const PageTable = virt.as_ptr();
            let table = unsafe { &*table_ptr };

            // read the page table entry and update `frame`
            let entry = &table[index];
            frame = entry.addr();
        }

        // calculate the physical address by adding the page offset
        Some(PhysAddr(frame.as_u64() + self.page_offset()))
    }

    /// Creates a new canonical virtual address.
    ///
    /// This function performs sign extension of bit 47 to make the address canonical.
    ///
    /// ## Panics
    ///
    /// This function panics if the bits in the range 48 to 64 contain data (i.e. are not null and no sign extension).
    #[inline]
    pub fn new(addr: u64) -> VirtAddr {
        Self::try_new(addr).expect(
            "address passed to VirtAddr::new must not contain any data \
             in bits 48 to 64",
        )
    }

    /// Tries to create a new canonical virtual address.
    ///
    /// This function tries to performs sign
    /// extension of bit 47 to make the address canonical. It succeeds if bits 48 to 64 are
    /// either a correct sign extension (i.e. copies of bit 47) or all null. Else, an error
    /// is returned.
    #[inline]
    pub fn try_new(addr: u64) -> Option<VirtAddr> {
        match addr >> 47 {
            0 | 0x1ffff => Some(VirtAddr(addr)), // address is canonical
            1 => Some(VirtAddr(((addr << 16) as i64 >> 16) as u64)), // address needs sign extension
            _ => None,
        }
    }

    /// Converts the address to an `u64`.
    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    #[inline]
    pub fn from_ptr<T>(ptr: *const T) -> Self {
        Self::new(ptr as u64)
    }

    /// Converts the address to a raw pointer.
    #[inline]
    pub const fn as_ptr<T>(self) -> *const T {
        self.as_u64() as *const T
    }

    /// Converts the address to a mutable raw pointer.
    #[inline]
    pub const fn as_mut_ptr<T>(self) -> *mut T {
        self.as_ptr::<T>() as *mut T
    }

    /// Returns the 9-bit level 4 page table index.
    #[inline]
    pub const fn p4_index(self) -> usize {
        (self.0 >> 12 >> 9 >> 9 >> 9) as usize & 0b111111111
    }

    /// Returns the 9-bit level 3 page table index.
    #[inline]
    pub const fn p3_index(self) -> usize {
        (self.0 >> 12 >> 9 >> 9) as usize & 0b111111111
    }

    /// Returns the 9-bit level 2 page table index.
    #[inline]
    pub const fn p2_index(self) -> usize {
        (self.0 >> 12 >> 9) as usize & 0b111111111
    }

    /// Returns the 21-bit page offset of this virtual address.
    #[inline]
    pub const fn page_offset(self) -> u64 {
        self.0 & 0x1FFFFF
    }

    /// Checks whether the virtual address has the demanded alignment.
    #[inline]
    pub fn is_aligned(self, align: u64) -> bool {
        self.align_down(align) == self
    }

    /// Aligns the virtual address downwards to the given alignment.
    #[inline]
    pub fn align_down(self, align: u64) -> Self {
        VirtAddr::new(self.0 & !(align - 1))
    }
}

impl fmt::Debug for VirtAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("VirtAddr")
            .field(&format_args!("{:#X}", self.0))
            .finish()
    }
}

/// A 64-bit physical memory address.
///
/// On `x86_64`, only the 52 lower bits of a physical address can be used. The top 12 bits need
/// to be zero. This type guarantees that it always represents a valid physical address.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PhysAddr(u64);

impl fmt::Debug for PhysAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("PhysAddr")
            .field(&format_args!("{:#X}", self.0))
            .finish()
    }
}

impl PhysAddr {
    /// Creates a new physical address.
    ///
    /// ## Panics
    ///
    /// This function panics if a bit in the range 52 to 64 is set.
    #[inline]
    pub const fn new(addr: u64) -> Self {
        // TODO: Replace with .ok().expect(msg) when that works on stable.
        match Self::try_new(addr) {
            Some(p) => p,
            None => panic!("physical addresses must not have any bits in the range 52 to 64 set"),
        }
    }

    /// Creates a new physical address, throwing bits 52..64 away.
    #[inline]
    pub const fn new_truncate(addr: u64) -> PhysAddr {
        PhysAddr(addr % (1 << 52))
    }

    /// Tries to create a new physical address.
    ///
    /// Fails if any bits in the range 52 to 64 are set.
    #[inline]
    pub const fn try_new(addr: u64) -> Option<Self> {
        let p = Self::new_truncate(addr);
        if p.0 == addr {
            Some(p)
        } else {
            None
        }
    }

    /// Converts the address to an `u64`.
    #[inline]
    pub const fn as_u64(self) -> u64 {
        self.0
    }

    /// Checks whether the physical address has the demanded alignment.
    #[inline]
    pub fn is_aligned(self, align: u64) -> bool {
        self.align_down(align) == self
    }

    /// Aligns the virtual physical downwards to the given alignment.
    #[inline]
    pub fn align_down(self, align: u64) -> Self {
        PhysAddr::new(self.0 & !(align - 1))
    }
}

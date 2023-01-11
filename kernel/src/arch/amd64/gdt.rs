use crate::logging;
use core::{arch::asm, mem::size_of};

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// They are defined in `start.S`, will reuse them for the time being
extern "C" {
    static mut GDT: u64;
}

static mut TSS: TaskStateSegment = TaskStateSegment::new();

pub fn init_tss() {
    unsafe {
        TSS.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: u64 = 4096 * 3;
            static mut STACK: [u8; STACK_SIZE as usize] = [0; STACK_SIZE as usize];

            let stack_start = &STACK as *const _ as u64;
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };

        GlobalDescriptorTable::replace_tss(5, &TSS);
        GlobalDescriptorTable::load();
        GlobalDescriptorTable::load_tss(5);
    }
}

/// GDT structure, for the moment very hacky
/// it uses the global variables defined in the assembly file
/// TODO: rework this
struct GlobalDescriptorTable;

impl GlobalDescriptorTable {
    /// Replaces the memory at [offset] with a new TSS
    pub fn replace_tss(offset: u64, tss: &'static TaskStateSegment) {
        let (low, high) = tss_segment(tss);
        unsafe {
            let tss_addr = (&mut GDT as *mut u64).offset(offset as isize);

            *tss_addr = low;
            *(tss_addr.offset(1)) = high;
        }
    }

    /// Load the GDT using the lgdt instruction.
    /// # Safety
    /// GDTPtr should be a valid GDT structure and it must live for the lifetime of the kernel
    #[inline]
    pub unsafe fn load() {
        unsafe { asm!("lgdt GDTPtr", options(readonly, nostack, preserves_flags)) }
    }

    /// Load the TSS using the ltr instruction.
    /// Index is the index into the GDT structure (not offset)
    /// # Safety
    /// At the index there should be a valid TSS structure and it must live for the lifetime of the kernel
    #[inline]
    pub unsafe fn load_tss(index: u16) {
        let sel = index << 3;
        unsafe {
            asm!("ltr {0:x}", in(reg) sel, options(nostack, preserves_flags));
        }
    }
}

/// Create a new GDT entry that points to a TSS
pub fn tss_segment(tss: &'static TaskStateSegment) -> (u64, u64) {
    let ptr = tss as *const _ as u64;

    let access_byte: u8 = 0b10001001; // Present + 64bit TSS available

    let mut low = 0;

    // limit
    low |= ((size_of::<TaskStateSegment>() - 1) as u64) & 0xFFFF;
    // base
    low |= (ptr & 0xFFFFFF) << 16;
    low |= (ptr & 0xFF000000) << 32;

    // access
    low |= (access_byte as u64) << 40;

    let high = ptr >> 32;

    (low, high)
}

/// In 64-bit mode the TSS holds information that is not
/// directly related to the task-switch mechanism,
/// but is used for finding kernel level stack
/// if interrupts arrive while in kernel mode.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed(4))]
pub struct TaskStateSegment {
    reserved_1: u32,
    /// The full 64-bit canonical forms of the stack pointers (RSP) for privilege levels 0-2.
    pub privilege_stack_table: [u64; 3],
    reserved_2: u64,
    /// The full 64-bit canonical forms of the interrupt stack table (IST) pointers.
    pub interrupt_stack_table: [u64; 7],
    reserved_3: u64,
    reserved_4: u16,
    /// The 16-bit offset to the I/O permission bit map from the 64-bit TSS base.
    pub iomap_base: u16,
}

impl TaskStateSegment {
    /// Creates a new TSS with zeroed privilege and interrupt stack table and an
    /// empty I/O-Permission Bitmap.
    ///
    /// As we always set the TSS segment limit to
    /// `size_of::<TaskStateSegment>() - 1`, this means that `iomap_base` is
    /// initialized to `size_of::<TaskStateSegment>()`.
    #[inline]
    pub const fn new() -> TaskStateSegment {
        TaskStateSegment {
            privilege_stack_table: [0u64; 3],
            interrupt_stack_table: [0u64; 7],
            iomap_base: size_of::<TaskStateSegment>() as u16,
            reserved_1: 0,
            reserved_2: 0,
            reserved_3: 0,
            reserved_4: 0,
        }
    }
}

/// A struct describing a pointer to a descriptor table (GDT / IDT).
/// This is in a format suitable for giving to 'lgdt' or 'lidt'.
#[derive(Debug, Clone, Copy)]
#[repr(C, packed(2))]
pub struct DescriptorTablePointer {
    /// Size of the DT.
    pub limit: u16,
    /// Pointer to the memory region containing the DT.
    pub base: u64,
}

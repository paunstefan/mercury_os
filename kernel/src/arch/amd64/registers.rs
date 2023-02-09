#![allow(dead_code)]
use core::arch::asm;

use super::addressing::PhysAddr;

/// Page fault source address
pub struct Cr2;

impl Cr2 {
    pub fn read() -> u64 {
        let value: u64;

        unsafe {
            asm!("mov {}, cr2", out(reg) value, options(nomem, nostack, preserves_flags));
        }

        value
    }
}

/// Initial page table physical address
pub struct Cr3;

impl Cr3 {
    /// Read the current P4 table address from the CR3 register
    pub fn read() -> (PhysAddr, u16) {
        let value: u64;

        unsafe {
            asm!("mov {}, cr3", out(reg) value, options(nomem, nostack, preserves_flags));
        }

        let addr = value & 0x_000f_ffff_ffff_f000;
        let addr = PhysAddr::new(addr);
        (addr, (value & 0xFFF) as u16)
    }

    /// Write a new P4 table address into the CR3 register.
    ///
    /// ## Safety
    ///
    /// Changing the level 4 page table is unsafe, because it's possible to violate memory safety by
    /// changing the page mapping.
    #[inline]
    unsafe fn write_raw(frame: PhysAddr, val: u16) {
        let value = frame.as_u64() | val as u64;

        unsafe {
            asm!("mov cr3, {}", in(reg) value, options(nostack, preserves_flags));
        }
    }
}

pub struct Rflags;

impl Rflags {
    /// Returns the raw current value of the RFLAGS register.
    #[inline]
    pub fn read_raw() -> u64 {
        let r: u64;

        unsafe {
            asm!("pushfq; pop {}", out(reg) r, options(nomem, preserves_flags));
        }

        r
    }

    /// Writes the RFLAGS register.
    ///
    /// Does not preserve any bits, including reserved bits.
    ///
    ///
    /// ## Safety
    ///
    /// Unsafe because undefined becavior can occur if certain flags are modified. For example,
    /// the `DF` flag must be unset in all Rust code. Also, modifying `CF`, `PF`, or any other
    /// flags also used by Rust/LLVM can result in undefined behavior too.
    #[inline]
    pub unsafe fn write_raw(val: u64) {
        // HACK: we mark this function as preserves_flags to prevent Rust from restoring
        // saved flags after the "popf" below. See above note on safety.
        unsafe {
            asm!("push {}; popfq", in(reg) val, options(nomem, preserves_flags));
        }
    }
}

pub mod rflags_values {
    /// Processor feature identification flag.
    ///
    /// If this flag is modifiable, the CPU supports CPUID.
    pub const ID: u64 = 1 << 21;
    /// Indicates that an external, maskable interrupt is pending.
    ///
    /// Used when virtual-8086 mode extensions (CR4.VME) or protected-mode virtual
    /// interrupts (CR4.PVI) are activated.
    pub const VIRTUAL_INTERRUPT_PENDING: u64 = 1 << 20;
    /// Virtual image of the INTERRUPT_FLAG bit.
    ///
    /// Used when virtual-8086 mode extensions (CR4.VME) or protected-mode virtual
    /// interrupts (CR4.PVI) are activated.
    pub const VIRTUAL_INTERRUPT: u64 = 1 << 19;
    /// Enable automatic alignment checking if CR0.AM is set. Only works if CPL is 3.
    pub const ALIGNMENT_CHECK: u64 = 1 << 18;
    /// Enable the virtual-8086 mode.
    pub const VIRTUAL_8086_MODE: u64 = 1 << 17;
    /// Allows to restart an instruction following an instrucion breakpoint.
    pub const RESUME_FLAG: u64 = 1 << 16;
    /// Used by `iret` in hardware task switch mode to determine if current task is nested.
    pub const NESTED_TASK: u64 = 1 << 14;
    /// The high bit of the I/O Privilege Level field.
    ///
    /// Specifies the privilege level required for executing I/O address-space instructions.
    pub const IOPL_HIGH: u64 = 1 << 13;
    /// The low bit of the I/O Privilege Level field.
    ///
    /// Specifies the privilege level required for executing I/O address-space instructions.
    pub const IOPL_LOW: u64 = 1 << 12;
    /// Set by hardware to indicate that the sign bit of the result of the last signed integer
    /// operation differs from the source operands.
    pub const OVERFLOW_FLAG: u64 = 1 << 11;
    /// Determines the order in which strings are processed.
    pub const DIRECTION_FLAG: u64 = 1 << 10;
    /// Enable interrupts.
    pub const INTERRUPT_FLAG: u64 = 1 << 9;
    /// Enable single-step mode for debugging.
    pub const TRAP_FLAG: u64 = 1 << 8;
    /// Set by hardware if last arithmetic operation resulted in a negative value.
    pub const SIGN_FLAG: u64 = 1 << 7;
    /// Set by hardware if last arithmetic operation resulted in a zero value.
    pub const ZERO_FLAG: u64 = 1 << 6;
    /// Set by hardware if last arithmetic operation generated a carry ouf of bit 3 of the
    /// result.
    pub const AUXILIARY_CARRY_FLAG: u64 = 1 << 4;
    /// Set by hardware if last result has an even number of 1 bits (only for some operations).
    pub const PARITY_FLAG: u64 = 1 << 2;
    /// Set by hardware if last arithmetic operation generated a carry out of the
    /// most-significant bit of the result.
    pub const CARRY_FLAG: u64 = 1;
}

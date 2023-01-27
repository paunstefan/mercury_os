use core::arch::asm;

use super::{addressing::PhysAddr, paging::Frame};

/// Page fault source address
pub struct Cr2;

impl Cr2 {
    #[inline]
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
    #[inline]
    pub fn read() -> (Frame, u16) {
        let value: u64;

        unsafe {
            asm!("mov {}, cr3", out(reg) value, options(nomem, nostack, preserves_flags));
        }

        let addr = value & 0x_000f_ffff_ffff_f000;
        let frame = Frame {
            start_address: PhysAddr::new(addr),
        };
        (frame, (value & 0xFFF) as u16)
    }
}

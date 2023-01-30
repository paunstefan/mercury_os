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
}

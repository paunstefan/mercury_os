#![allow(non_snake_case)]
use crate::logging;
use core::{arch::asm, mem::size_of};

use super::{
    addressing::VirtAddr,
    registers::{rflags_values, Rflags},
};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

extern "C" {
    fn syscall_asm();
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Registers {
    pub rax: u64,
    pub rdi: u64,
    pub rsi: u64,
    pub rdx: u64,
    pub rcx: u64,
    pub r8: u64,
    pub r9: u64,
}

#[allow(clippy::fn_to_numeric_cast)]
pub fn init_idt() {
    unsafe {
        IDT.breakpoint.set_handler_fn(breakpoint_handler as u64);
        // IDT.breakpoint.options.set_IST(1);
        IDT.double_fault.set_handler_fn(double_fault_handler as u64);
        IDT.double_fault.options.set_IST(1);
        IDT.page_fault.set_handler_fn(page_fault_handler as u64);
        IDT.interrupts[InterruptIndex::Timer.IRQ_index()]
            .set_handler_fn(timer_interrupt_handler as u64);
        IDT.interrupts[InterruptIndex::Syscall.IRQ_index()].set_handler_fn(syscall_asm as u64);
        IDT.load();
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use super::pic::Timer::COUNT_DOWN;
    use super::pic::Timer::UPTIME;
    unsafe {
        let volatile = &mut COUNT_DOWN as *mut u64;
        let count = core::ptr::read_volatile(volatile);
        if count > 0 {
            core::ptr::write_volatile(volatile, count - 1);
        }

        let volatile = &mut UPTIME as *mut u64;
        let count = core::ptr::read_volatile(volatile);
        core::ptr::write_volatile(volatile, count + 1);

        crate::arch::pic::PICS
            .lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: u64,
) -> ! {
    log!(
        "EXCEPTION: DOUBLE FAULT error code: {}\n{:#?}",
        error_code,
        stack_frame
    );

    panic!();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    log!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn page_fault_handler(stack_frame: InterruptStackFrame, error_code: u64) {
    log!("EXCEPTION: PAGE FAULT");
    log!(
        "Accessed Address: 0x{:x}",
        crate::arch::registers::Cr2::read()
    );
    log!("Error Code: {}", error_code);
    log!("{:#?}", stack_frame);

    crate::hlt_loop()
}

/// Represents the interrupt stack frame pushed by the CPU on interrupt or exception entry.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct InterruptStackFrame {
    /// This value points to the instruction that should be executed when the interrupt
    /// handler returns. For most interrupts, this value points to the instruction immediately
    /// following the last executed instruction. However, for some exceptions (e.g., page faults),
    /// this value points to the faulting instruction, so that the instruction is restarted on
    /// return. See the documentation of the [`InterruptDescriptorTable`] fields for more details.
    pub instruction_pointer: u64,
    /// The code segment selector, padded with zeros.
    pub code_segment: u64,
    /// The flags register before the interrupt handler was invoked.
    pub cpu_flags: u64,
    /// The stack pointer at the time of the interrupt.
    pub stack_pointer: u64,
    /// The stack segment descriptor at the time of the interrupt (often zero in 64-bit mode).
    pub stack_segment: u64,
}
/// An Interrupt Descriptor Table entry.
///
/// The generic parameter can either be `HandlerFunc` or `HandlerFuncWithErrCode`, depending
/// on the interrupt vector.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Entry {
    pointer_low: u16,
    gdt_selector: u16,
    options: EntryOptions,
    pointer_middle: u16,
    pointer_high: u32,
    reserved: u32,
}

impl Entry {
    /// Creates a non-present IDT entry (but sets the must-be-one bits).
    pub const fn missing() -> Self {
        Entry {
            gdt_selector: 0,
            pointer_low: 0,
            pointer_middle: 0,
            pointer_high: 0,
            options: EntryOptions::minimal(),
            reserved: 0,
        }
    }

    /// Set the handler address for the IDT entry and sets the present bit.
    ///
    /// For the code selector field, this function uses the code segment selector currently
    /// active in the CPU.
    ///
    /// The function returns a mutable reference to the entry's options that allows
    /// further customization.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `addr` is the address of a valid interrupt handler function,
    /// and the signature of such a function is correct for the entry type.
    pub unsafe fn set_handler_fn(&mut self, addr: u64) -> &mut EntryOptions {
        self.pointer_low = addr as u16;
        self.pointer_middle = (addr >> 16) as u16;
        self.pointer_high = (addr >> 32) as u32;

        let cs: u16;
        asm!("mov {0:x}, cs", out(reg) cs, options(nomem, nostack, preserves_flags));

        self.gdt_selector = cs;

        self.options.set_present(true);
        &mut self.options
    }

    /// Returns the virtual address of this IDT entry's handler function.
    #[inline]
    pub fn handler_addr(&self) -> VirtAddr {
        VirtAddr::new(
            self.pointer_low as u64
                | (self.pointer_middle as u64) << 16
                | (self.pointer_high as u64) << 32,
        )
    }
}

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct EntryOptions(u16);

impl EntryOptions {
    /// Creates a minimal options field with the Gate Type bits set to Interrupt Gate
    #[inline]
    const fn minimal() -> Self {
        EntryOptions(0b1110_0000_0000)
    }

    /// Set or reset the present bit.
    #[inline]
    pub fn set_present(&mut self, present: bool) -> &mut Self {
        if present {
            self.0 |= 1 << 15;
        } else {
            self.0 &= !(1 << 15);
        }
        self
    }

    /// Set the required privilege level (DPL) for invoking the handler. The DPL can be 0, 1, 2,
    /// or 3, the default is 0. If CPL < DPL, a general protection fault occurs.
    #[inline]
    pub fn set_privilege_level(&mut self, dpl: u16) -> &mut Self {
        self.0 &= !(3 << 13);
        self.0 |= (dpl & 3) << 13;

        self
    }

    /// Let the CPU disable hardware interrupts when the handler is invoked. By default,
    /// interrupts are disabled on handler invocation.
    #[inline]
    pub fn disable_interrupts(&mut self, disable: bool) -> &mut Self {
        if disable {
            self.0 &= !(1 << 8);
        } else {
            self.0 |= 1 << 8;
        }

        self
    }

    /// A 3-bit value which is an offset into the Interrupt Stack Table,
    /// which is stored in the Task State Segment.
    /// If the bits are all set to zero, the Interrupt Stack Table is not used.
    #[inline]
    pub fn set_IST(&mut self, ist: u16) -> &mut Self {
        self.0 &= !7;
        self.0 |= ist & 7;

        self
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C, packed(2))]
pub struct DescriptorTablePointer {
    /// Size of the DT.
    pub limit: u16,
    /// Pointer to the memory region containing the DT.
    pub base: u64,
}

#[derive(Clone, Debug)]
#[repr(C)]
#[repr(align(16))]
pub struct InterruptDescriptorTable {
    pub divide_error: Entry,
    pub debug: Entry,
    pub non_maskable_interrupt: Entry,
    pub breakpoint: Entry,
    pub overflow: Entry,
    pub bound_range_exceeded: Entry,
    pub invalid_opcode: Entry,
    pub device_not_available: Entry,
    pub double_fault: Entry,
    pub coprocessor_segment_overrun: Entry,
    pub invalid_tss: Entry,
    pub segment_not_present: Entry,
    pub stack_segment_fault: Entry,
    pub general_protection_fault: Entry,
    pub page_fault: Entry,
    reserved_1: Entry,
    pub x87_floating_point: Entry,
    pub alignment_check: Entry,
    pub machine_check: Entry,
    pub simd_floating_point: Entry,
    pub virtualization: Entry,
    pub cp_protection_exception: Entry,
    reserved_2: [Entry; 6],
    pub hv_injection_exception: Entry,
    pub vmm_communication_exception: Entry,
    pub security_exception: Entry,
    reserved_3: Entry,
    pub interrupts: [Entry; 256 - 32],
}

impl InterruptDescriptorTable {
    /// Creates a new IDT filled with non-present entries.
    pub const fn new() -> InterruptDescriptorTable {
        InterruptDescriptorTable {
            divide_error: Entry::missing(),
            debug: Entry::missing(),
            non_maskable_interrupt: Entry::missing(),
            breakpoint: Entry::missing(),
            overflow: Entry::missing(),
            bound_range_exceeded: Entry::missing(),
            invalid_opcode: Entry::missing(),
            device_not_available: Entry::missing(),
            double_fault: Entry::missing(),
            coprocessor_segment_overrun: Entry::missing(),
            invalid_tss: Entry::missing(),
            segment_not_present: Entry::missing(),
            stack_segment_fault: Entry::missing(),
            general_protection_fault: Entry::missing(),
            page_fault: Entry::missing(),
            reserved_1: Entry::missing(),
            x87_floating_point: Entry::missing(),
            alignment_check: Entry::missing(),
            machine_check: Entry::missing(),
            simd_floating_point: Entry::missing(),
            virtualization: Entry::missing(),
            cp_protection_exception: Entry::missing(),
            reserved_2: [Entry::missing(); 6],
            hv_injection_exception: Entry::missing(),
            vmm_communication_exception: Entry::missing(),
            security_exception: Entry::missing(),
            reserved_3: Entry::missing(),
            interrupts: [Entry::missing(); 256 - 32],
        }
    }

    /// Load the IDT using the lidt instruction.
    /// # Safety
    /// Self should be a valid IDT structure and it must live for the lifetime of the kernel
    #[inline]
    pub unsafe fn load(&'static self) {
        let idtr = DescriptorTablePointer {
            base: self as *const _ as u64,
            limit: (size_of::<Self>() - 1) as u16,
        };
        asm!("lidt [{}]", in(reg) &idtr, options(readonly, nostack, preserves_flags))
    }
}

// Utility functions
#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = super::pic::PIC_1_OFFSET,
    Syscall = 0x80,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }

    pub fn IRQ_index(self) -> usize {
        self.as_usize() - 32
    }
}

/// Returns whether interrupts are enabled.
#[inline]
pub fn are_enabled() -> bool {
    Rflags::read_raw() & rflags_values::INTERRUPT_FLAG != 0
}

/// Enable interrupts.
///
/// This is a wrapper around the `sti` instruction.
#[inline]
pub fn enable() {
    unsafe {
        asm!("sti", options(nomem, nostack));
    }
}

/// Disable interrupts.
///
/// This is a wrapper around the `cli` instruction.
#[inline]
pub fn disable() {
    unsafe {
        asm!("cli", options(nomem, nostack));
    }
}

/// Run a closure with disabled interrupts.
///
/// Run the given closure, disabling interrupts before running it (if they aren't already disabled).
/// Afterwards, interrupts are enabling again if they were enabled before.
///
/// If you have other `enable` and `disable` calls _within_ the closure, things may not work as expected.
#[inline]
pub fn free<F, R>(f: F) -> R
where
    F: FnOnce() -> R,
{
    // true if the interrupt flag is set (i.e. interrupts are enabled)
    let saved_intpt_flag = are_enabled();

    // if interrupts are enabled, disable them for now
    if saved_intpt_flag {
        disable();
    }

    // do `f` while interrupts are disabled
    let ret = f();

    // re-enable interrupts if they were previously enabled
    if saved_intpt_flag {
        enable();
    }

    // return the result of `f` to the caller
    ret
}

/// Halts the CPU until the next interrupt arrives.
#[inline]
pub fn hlt() {
    unsafe {
        asm!("hlt", options(nomem, nostack, preserves_flags));
    }
}

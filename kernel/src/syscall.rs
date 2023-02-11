use crate::logging;

#[derive(Debug, Clone, Copy)]
#[repr(C, packed)]
pub struct Registers {
    rdi: u64,
    rsi: u64,
    rdx: u64,
    r10: u64,
    r8: u64,
    r9: u64,
}

#[no_mangle]
pub extern "C" fn syscall_handler(regs: Registers) {
    log!("{:?}", regs);
}

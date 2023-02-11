use crate::{arch::interrupts::Registers, logging};

#[no_mangle]
pub unsafe extern "C" fn syscall_handler(regs: Registers) {
    log!("{:?}", regs);

    let ret = match regs.rdi {
        0 => syscall_read(regs.rsi, regs.rdx, regs.rcx, regs.r8),
        1 => syscall_write(regs.rsi, regs.rdx, regs.rcx, regs.r8),
        2 => syscall_open(regs.rsi),
        3 => syscall_close(regs.rsi),
        4 => syscall_sleep(regs.rsi),
        5 => syscall_exit(),
        6 => syscall_getpid(),
        7 => syscall_uptime(),
        8 => syscall_exec(regs.rsi),
        9 => syscall_blit(regs.rsi),
        _ => 0,
    };

    log!("Syscall return: {}", ret);

    core::arch::asm!("mov rax, {}", in(reg) ret)
}

unsafe fn syscall_sleep(ms: u64) -> u64 {
    crate::arch::pic::Timer::sleep(ms);
    0
}

unsafe fn syscall_exit() -> u64 {
    todo!()
}

unsafe fn syscall_getpid() -> u64 {
    crate::task::MULTIPROCESSING.as_ref().unwrap().current_id - 1
}

unsafe fn syscall_uptime() -> u64 {
    todo!()
}

unsafe fn syscall_open(path_addr: u64) -> u64 {
    todo!()
}

unsafe fn syscall_close(fd: u64) -> u64 {
    todo!()
}

unsafe fn syscall_read(fd: u64, offset: u64, length: u64, buf_addr: u64) -> u64 {
    todo!()
}

unsafe fn syscall_write(fd: u64, offset: u64, length: u64, buf_addr: u64) -> u64 {
    todo!()
}

unsafe fn syscall_exec(path_addr: u64) -> u64 {
    todo!()
}

unsafe fn syscall_blit(address: u64) -> u64 {
    todo!()
}

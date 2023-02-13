use core::slice::{from_raw_parts, from_raw_parts_mut};

use crate::{arch::interrupts::Registers, logging, task::MULTIPROCESSING};

#[no_mangle]
pub unsafe extern "C" fn syscall_handler(regs: Registers) {
    //log!("{:?}", regs);

    let ret = match regs.rax {
        0 => syscall_read(regs.rdi, regs.rsi, regs.rdx),
        1 => syscall_write(regs.rdi, regs.rsi, regs.rdx),
        2 => syscall_open(regs.rdi),
        3 => syscall_close(regs.rdi),
        4 => syscall_sleep(regs.rdi),
        5 => syscall_exit(),
        6 => syscall_getpid(),
        7 => syscall_uptime(),
        8 => syscall_exec(regs.rdi),
        9 => syscall_blit(regs.rdi),
        _ => 0,
    };

    //log!("Syscall return: {}", ret);

    core::arch::asm!("mov rax, {}", in(reg) ret)
}

unsafe fn syscall_sleep(ms: u64) -> i64 {
    crate::arch::pic::Timer::sleep(ms);
    0
}

unsafe fn syscall_exit() -> i64 {
    todo!()
}

unsafe fn syscall_getpid() -> i64 {
    crate::task::MULTIPROCESSING.as_ref().unwrap().current_id as i64
}

unsafe fn syscall_uptime() -> i64 {
    todo!()
}

unsafe fn syscall_open(path_addr: u64) -> i64 {
    todo!()
}

unsafe fn syscall_close(fd: u64) -> i64 {
    todo!()
}

// TODO: implement seek (offset) in kernel
unsafe fn syscall_read(fd: u64, length: u64, buf_addr: u64) -> i64 {
    let mp_module = MULTIPROCESSING.as_mut().unwrap();
    let file_node = &mut *(mp_module.tasks[mp_module.current_id as usize].open_fd[fd as usize]);
    let slice = from_raw_parts_mut(buf_addr as *mut u8, length as usize);
    let ret = file_node.read(0, length as usize, slice);

    if let Some(read) = ret {
        read as i64
    } else {
        -1
    }
}

unsafe fn syscall_write(fd: u64, length: u64, buf_addr: u64) -> i64 {
    let mp_module = MULTIPROCESSING.as_mut().unwrap();
    let file_node = &mut *(mp_module.tasks[mp_module.current_id as usize].open_fd[fd as usize]);
    let slice = from_raw_parts(buf_addr as *const u8, length as usize);
    let ret = file_node.write(0, length as usize, slice);

    if let Some(wrote) = ret {
        wrote as i64
    } else {
        -1
    }
}

unsafe fn syscall_exec(path_addr: u64) -> i64 {
    todo!()
}

unsafe fn syscall_blit(address: u64) -> i64 {
    todo!()
}

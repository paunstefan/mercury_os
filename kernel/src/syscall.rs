use core::{
    slice::{from_raw_parts, from_raw_parts_mut},
    str::from_utf8_unchecked,
};

use crate::{
    arch::interrupts::Registers,
    drivers::framebuffer::FRAMEBUFFER,
    filesystem::{self, VFS_Node},
    task::MULTIPROCESSING,
};

#[no_mangle]
pub unsafe extern "C" fn syscall_handler(regs: Registers) {
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
        10 => syscall_fseek(regs.rdi, regs.rsi, regs.rdx),
        _ => 0,
    };

    core::arch::asm!("mov rax, {}", in(reg) ret)
}

unsafe fn syscall_sleep(ms: u64) -> i64 {
    crate::arch::pic::Timer::sleep(ms);
    0
}

unsafe fn syscall_exit() -> i64 {
    let mp_module = MULTIPROCESSING.as_mut().unwrap();

    mp_module.exit();

    0
}

unsafe fn syscall_getpid() -> i64 {
    crate::task::MULTIPROCESSING.as_ref().unwrap().current_id as i64
}

unsafe fn syscall_uptime() -> i64 {
    crate::arch::pic::Timer::UPTIME as i64
}

unsafe fn syscall_open(path_addr: u64) -> i64 {
    let path_ptr = path_addr as *const u8;
    let path_len = {
        let mut l = 0;
        while *path_ptr.add(l) != 0 {
            l += 1;
        }
        l
    };
    let path = from_utf8_unchecked(from_raw_parts(path_ptr, path_len));

    let mp_module = MULTIPROCESSING.as_mut().unwrap();

    match filesystem::fopen(path) {
        Some(file_ref) => {
            let open_fd = (file_ref as *mut VFS_Node, 0);
            let fd = mp_module.tasks[mp_module.current_id as usize].open_fd.len();
            mp_module.tasks[mp_module.current_id as usize]
                .open_fd
                .push(open_fd);
            fd as i64
        }
        None => -1,
    }
}

unsafe fn syscall_close(fd: u64) -> i64 {
    let mp_module = MULTIPROCESSING.as_mut().unwrap();
    mp_module.tasks[mp_module.current_id as usize]
        .open_fd
        .remove(fd as usize);
    fd as i64
}

unsafe fn syscall_read(fd: u64, length: u64, buf_addr: u64) -> i64 {
    let mp_module = MULTIPROCESSING.as_mut().unwrap();
    let (file_ptr, pos) = mp_module.tasks[mp_module.current_id as usize].open_fd[fd as usize];
    let slice = from_raw_parts_mut(buf_addr as *mut u8, length as usize);
    let file_node = &*file_ptr;
    let ret = file_node.read(pos as usize, length as usize, slice);

    if let Some(read) = ret {
        mp_module.tasks[mp_module.current_id as usize].open_fd[fd as usize].1 += read as u64;
        read as i64
    } else {
        -1
    }
}

unsafe fn syscall_write(fd: u64, length: u64, buf_addr: u64) -> i64 {
    let mp_module = MULTIPROCESSING.as_mut().unwrap();
    let (file_ptr, pos) = mp_module.tasks[mp_module.current_id as usize].open_fd[fd as usize];
    let slice = from_raw_parts_mut(buf_addr as *mut u8, length as usize);
    let file_node = &mut *file_ptr;
    let ret = file_node.write(pos as usize, length as usize, slice);

    if let Some(wrote) = ret {
        mp_module.tasks[mp_module.current_id as usize].open_fd[fd as usize].1 += wrote as u64;
        wrote as i64
    } else {
        -1
    }
}

unsafe fn syscall_fseek(fd: u64, offset: u64, whence: u64) -> i64 {
    let mp_module = MULTIPROCESSING.as_mut().unwrap();
    let file_size =
        { (*mp_module.tasks[mp_module.current_id as usize].open_fd[fd as usize].0).size };
    let pos = &mut mp_module.tasks[mp_module.current_id as usize].open_fd[fd as usize].1;

    match whence {
        0 => {
            *pos = offset;
            *pos as i64
        }
        1 => {
            *pos += offset;
            *pos as i64
        }
        2 => {
            *pos = file_size as u64 + offset;
            *pos as i64
        }
        _ => -1,
    }
}

unsafe fn syscall_exec(path_addr: u64) -> i64 {
    let path_ptr = path_addr as *const u8;
    let path_len = {
        let mut l = 0;
        while *path_ptr.add(l) != 0 {
            l += 1;
        }
        l
    };
    let path = from_utf8_unchecked(from_raw_parts(path_ptr, path_len));

    let mp_module = MULTIPROCESSING.as_mut().unwrap();

    mp_module.execute(path);

    0
}

unsafe fn syscall_blit(address: u64) -> i64 {
    let fb = FRAMEBUFFER.as_mut().unwrap();
    let slice = from_raw_parts_mut(address as *mut u32, fb.width * fb.height);
    fb.buffer.copy_from_slice(slice);

    0
}

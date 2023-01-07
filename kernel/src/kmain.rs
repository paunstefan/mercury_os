#![no_std]

#[panic_handler]
pub fn panic_implementation(_info: &::core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub fn kmain() {
    loop {}
}

// TODO: make it use actual Rust chars
// TODO: extend with options
pub trait SerialPort {
    fn put_char(c: u8);

    fn puts(s: &str) {
        for b in s.bytes() {
            Self::put_char(b);
        }
    }
}

pub struct Serial;

impl SerialPort for Serial {
    fn put_char(c: u8) {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            crate::arch::serial::putb(c)
        }
    }
}

use alloc::string::String;

use super::chardev::CharDev;

// TODO: extend with options
pub struct Serial;

impl Serial {
    pub fn put_char(c: u8) {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            crate::arch::serial::putb(c)
        }
    }

    pub fn puts(s: &str) {
        for b in s.bytes() {
            Self::put_char(b);
        }
    }

    pub fn get_char() -> u8 {
        unsafe {
            #[cfg(target_arch = "x86_64")]
            crate::arch::serial::getb()
        }
    }

    pub fn get_line() -> String {
        let mut ret = String::new();
        let mut c;
        loop {
            c = Self::get_char();

            if c == 13 {
                break;
            }
            ret.push(c as char);
        }
        ret
    }
}

impl CharDev for Serial {
    fn read(&self, size: usize, buf: &mut [u8]) -> Option<usize> {
        let mut index = 0_usize;
        while index < size {
            buf[index] = Serial::get_char();
            index += 1;
        }
        Some(index)
    }

    fn write(&mut self, size: usize, buf: &[u8]) -> Option<usize> {
        let mut index = 0_usize;
        while index < size {
            Serial::put_char(buf[index]);
            index += 1;
        }
        Some(index)
    }
}

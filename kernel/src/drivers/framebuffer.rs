use core::slice::from_raw_parts_mut;

use crate::arch::addressing::VirtAddr;

pub static mut FRAMEBUFFER: Option<Framebuffer> = None;

pub struct Framebuffer {
    pub buffer: &'static mut [u32],
    pub width: usize,
    pub height: usize,
    pub bpp: u8,
}

pub struct Rgb {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl Rgb {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Rgb { red, green, blue }
    }

    pub fn pack32(&self) -> u32 {
        self.blue as u32 + ((self.green as u32) << 8) + ((self.red as u32) << 16)
    }
}

impl Framebuffer {
    pub fn init(addr: VirtAddr, width: usize, height: usize, bpp: u8) {
        unsafe {
            let fb = Framebuffer {
                buffer: from_raw_parts_mut(addr.as_u64() as *mut u32, width * height),
                width,
                height,
                bpp,
            };

            FRAMEBUFFER = Some(fb);
        }
    }

    pub fn fill(&mut self, color: Rgb) {
        self.buffer[..].fill(color.pack32());
    }
}

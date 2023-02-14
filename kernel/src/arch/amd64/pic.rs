use crate::sync::SpinMutex;

use super::io::{inb, outb};

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;
/// Command sent to begin PIC initialization.
const CMD_INIT: u8 = 0x11;
/// Command sent to acknowledge an interrupt.
const CMD_END_OF_INTERRUPT: u8 = 0x20;
// The mode in which we want to run our PICs.
const MODE_8086: u8 = 0x01;

pub static PICS: SpinMutex<ChainedPics> =
    SpinMutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

/// An individual PIC chip.
struct Pic {
    /// The base offset to which our interrupts are mapped.
    offset: u8,
    /// The processor I/O port on which we send commands.
    command: u16,
    /// The processor I/O port on which we send and receive data.
    data: u16,
}

impl Pic {
    /// Are we in change of handling the specified interrupt?
    /// (Each PIC handles 8 interrupts.)
    fn handles_interrupt(&self, interupt_id: u8) -> bool {
        self.offset <= interupt_id && interupt_id < self.offset + 8
    }

    /// Notify us that an interrupt has been handled and that we're ready
    /// for more.
    unsafe fn end_of_interrupt(&mut self) {
        outb(self.command, CMD_END_OF_INTERRUPT);
    }

    /// Reads the interrupt mask of this PIC.
    unsafe fn read_mask(&mut self) -> u8 {
        inb(self.data)
    }

    /// Writes the interrupt mask of this PIC.
    unsafe fn write_mask(&mut self, mask: u8) {
        outb(self.data, mask);
    }
}

/// A pair of chained PIC controllers.  This is the standard setup on x86.
pub struct ChainedPics {
    pics: [Pic; 2],
}

impl ChainedPics {
    /// Create a new interface for the standard PIC1 and PIC2 controllers,
    /// specifying the desired interrupt offsets.
    pub const unsafe fn new(offset1: u8, offset2: u8) -> ChainedPics {
        ChainedPics {
            pics: [
                Pic {
                    offset: offset1,
                    command: 0x20,
                    data: 0x21,
                },
                Pic {
                    offset: offset2,
                    command: 0xA0,
                    data: 0xA1,
                },
            ],
        }
    }

    /// Initialize both our PICs.  We initialize them together, at the same
    /// time, because it's traditional to do so, and because I/O operations
    /// might not be instantaneous on older processors.
    pub unsafe fn initialize(&mut self) {
        // We need to add a delay between writes to our PICs, especially on
        // older motherboards.  But we don't necessarily have any kind of
        // timers yet, because most of them require interrupts.  Various
        // older versions of Linux and other PC operating systems have
        // worked around this by writing garbage data to port 0x80, which
        // allegedly takes long enough to make everything work on most
        // hardware.  Here, `wait` is a closure.
        let wait_port: u16 = 0x80;
        let wait = || outb(wait_port, 0);

        // Save our original interrupt masks, because I'm too lazy to
        // figure out reasonable values.  We'll restore these when we're
        // done.
        let saved_masks = self.read_masks();

        // Tell each PIC that we're going to send it a three-byte
        // initialization sequence on its data port.
        outb(self.pics[0].command, CMD_INIT);
        wait();
        outb(self.pics[1].command, CMD_INIT);
        wait();

        // Byte 1: Set up our base offsets.
        outb(self.pics[0].data, self.pics[0].offset);
        wait();
        outb(self.pics[1].data, self.pics[1].offset);
        wait();

        // Byte 2: Configure chaining between PIC1 and PIC2.
        outb(self.pics[0].data, 4);
        wait();
        outb(self.pics[1].data, 2);
        wait();

        // Byte 3: Set our mode.
        outb(self.pics[0].data, MODE_8086);
        wait();
        outb(self.pics[1].data, MODE_8086);
        wait();

        // Restore our saved masks.
        self.write_masks(saved_masks[0], saved_masks[1])
    }

    /// Reads the interrupt masks of both PICs.
    pub unsafe fn read_masks(&mut self) -> [u8; 2] {
        [self.pics[0].read_mask(), self.pics[1].read_mask()]
    }

    /// Writes the interrupt masks of both PICs.
    pub unsafe fn write_masks(&mut self, mask1: u8, mask2: u8) {
        self.pics[0].write_mask(mask1);
        self.pics[1].write_mask(mask2);
    }

    /// Disables both PICs by masking all interrupts.
    pub unsafe fn disable(&mut self) {
        self.write_masks(u8::MAX, u8::MAX)
    }

    /// Do we handle this interrupt?
    pub fn handles_interrupt(&self, interrupt_id: u8) -> bool {
        self.pics.iter().any(|p| p.handles_interrupt(interrupt_id))
    }

    /// Figure out which (if any) PICs in our chain need to know about this
    /// interrupt.  This is tricky, because all interrupts from `pics[1]`
    /// get chained through `pics[0]`.
    pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
        if self.handles_interrupt(interrupt_id) {
            if self.pics[1].handles_interrupt(interrupt_id) {
                self.pics[1].end_of_interrupt();
            }
            self.pics[0].end_of_interrupt();
        }
    }
}

pub mod Timer {
    use crate::arch::io::outb;

    pub static mut COUNT_DOWN: u64 = 0;
    pub static mut UPTIME: u64 = 0;

    pub const FREQUENCY: u32 = 1193180;

    const CHANNEL_0_PORT: u16 = 0x40;
    const COMMAND_PORT: u16 = 0x43;

    /// Initialize the timer to generate an interrupt with a given frequency
    pub fn init_timer(frequency: u32) {
        let divisor = FREQUENCY / frequency;

        unsafe {
            // Square wave generator (mode 3)
            outb(COMMAND_PORT, 0x36);

            outb(CHANNEL_0_PORT, (divisor & 0xFF) as u8);
            outb(CHANNEL_0_PORT, ((divisor >> 8) & 0xFF) as u8);
        }
    }

    /// Sleep for a number of milliseconds
    pub fn sleep(millis: u64) {
        unsafe {
            let volatile = &mut COUNT_DOWN as *mut u64;

            core::ptr::write_volatile(volatile, millis);

            while core::ptr::read_volatile(volatile) > 0 {
                crate::arch::interrupts::hlt();
            }
        }
    }
}

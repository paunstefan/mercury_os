use crate::arch;

/// Write a single byte to the output channel
///
/// # Safety
/// This method is unsafe because it does port accesses without synchronisation
pub unsafe fn putb(b: u8) {
    // Wait for the serial port's fifo to not be empty
    while (arch::io::inb(0x3F8 + 5) & 0x20) == 0 {
        // Do nothing
    }
    // Send the byte out the serial port
    arch::io::outb(0x3F8, b);

    // Also send to the bochs 0xe9 hack
    arch::io::outb(0xe9, b);
}

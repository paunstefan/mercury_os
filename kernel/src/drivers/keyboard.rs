use crate::sync::SpinMutex;

use super::chardev::CharDev;

pub static KEYS: SpinMutex<KeyboardState> = SpinMutex::new(KeyboardState::new());

const NO_KEYS: usize = 6;

pub struct KeyboardState {
    pressed_keys: [u8; NO_KEYS],
    no_pressed: u8,
}

impl KeyboardState {
    const fn new() -> Self {
        KeyboardState {
            pressed_keys: [0; 6],
            no_pressed: 0,
        }
    }

    /// Fills the given buffer with the currently pressed keys
    pub fn get(buf: &mut [u8]) {
        crate::arch::interrupts::free(|| buf.copy_from_slice(&KEYS.lock().pressed_keys));
    }

    /// Update the currently pressed keys
    /// Safety
    /// Should only be callded from the interrupt handler
    pub unsafe fn update(scancode: u8) {
        let pk = &mut KEYS.lock();
        if pk.no_pressed == 6 {
            return;
        }

        // Pressed
        if scancode < 128 {
            for k in &pk.pressed_keys {
                if *k == scancode {
                    return;
                }
            }
            for k in &mut pk.pressed_keys {
                if *k == 0 {
                    *k = scancode;
                    break;
                }
            }
        }
        // Released
        else {
            for k in &mut pk.pressed_keys {
                if *k == (scancode - 128) {
                    *k = 0;
                    break;
                }
            }
        }
    }
}

pub struct Keyboard;

impl CharDev for Keyboard {
    fn read(&self, size: usize, buf: &mut [u8]) -> Option<usize> {
        if size != NO_KEYS || buf.len() != NO_KEYS {
            return None;
        }
        KeyboardState::get(buf);

        Some(buf.iter().filter(|k| **k != 0).count())
    }

    fn write(&mut self, _size: usize, _buf: &[u8]) -> Option<usize> {
        None
    }
}

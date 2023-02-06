use core::ffi::c_void;

/// Checks if an address is aligned to a specified alignment.
#[inline]
pub fn is_aligned(address: u64, alignment: u64) -> bool {
    assert!(alignment & (alignment - 1) == 0);

    address == align_down(address, alignment)
}

/// Aligns an address to the first smaller aligned address.
/// Alignment must be a power of 2.
#[inline]
pub fn align_down(address: u64, alignment: u64) -> u64 {
    assert!(alignment & (alignment - 1) == 0);
    address & !(alignment - 1)
}

/// Aligns an address to the first bigger aligned address.
/// Alignment must be a power of 2.
#[inline]
pub fn align_up(address: u64, alignment: u64) -> u64 {
    assert!(alignment & (alignment - 1) == 0);
    if is_aligned(address, alignment) {
        return address;
    }
    (address & !(alignment - 1)) + alignment
}

/// Default string compare uses `memcmp`, which seems to be undefined in compiler_builtins
/// FIXED, but will keep it here anyway
pub fn _string_cmp(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    a.chars().zip(b.chars()).all(|(x, y)| x == y)
}

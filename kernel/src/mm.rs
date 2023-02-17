use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

use crate::arch::paging::{PageAllocator, PAGE_SIZE};
use crate::sync::SpinMutex;
use crate::utils::align_up;

#[global_allocator]
pub static ALLOCATOR: SpinMutex<BumpAllocator> = SpinMutex::new(BumpAllocator::new());

pub struct BumpAllocator {
    pub page_allocator: Option<PageAllocator>,
    heap_start: usize,
    heap_end: usize,
    next: usize,
    count: usize,
}

impl BumpAllocator {
    const fn new() -> Self {
        BumpAllocator {
            page_allocator: None,
            heap_start: 0,
            heap_end: 0,
            next: 0,
            count: 0,
        }
    }

    /// Initializes the heap to a size of _no_pages * PAGE_SIZE
    pub fn init(&mut self, mut allocator: PageAllocator, no_pages: usize) {
        let start = allocator.alloc_next_page(no_pages).unwrap().start_address;

        self.page_allocator = Some(allocator);
        self.heap_start = start.as_u64() as usize;
        self.heap_end = self.heap_start + no_pages * PAGE_SIZE as usize - 1;
        self.next = self.heap_start;
    }
}

unsafe impl GlobalAlloc for SpinMutex<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock();
        let alloc_start = align_up(bump.next as u64, layout.align() as u64) as usize;
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return null_mut(),
        };

        if alloc_end > bump.heap_end {
            null_mut() // out of memory
        } else {
            bump.next = alloc_end;
            bump.count += 1;
            alloc_start as *mut u8
        }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock();

        bump.count -= 1;
        if bump.count == 0 {
            bump.next = bump.heap_start;
        }
    }
}

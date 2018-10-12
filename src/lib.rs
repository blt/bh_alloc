extern crate libc;

use libc::{_exit, EXIT_SUCCESS};

use std::alloc::{GlobalAlloc, Layout};
use std::cell::UnsafeCell;

pub const TOTAL_BYTES: usize = 500_000_000; // 500 MB
static mut HEAP: [u8; TOTAL_BYTES] = [0; TOTAL_BYTES];

// Bump allocator for *single* core systems
pub struct BumpAlloc {
    offset: UnsafeCell<usize>,
}

unsafe impl Sync for BumpAlloc {}

// thanks, wee_alloc
trait ConstInit {
    const INIT: Self;
}

impl ConstInit for BumpAlloc {
    const INIT: BumpAlloc = BumpAlloc {
        offset: UnsafeCell::new(0),
    };
}

impl BumpAlloc {
    pub const INIT: Self = <Self as ConstInit>::INIT;
}

unsafe impl GlobalAlloc for BumpAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let offset = self.offset.get();
        let byte_size: usize = layout.size() as usize;

        let end = *offset + byte_size;

        if end >= TOTAL_BYTES {
            _exit(EXIT_SUCCESS);
        } else {
            let p = HEAP[*offset..end].as_mut_ptr() as *mut u8;
            *offset = end;
            p
        }
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // never deallocate
    }
}

extern crate libc;

use libc::{c_void, sbrk};

use std::alloc::{GlobalAlloc, Layout};
use std::cell::UnsafeCell;
use std::ptr;

// Bump pointer allocator for *single* core systems
struct BumpPointerAlloc {
    head: UnsafeCell<*mut u8>,
    end: UnsafeCell<*const u8>,
}

const TOTAL_ALLOC: i32 = 1024;

unsafe impl Sync for BumpPointerAlloc {}

unsafe impl GlobalAlloc for BumpPointerAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // It'd be nice to avoid this branch at every alloc but I'm unsure how
        // to get GlobalAlloc to initialize at start-of-program. If we had that,
        // we could call sbrk prior to any operations and be done.
        let head = self.head.get();
        let end = self.end.get();

        if head.is_null() {
            let base: *mut c_void = sbrk(TOTAL_ALLOC);
            *head = base as *mut u8;
            *end = *head.offset(TOTAL_ALLOC as isize);
        }

        unimplemented!()
        // // `interrupt::free` is a critical section that makes our allocator safe
        // // to use from within interrupts
        // interrupt::free(|_| {
        //     let head = self.head.get();

        //     let align = layout.align();
        //     let res = *head % align;
        //     let start = if res == 0 { *head } else { *head + align - res };
        //     if start + align > self.end {
        //         // a null pointer signal an Out Of Memory condition
        //         ptr::null_mut()
        //     } else {
        //         *head = start + align;
        //         start as *mut u8
        //     }
        // })
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // never deallocate
    }
}

// Declaration of the global memory allocator
// NOTE the user must ensure that the memory region `[0x2000_0100, 0x2000_0200]`
// is not used by other parts of the program
#[global_allocator]
static HEAP: BumpPointerAlloc = BumpPointerAlloc {
    head: UnsafeCell::new(ptr::null_mut()),
    end: UnsafeCell::new(ptr::null()),
};

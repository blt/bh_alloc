#![deny(clippy::pedantic)]
#![deny(clippy::all)]
#![no_std]

#[deny(bad_style)]
#[deny(future_incompatible)]
#[deny(missing_docs)]
#[deny(nonstandard_style)]
#[deny(rust_2018_compatibility)]
#[deny(rust_2018_idioms)]
#[deny(unused)]
#[deny(warnings)]
pub mod fuzz;
mod util;

use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr;
use core::sync::atomic::{
    spin_loop_hint, AtomicBool, AtomicUsize, Ordering, ATOMIC_BOOL_INIT, ATOMIC_USIZE_INIT,
};
use util::align_diff;

extern crate libc;

/// Total number of bytes that [`BumpAlloc`] will have available to it.
pub const TOTAL_BYTES: usize = 5_048_576; // 5 mebibytes

/// Bump allocator for multi-core systems
///
/// A bump allocator keeps a single pointer to the start of the unitialized
/// heap. When an allocation happens this pointer is 'bumped' sufficiently to
/// fit the allocation. Deallocations have no effect on the pointer, meaning
/// that memory is allocated at program start and never freed. This is very
/// fast.
pub struct BumpAlloc {
    alloc_lock: AtomicBool,
    memblk: UnsafeCell<*mut u8>,
    offset: AtomicUsize,
}

unsafe impl Sync for BumpAlloc {}

// thanks, wee_alloc
trait ConstInit {
    #[allow(clippy::declare_interior_mutable_const)]
    const INIT: Self;
}

impl ConstInit for BumpAlloc {
    const INIT: Self = Self {
        alloc_lock: ATOMIC_BOOL_INIT,
        memblk: UnsafeCell::new(ptr::null_mut()),
        offset: ATOMIC_USIZE_INIT,
    };
}

impl BumpAlloc {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const INIT: Self = <Self as ConstInit>::INIT;
}

unsafe impl GlobalAlloc for BumpAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut offset = self.offset.load(Ordering::Relaxed);

        while (*self.memblk.get()).is_null() {
            // first allocation, no prior call to mmap to get pages from OS
            loop {
                match self.alloc_lock.compare_exchange_weak(
                    false,
                    true,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        let ptr = libc::mmap(
                            0 as *mut libc::c_void,
                            TOTAL_BYTES as libc::size_t,
                            libc::PROT_READ | libc::PROT_WRITE,
                            libc::MAP_ANON | libc::MAP_PRIVATE,
                            -1,
                            0 as libc::off_t,
                        );
                        if ptr == libc::MAP_FAILED {
                            return ptr::null_mut();
                        }
                        *self.memblk.get() = ptr as *mut u8;
                        self.alloc_lock.store(false, Ordering::Release);
                        break;
                    }
                    Err(_) => spin_loop_hint(),
                }
            }
        }

        loop {
            let diff = align_diff((*self.memblk.get()).add(offset) as usize, layout.align());
            let start = offset + diff;
            let end = start.saturating_add(layout.size());

            if end >= TOTAL_BYTES {
                return ptr::null_mut();
            } else {
                match self.offset.compare_exchange_weak(
                    offset,
                    end,
                    Ordering::Acquire,
                    Ordering::Relaxed,
                ) {
                    Ok(_) => {
                        return (*self.memblk.get()).add(start);
                    }
                    Err(cur) => {
                        offset = cur;
                    }
                }
            }
        }
    }

    unsafe fn dealloc(&self, _: *mut u8, _: Layout) {
        // never deallocate
    }
}

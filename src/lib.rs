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
use core::ptr;
use core::sync::atomic::{AtomicUsize, Ordering};
use util::align_diff;

/// Total number of bytes that [`BumpAlloc`] will have available to it.
pub const TOTAL_BYTES: usize = 512_000; // 500 kibibytes
static mut HEAP: [u8; TOTAL_BYTES] = [0; TOTAL_BYTES];

/// Bump allocator for multi-core systems
///
/// A bump allocator keeps a single pointer to the start of the unitialized
/// heap. When an allocation happens this pointer is 'bumped' sufficiently to
/// fit the allocation. Deallocations have no effect on the pointer, meaning
/// that memory is allocated at program start and never freed. This is very
/// fast.
pub struct BumpAlloc {
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
        offset: AtomicUsize::new(0),
    };
}

impl BumpAlloc {
    #[allow(clippy::declare_interior_mutable_const)]
    pub const INIT: Self = <Self as ConstInit>::INIT;
}

unsafe impl GlobalAlloc for BumpAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut offset = self.offset.load(Ordering::Relaxed);
        loop {
            let diff = align_diff(HEAP.as_mut_ptr().add(offset) as usize, layout.align());
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
                        return HEAP[start..end].as_mut_ptr() as *mut u8;
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

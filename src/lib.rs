#![cfg_attr(feature = "cargo-clippy", allow(clippy::cargo))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::complexity))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::correctness))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::perf))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

// #[cfg(test)]
// extern crate quickcheck;

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

use std::alloc::{GlobalAlloc, Layout};
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};
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
    const INIT: Self;
}

impl ConstInit for BumpAlloc {
    const INIT: BumpAlloc = BumpAlloc {
        offset: AtomicUsize::new(0),
    };
}

impl BumpAlloc {
    pub const INIT: Self = <Self as ConstInit>::INIT;
}

const BYTE_ALIGNMENT: usize = 16;

unsafe impl GlobalAlloc for BumpAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut offset = self.offset.load(Ordering::Relaxed);
        loop {
            let diff = align_diff(HEAP.as_mut_ptr().add(offset) as usize, BYTE_ALIGNMENT);
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

#![cfg_attr(feature = "cargo-clippy", allow(clippy::cargo))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::complexity))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::correctness))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::perf))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]

#[cfg(test)]
extern crate quickcheck;

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
use util::align_gt;

/// Total number of bytes that [`BumpAlloc`] will have available to it.
pub const TOTAL_BYTES: usize = 500_000_000; // 500 MB
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

unsafe impl GlobalAlloc for BumpAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut offset = self.offset.load(Ordering::Relaxed);
        loop {
            let start = align_gt(offset, layout);
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

#[cfg(test)]
mod test {
    use super::*;
    use std::alloc::Layout;
    use std::mem;

    #[test]
    fn algorithm_test() {
        // This test is the same basic bump algorithm -- minus the concurrency
        // bits -- that you'll find above. The allocation should consume only
        // the minimum number of bytes needed but each subsequent allocation
        // should be aligned on word boundaries.
        let mut offset = 0;
        // start, end
        let layout = Layout::from_size_align(mem::size_of::<u8>(), mem::size_of::<u64>()).unwrap();
        let examples = vec![(0, 1), (16, 17), (48, 49)];
        for (start_exp, end_exp) in examples.into_iter() {
            let start = align_gt(offset, layout);
            let end = start.saturating_add(layout.size());

            assert_eq!(start, start_exp);
            assert_eq!(end, end_exp);

            offset = end;
        }
    }
}

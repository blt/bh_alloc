#![cfg_attr(feature = "cargo-clippy", allow(clippy::cargo))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::complexity))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::correctness))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::pedantic))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::perf))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![cfg_attr(feature = "cargo-clippy", feature(tool_lints))]

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

use std::alloc::{GlobalAlloc, Layout};
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};

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

fn round_to_multiple_of(val: usize, align: usize) -> usize {
    if align == 0 {
        return val;
    }

    let rem = val % align;
    if rem == 0 {
        val
    } else {
        (val + align) - rem
    }
}

unsafe impl GlobalAlloc for BumpAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let alignment = layout.align();
        let byte_size: usize = round_to_multiple_of(layout.size() as usize, alignment);

        let mut offset = self.offset.load(Ordering::Relaxed);
        loop {
            let end = offset + byte_size;

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
                        return HEAP[offset..end].as_mut_ptr() as *mut u8;
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
    use quickcheck::{QuickCheck, TestResult};

    #[test]
    fn round_up_test_always_greater_equal() {
        fn inner(val: usize, align: usize) -> TestResult {
            let ret = round_to_multiple_of(val, align);
            TestResult::from_bool(ret >= val)
        }
        QuickCheck::new().quickcheck(inner as fn(usize, usize) -> TestResult);
    }
}

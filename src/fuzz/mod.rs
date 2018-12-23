//! Allocators suitable for fuzzing environments
//!
//! The allocators available in this sub-crate are intended to be used in
//! fuzzing targets. The number of branches are kept intentionally low and
//! suitability for threaded environments are a non-priority.

extern crate libc;

use self::libc::{_exit, EXIT_SUCCESS};
use super::util::align_diff;
use super::TOTAL_BYTES;
use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr;

/// Total number of bytes that [`BumpAlloc`] will have available to it.
static mut HEAP: [u8; TOTAL_BYTES] = [0; TOTAL_BYTES];

/// Bump allocator for *single* core systems
///
/// A bump allocator keeps a single pointer to the start of the unitialized
/// heap. When an allocation happens this pointer is 'bumped' sufficiently to
/// fit the allocation. Deallocations have no effect on the pointer, meaning
/// that memory is allocated at program start and never freed. This is very
/// fast.
///
/// `BumpAlloc` has an additional feature. When all its heap memory is exhausted
/// `libc::_exit(EXIT_SUCCESS)` is called. This behaviour aids in the production
/// of fuzzers.
pub struct BumpAlloc {
    memblk: UnsafeCell<*mut u8>,
    offset: UnsafeCell<usize>,
}

unsafe impl Sync for BumpAlloc {}

// thanks, wee_alloc
trait ConstInit {
    #[allow(clippy::declare_interior_mutable_const)]
    const INIT: Self;
}

impl ConstInit for BumpAlloc {
    const INIT: Self = Self {
        memblk: UnsafeCell::new(ptr::null_mut()),
        offset: UnsafeCell::new(0),
    };
}

impl BumpAlloc {
    /// Initialization for [`BumpAlloc`]
    ///
    /// See the binaries in this repository for full examples.
    #[allow(clippy::declare_interior_mutable_const)]
    pub const INIT: Self = <Self as ConstInit>::INIT;
}

unsafe impl GlobalAlloc for BumpAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let offset = self.offset.get();

        if (*self.memblk.get()).is_null() {
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
        }

        let diff = align_diff(HEAP.as_mut_ptr().add(*offset) as usize, layout.align());
        let start = *offset + diff;
        let end = start.saturating_add(layout.size());

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

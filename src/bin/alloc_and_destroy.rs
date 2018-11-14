extern crate bh_alloc;

use std::mem;

#[global_allocator]
static ALLOC: bh_alloc::BumpAlloc = bh_alloc::BumpAlloc::INIT;

fn main() {
    // From experimentation, this program requires 24 words of allocation before
    // getting to the loop below. I'm unsure of what's being allocated. If this
    // program fails to run to completion try fiddling with the magic constant.
    let cap = (bh_alloc::TOTAL_BYTES / mem::size_of::<usize>()) - 24;
    for i in 0..cap {
        let bi = Box::new(i);
        drop(bi);
    }
}

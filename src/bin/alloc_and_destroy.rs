extern crate bh_alloc;

#[global_allocator]
static ALLOC: bh_alloc::fuzz::BumpAlloc = bh_alloc::fuzz::BumpAlloc::INIT;

fn main() {
    for i in 0..=bh_alloc::TOTAL_BYTES {
        let bi = Box::new(i);
        drop(bi);
    }
    // We will exit(0) before leaving the above loop.
    unreachable!()
}

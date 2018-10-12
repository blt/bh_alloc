extern crate bh_alloc;

#[global_allocator]
static ALLOC: bh_alloc::BumpAlloc = bh_alloc::BumpAlloc::INIT;

fn main() {
    for i in 0..bh_alloc::TOTAL_BYTES + 1 {
        let bi = Box::new(i);
        drop(bi);
    }
    // We will exit(0) before leaving the above loop.
    unreachable!()
}

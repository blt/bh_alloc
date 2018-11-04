extern crate bh_alloc;

#[global_allocator]
static ALLOC: bh_alloc::BumpAlloc = bh_alloc::BumpAlloc::INIT;

fn main() {
    println!("Hello, world!");
    for i in 0..1_000_000 {
        println!("{}", i);
    }
}

extern crate hashbrown;
extern crate bh_alloc;

#[global_allocator]
static ALLOC: bh_alloc::BumpAlloc = bh_alloc::BumpAlloc::INIT;

#[test]
fn hello_world() {
    println!("Hello, world!");
}

#[test]
fn alloc_and_destroy() {
    for i in 0..5_000_000 {
        let bi = Box::new(i);
        drop(bi);
    }
}

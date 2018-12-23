extern crate bh_alloc;
extern crate rayon;

#[global_allocator]
static ALLOC: bh_alloc::BumpAlloc = bh_alloc::BumpAlloc::INIT;

use rayon::prelude::*;
fn sum_of_squares(input: &[u16]) -> usize {
    input
        .par_iter() // <-- just change that!
        .map(|&i| (i as usize) * (i as usize))
        .sum()
}

#[test]
fn doc_example() {
    let input: Vec<u16> = (0..u16::max_value()).collect();
    assert_eq!(93_818_549_927_935, sum_of_squares(&input));
}

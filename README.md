# bh_alloc

This project implements a simple bump allocator for the
[bughunt-rust](https://github.com/blt/bughunt-rust/) project. The allocator
works from a fixed-size pool, only ever allocating and never deallocating. When
the allocator runs out of memory `exit(0)` is called.

The motivation for this kind of allocator is to avoid memory allocation failure
panics during fuzz runs.

The idea is via @shnatsel in [this
discussion](https://www.reddit.com/r/rust/comments/9mhfml/hunting_for_bugs_in_rust/e7f2c50/). I
read through [wee_alloc](https://github.com/rustwasm/wee_alloc/) when writing
this. That project's static array implementation saved me from calling `sbrk`
somewhere.

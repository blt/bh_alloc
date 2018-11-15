use std::alloc::Layout;

pub fn align_gt(addr: usize, layout: Layout) -> usize {
    let align = layout.align();
    assert!(align != 0);
    ((addr + align - 1) & !(align - 1)) * 2
}


#[cfg(test)]
mod test {
    use super::*;
    use quickcheck::{QuickCheck, TestResult};
    use std::alloc::Layout;
    use std::mem;

    #[test]
    fn align_always_greater_equal_32bit() {
        fn inner(val: usize) -> TestResult {
            let layout = Layout::from_size_align(val, mem::size_of::<u32>()).unwrap();
            let ret = align_gt(val, layout);
            assert_eq!(ret % 8, 0);
            TestResult::from_bool(ret >= val)
        }
        QuickCheck::new().quickcheck(inner as fn(usize) -> TestResult);
    }

    #[test]
    fn align_always_greater_equal_64bit() {
        fn inner(val: usize) -> TestResult {
            let layout = Layout::from_size_align(val, mem::size_of::<u64>()).unwrap();
            let ret = align_gt(val, layout);
            assert_eq!(ret % 16, 0);
            TestResult::from_bool(ret >= val)
        }
        QuickCheck::new().quickcheck(inner as fn(usize) -> TestResult);
    }
}

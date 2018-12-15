pub fn align_diff(offset: usize, align_size_bytes: usize) -> usize {
    let rem = offset % align_size_bytes;
    if rem == 0 {
        return 0;
    }

    align_size_bytes - rem

    // if align_size_bytes == 0 {
    //     return offset;
    // }

    // let rem = offset % align_size_bytes;
    // if rem == 0 {
    //     return offset;
    // }

    // (offset + align_size_bytes) - rem
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn align_diff_test() {
        assert_eq!(0, align_diff(0, 16));
        assert_eq!(0, align_diff(32, 16));
        assert_eq!(0, align_diff(16, 16));
        assert_eq!(3, align_diff(13, 16));
    }

    // #[test]
    // fn align_test() {
    //     // Goal is to align allocations along double words, here 16 bytes.
    //     let double_word_bytes = 16;
    //     let mut offset = 0;
    //     // The first allocation takes place at memory offset 16 and is for one
    //     // byte. Expectation is that 0 bytes will need to be added to the offset
    //     // to meet alignment.
    //     assert_eq!(0, align(offset, double_word_bytes));
    //     offset += 1;
    //     // Okay, now, offset is 1 and we want to allocate another byte. We'll be
    //     // scooted forward to the 16th byte.
    //     assert_eq!(16, align(offset, double_word_bytes));
    //     offset = 16;
    //     offset += 1;
    //     // Now the offset is 17 and a u32 is being allocated. We'll be scooted
    //     // forward to the 32nd byte and the offset will be 36.
    //     assert_eq!(32, align(offset, double_word_bytes));
    //     offset = 32;
    //     offset += 4;
    //     // Now allocate another byte.
    //     assert_eq!(48, align(offset, double_word_bytes));
    // }
}

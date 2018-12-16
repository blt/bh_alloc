pub fn align_diff(offset: usize, align_size_bytes: usize) -> usize {
    let rem = offset % align_size_bytes;
    if rem == 0 {
        return 0;
    }

    align_size_bytes - rem
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
}

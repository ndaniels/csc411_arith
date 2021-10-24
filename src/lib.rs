static Q_TABLE: [f32; 16] = [
    -0.35,  // 0
    -0.20,  // 1
    -0.15,  // 2
    -0.10,  // 3
    -0.077, // 4
    -0.055, // 5
    -0.033, // 6
    -0.011, // 7
    0.011,  // 8
    0.033,  // 9
    0.055,  // 10
    0.077,  // 11
    0.10,   // 12
    0.15,   // 13
    0.20,   // 14
    0.35,   // 15
];

/// Given an `index` in the [0, 15] range, returns the `chroma` value for that `index`.
/// 
/// # Arguments:
/// * `index`: The index in a quantization table. If greater than 15, this function will panic.
pub fn chroma_of_index(index: usize) -> f32 {
    Q_TABLE[index]
}

/// Given a `chroma` value, returns the index of its closest value in the quantization table.
/// 
/// # Arguments:
/// * `chroma`: A chroma value.
pub fn index_of_chroma(chroma: f32) -> usize {
    Q_TABLE
        .iter()
        .map(|&c| f32::abs(c - chroma))
        .enumerate()
        .fold(
            (0, 1_f32),
            |(i_min, v_min), (i, v)| {
                if v < v_min {
                    (i, v)
                } else {
                    (i_min, v_min)
                }
            },
        )
        .0
}

#[cfg(test)]
mod tests {
    use crate::chroma_of_index;
    use crate::index_of_chroma;

    #[test]
    fn test_exact() {
        for i in 0..16 {
            assert_eq!(i, index_of_chroma(chroma_of_index(i)));
        }
    }

    #[test]
    fn test_inexact() {
        assert_eq!(0, index_of_chroma(-0.5));
        assert_eq!(0, index_of_chroma(-0.3));
        assert_eq!(1, index_of_chroma(-0.25));
        assert_eq!(4, index_of_chroma(-0.08));
        assert_eq!(7, index_of_chroma(-1e-8));
        assert_eq!(7, index_of_chroma(0.));
        assert_eq!(8, index_of_chroma(1e-8));
        assert_eq!(12, index_of_chroma(0.12));
        assert_eq!(14, index_of_chroma(0.25));
        assert_eq!(15, index_of_chroma(0.3));
        assert_eq!(15, index_of_chroma(0.5));
    }

    #[test]
    #[should_panic]
    fn test_out_of_bounds() {
        chroma_of_index(20);
    }
}

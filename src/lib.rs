#![warn(clippy::pedantic, clippy::nursery)]

/// # Errors
///
/// Will return `Err` if the length of `row` is not `n + 1`.
pub fn nth_pascal_row(n: usize, row: &mut [u64]) -> Result<(), &'static str> {
    if row.len() != n + 1 {
        return Err("Slice must have length n + 1");
    }

    row[0] = 1;

    for i in 1..=n {
        // Work backwards to avoid overwriting values we still need
        for j in (1..=i).rev() {
            row[j] += row[j - 1];
        }

        // Rightmost element of every row is 1
        row[i] = 1;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::nth_pascal_row;

    #[test]
    fn slice_too_small() {
        let mut row = vec![0u64; 5];
        let result = nth_pascal_row(10, &mut row);
        assert!(result.is_err());
    }

    #[test]
    fn slice_too_large() {
        let mut row = vec![0u64; 10];
        let result = nth_pascal_row(5, &mut row);
        assert!(result.is_err());
    }

    #[test]
    fn row_0() {
        let mut row = vec![0u64; 1];
        nth_pascal_row(0, &mut row).unwrap();
        assert_eq!(row, vec![1]);
    }

    #[test]
    fn row_0_nonzero_init() {
        let mut row = vec![42u64; 1];
        nth_pascal_row(0, &mut row).unwrap();
        assert_eq!(row, vec![1]);
    }

    #[test]
    fn row_1() {
        let mut row = vec![0u64; 2];
        nth_pascal_row(1, &mut row).unwrap();
        assert_eq!(row, vec![1, 1]);
    }

    #[test]
    fn row_1_nonzero_init() {
        let mut row = vec![42u64; 2];
        nth_pascal_row(1, &mut row).unwrap();
        assert_eq!(row, vec![1, 1]);
    }

    #[test]
    fn row_2() {
        let mut row = vec![0u64; 3];
        nth_pascal_row(2, &mut row).unwrap();
        assert_eq!(row, vec![1, 2, 1]);
    }

    #[test]
    fn row_3() {
        let mut row = vec![0u64; 4];
        nth_pascal_row(3, &mut row).unwrap();
        assert_eq!(row, vec![1, 3, 3, 1]);
    }

    #[test]
    fn row_4() {
        let mut row = vec![0u64; 5];
        nth_pascal_row(4, &mut row).unwrap();
        assert_eq!(row, vec![1, 4, 6, 4, 1]);
    }

    #[test]
    fn row_8() {
        let mut row = vec![0u64; 9];
        nth_pascal_row(8, &mut row).unwrap();
        assert_eq!(row, vec![1, 8, 28, 56, 70, 56, 28, 8, 1]);
    }

    #[test]
    fn row_16() {
        let mut row = vec![0u64; 17];
        nth_pascal_row(16, &mut row).unwrap();
        assert_eq!(
            row,
            vec![
                1, 16, 120, 560, 1820, 4368, 8008, 11440, 12870, 11440, 8008, 4368, 1820, 560, 120,
                16, 1
            ]
        );
    }

    #[test]
    fn row_16_nonzero_init() {
        let mut row = vec![42u64; 17];
        nth_pascal_row(16, &mut row).unwrap();
        assert_eq!(
            row,
            vec![
                1, 16, 120, 560, 1820, 4368, 8008, 11440, 12870, 11440, 8008, 4368, 1820, 560, 120,
                16, 1
            ]
        );
    }

    #[test]
    fn row_32() {
        let mut row = vec![0u64; 33];
        nth_pascal_row(32, &mut row).unwrap();
        // Check selected values
        assert_eq!(row[0], 1);
        assert_eq!(row[1], 32);
        assert_eq!(row[16], 601080390); // Peak
        assert_eq!(row[31], 32);
        assert_eq!(row[32], 1);
    }
}

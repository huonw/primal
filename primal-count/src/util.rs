/// Returns the largest integer at least sqrt(n) using Newton's method
pub fn int_square_root(value: usize) -> usize {
    let mut x = value;
    let mut y = (x + 1) >> 1;
    while y < x {
        x = y;
        y = (x + value / x) >> 1;
    }
    return x;
}

/// Returns the largest integer at least cbrt(n) using Newton's method
pub fn int_cubic_root(value: usize) -> usize {
    let mut x = value;
    let mut y = (2 * value + 1) / 3; // Plus 1 only to protect against division by 0 later
    while y < x {
        x = y;
        y = (2 * x + value / x / x) / 3; // Divide split to protect against overflow
    }
    return x;
}

// Returns the largest integer at least n^(1/4) using our fast integer sqrt
// N.b. it's faster to use two sqrts than naively apply Newton here
pub fn int_quartic_root(value: usize) -> usize {
    return int_square_root(int_square_root(value));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_int_sqrt() {
        use util::int_square_root;
        assert_eq!(int_square_root(0), 0);
        assert_eq!(int_square_root(1), 1);
        assert_eq!(int_square_root(16), 4);
        assert_eq!(int_square_root(17), 4);
        assert_eq!(int_square_root(24), 4);
        assert_eq!(int_square_root(587 * 587 - 1), 586);
    }

    #[test]
    fn test_int_cbrt() {
        use util::int_cubic_root;
        assert_eq!(int_cubic_root(0), 0);
        assert_eq!(int_cubic_root(1), 1);
        assert_eq!(int_cubic_root(26), 2);
        assert_eq!(int_cubic_root(27), 3);
        assert_eq!(int_cubic_root(28), 3);
        assert_eq!(int_cubic_root(587 * 587 * 587 - 1), 586);
    }

    #[test]
    fn test_int_quartic_root() {
        use util::int_quartic_root;
        assert_eq!(int_quartic_root(0), 0);
        assert_eq!(int_quartic_root(1), 1);
        assert_eq!(int_quartic_root(15), 1);
        assert_eq!(int_quartic_root(16), 2);
        assert_eq!(int_quartic_root(17), 2);
        assert_eq!(int_quartic_root(587 * 587 * 587 * 587 - 1), 586);
        assert_eq!(int_quartic_root(587 * 587 * 587 * 587 + 1), 587);
    }
}

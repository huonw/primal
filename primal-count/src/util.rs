pub fn integer_square_root(value: usize) -> usize {
    // Returns the largest integer at least sqrt(n) using Newton's method
    let mut x = value;
    let mut y = (x + 1) >> 1;
    while y < x {
        x = y;
        y = (x + value / x) >> 1;
    }
    return x;
}

pub fn integer_cubic_root(value: usize) -> usize {
    // Returns the largest integer at least cbrt(n) using Newton's method
    let mut x = value;
    let mut y = (2 * value + 1) / 3; // Plus 1 only to protect against division by 0 later - not because it's right
    while y < x {
        x = y;
        y = (2 * x + value / x / x) / 3; // Weird divide to protect against overflow
    }
    return x;
}

pub fn integer_quartic_root(value: usize) -> usize {
    // Returns the largest integer at least n^(1/4) using our fast integer sqrt
    // N.b. it's faster to use two sqrts than naively apply Newton here
    return integer_square_root(integer_square_root(value));
}
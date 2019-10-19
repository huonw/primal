#[cfg(test)]
mod tests {        
    #[test]
    fn test_int_sqrt() {
        use crate::util::integer_square_root;
        assert_eq!(integer_square_root(0), 0);
        assert_eq!(integer_square_root(1), 1);
        assert_eq!(integer_square_root(16), 4);
        assert_eq!(integer_square_root(17), 4);
        assert_eq!(integer_square_root(24), 4);
        assert_eq!(integer_square_root(587 * 587 - 1), 586);
    }

    #[test]
    fn test_int_cbrt() {
        use crate::util::integer_cubic_root;
        assert_eq!(integer_cubic_root(0), 0);
        assert_eq!(integer_cubic_root(1), 1);
        assert_eq!(integer_cubic_root(26), 2);
        assert_eq!(integer_cubic_root(27), 3);
        assert_eq!(integer_cubic_root(28), 3);
        assert_eq!(integer_cubic_root(587 * 587 * 587 - 1), 586);
    }

    #[test]
    fn test_int_quartic_root() {
        use crate::util::integer_quartic_root;
        assert_eq!(integer_quartic_root(0), 0);
        assert_eq!(integer_quartic_root(1), 1);
        assert_eq!(integer_quartic_root(15), 1);
        assert_eq!(integer_quartic_root(16), 2);
        assert_eq!(integer_quartic_root(17), 2);
        assert_eq!(integer_quartic_root(587 * 587 * 587 * 587 - 1), 586);
        assert_eq!(integer_quartic_root(587 * 587 * 587 * 587 + 1), 587);
    }

    #[test]
    fn test_meissel_fn() {
        use crate::prime_count::meissel_fn;
        use std::collections::HashMap;
        let prime_array = vec![2, 3, 5, 7, 11, 13, 17, 19];
        let mut meissel_cache = HashMap::new();
        assert_eq!(meissel_fn(30, 8, &prime_array, &mut meissel_cache), 3);
        assert_eq!(meissel_fn(100, 1, &prime_array, &mut meissel_cache), 50);
    }

    #[test]
    fn test_primes_below() {
        use crate::prime_count::primes_below;
        assert_eq!(primes_below(7), 4);
        assert_eq!(primes_below(100), 25);
        assert_eq!(primes_below(2143), 324);
        assert_eq!(primes_below(1_000_000), 78_498);
        assert_eq!(primes_below(1_000_000_000), 50_847_534);
        // assert_eq!(primes_below(1_000_000_000_000), 37_607_912_018);
        // assert_eq!(primes_below(1_000_000_000_000_000), 29_844_570_422_669);
    }

    #[test]
    fn test_primes_below_struct() {
        use crate::prime_count::PrimeCounter;
        let mut pc = PrimeCounter::new(10_000);
        assert_eq!(pc.primes_below(7), 4);
        assert_eq!(pc.primes_below(100), 25);
        assert_eq!(pc.primes_below(2143), 324);

        pc.update_limit(1_000_000_000);
        assert_eq!(pc.primes_below(1_000_000), 78_498);
        assert_eq!(pc.primes_below(1_000_000_000), 50_847_534);
        // assert_eq!(primes_below(1_000_000_000_000), 37_607_912_018);
        // assert_eq!(primes_below(1_000_000_000_000_000), 29_844_570_422_669);
    }
}

mod prime_count;
mod util;
pub use prime_count::primes_below;
pub use prime_count::PrimeCounter;

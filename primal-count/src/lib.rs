#[cfg(test)]
mod tests {        
    #[test]
    fn test_int_sqrt() {
        assert_eq!(integer_square_root(0), 0);
        assert_eq!(integer_square_root(1), 1);
        assert_eq!(integer_square_root(16), 4);
        assert_eq!(integer_square_root(17), 4);
        assert_eq!(integer_square_root(24), 4);
        assert_eq!(integer_square_root(587 * 587 - 1), 586);
    }

    #[test]
    fn test_int_cbrt() {
        assert_eq!(integer_cubic_root(0), 0);
        assert_eq!(integer_cubic_root(1), 1);
        assert_eq!(integer_cubic_root(26), 2);
        assert_eq!(integer_cubic_root(27), 3);
        assert_eq!(integer_cubic_root(28), 3);
        assert_eq!(integer_cubic_root(587 * 587 * 587 - 1), 586);
    }

    #[test]
    fn test_int_quartic_root() {
        assert_eq!(integer_quartic_root(0), 0);
        assert_eq!(integer_quartic_root(1), 1);
        assert_eq!(integer_quartic_root(15), 1);
        assert_eq!(integer_quartic_root(16), 2);
        assert_eq!(integer_quartic_root(17), 2);
        assert_eq!(integer_quartic_root(587 * 587 * 587 * 587 - 1), 586);
        assert_eq!(integer_quartic_root(587 * 587 * 587 * 587 + 1), 587);
    }

    #[test]
    fn test_prime_array() {
        let prime_array = vec![2, 3, 5, 7, 11, 13, 17, 19];
        assert_eq!(create_prime_array(19), prime_array);
        assert_eq!(create_prime_array(20), prime_array);
    }

    #[test]
    fn test_meissel_fn_small() {
        let prime_array = vec![2, 3, 5, 7, 11, 13, 17, 19];
        assert_eq!(meissel_fn_small(30, 8, &prime_array), 3);
        assert_eq!(meissel_fn_small(100, 1, &prime_array), 50);
    }

    #[test]
    fn test_meissel_fn_large() {
        let prime_array = vec![2, 3, 5, 7, 11, 13, 17, 19];
        assert_eq!(meissel_fn_large(30, 8, &prime_array), 3);
        assert_eq!(meissel_fn_large(100, 1, &prime_array), 50);
    }


    #[test]
    fn test_meissel_fn_memoized() {
        let prime_array = vec![2, 3, 5, 7, 11, 13, 17, 19];
        let mut cache = HashMap::new();
        assert_eq!(meissel_memoized(30, 8, &prime_array, &mut cache), 3);
        assert_eq!(meissel_memoized(100, 1, &prime_array, &mut cache), 50);
    }

    #[test]
    fn test_num_primes_less_than() {
        assert_eq!(num_primes_less_than(7), 4);
        assert_eq!(num_primes_less_than(100), 25);
        assert_eq!(num_primes_less_than(2143), 324);
        assert_eq!(num_primes_less_than(1_000_000), 78_498);
        // assert_eq!(num_primes_less_than(1_000_000_000), 50_847_534);
    }

    #[test]
    fn test_num_primes_less_than_main() {
        assert_eq!(num_primes_less_than_main(7), 4);
        assert_eq!(num_primes_less_than_main(100), 25);
        assert_eq!(num_primes_less_than_main(2143), 324);
        assert_eq!(num_primes_less_than_main(1_000_000), 78_498);
        assert_eq!(num_primes_less_than_main(1_000_000_000), 50_847_534);
        // assert_eq!(num_primes_less_than_main(1_000_000_000_000), 37_607_912_018);
        // assert_eq!(num_primes_less_than_main(1_000_000_000_000_000), 29_844_570_422_669);
    }
}

mod prime_count;
mod util;
pub use prime_count::num_primes_less_than_main;

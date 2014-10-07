fn mod_exp(mut x: u64, mut d: u64, n: u64) -> u64 {
    let mut ret = 1;
    while d != 0 {
        if d % 2 == 1 {
            ret *= x;
            if ret >= n {
                ret %= n;
            }
        }
        d /= 2;
        x *= x;
        if x >= n {
            x %= n;
        }
    }
    ret
}

/// Test if `n` is prime, using the deterministic version of the
/// Miller-Rabin test.
///
/// Doing a lot of primality tests with numbers strictly below some
/// upper bound will be faster using the `is_prime` method of a
/// `Primes` instance.
pub fn is_prime_miller_rabin(n: u64) -> bool {
    static HINT: &'static [u64] = &[2];

    // we have a strict upper bound, so we can just use the witness
    // table of Pomerance, Selfridge & Wagstaff and Jeaschke to be as
    // efficient as possible, without having to fall back to
    // randomness.
    static WITNESSES: &'static [(u64, &'static [u64])] =
        [(2_046, HINT),
         (1_373_652, &[2, 3]),
         (9_080_190, &[31, 73]),
         (25_326_000, &[2, 3, 5]),
         (4_759_123_140, &[2, 7, 61]),
         (1_112_004_669_632, &[2, 13, 23, 1662803]),
         (2_152_302_898_746, &[2, 3, 5, 7, 11]),
         (3_474_749_660_382, &[2, 3, 5, 7, 11, 13]),
         (341_550_071_728_320, &[2, 3, 5, 7, 11, 13, 17]),
         (0xFFFF_FFFF_FFFF_FFFF, &[2, 3, 5, 7, 11, 13, 17, 19, 23])
         ];

    if n % 2 == 0 { return n == 2 }
    if n == 1 { return false }

    let mut d = n - 1;
    let mut s = 0u;
    while d % 2 == 0 { d /= 2; s += 1 }

    let witnesses =
        WITNESSES.iter().find(|&&(hi, _)| hi >= n)
            .map(|&(_, wtnss)| wtnss).unwrap();
    'next_witness: for &a in witnesses.iter() {
        let mut power = mod_exp(a, d, n);
        if power == 1 { continue 'next_witness }

        for _r in range(0, s) {
            if power == n - 1 {
                continue 'next_witness
            }
            power *= power;
            if power >= n {
                power %= n;
            }
        }
        return false
    }

    true
}

#[cfg(test)]
mod tests {
    use Primes;
    use super::is_prime_miller_rabin;

    #[test]
    fn miller_rabin() {
        static LIMIT: uint = 1_000_000;
        let sieve = Primes::sieve(LIMIT);
        for x in range(0, LIMIT) {
            let s = sieve.is_prime(x);
            let mr = is_prime_miller_rabin(x as u64);

            assert!(s == mr, "miller_rabin {} mismatches sieve {} for {}",
                    mr, s, x)
        }
    }
}

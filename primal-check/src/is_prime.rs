#[derive(Copy, Clone, PartialEq, PartialOrd, Ord, Eq, Debug)]
struct U128 {
    hi: u64,
    lo: u64,
}

fn modulo(mut a: U128, m: u64) -> u64 {
    if a.hi >= m {
        a.hi -= (a.hi / m) * m;
    }
    let mut x = a.hi;
    let mut y = a.lo;
    for _ in 0..64 {
        let t = (x as i64 >> 63) as u64;
        x = (x << 1) | (y >> 63);
        y <<= 1;
        if (x | t) >= m {
            x = x.wrapping_sub(m);
            y += 1;
        }
    }
    x
}
fn mul128(u: u64, v: u64) -> U128 {
    let u1 = u >> 32;
    let u0 = u & (!0 >> 32);
    let v1 = v >> 32;
    let v0 = v & (!0 >> 32);

    let t = u0 * v0;
    let w0 = t & (!0 >> 32);
    let k = t >> 32;

    let t = u1 * v0 + k;
    let w1 = t & (!0 >> 32);
    let w2 = t >> 32;

    let t = u0 * v1 + w1;
    let k = t >> 32;
    U128 {
        lo: (t << 32) + w0,
        hi: u1*v1 + w2 + k
    }
}
fn mod_mul_(a: u64, b: u64, m: u64) -> u64 {
    modulo(mul128(a, b), m)
}

fn mod_mul(a: u64, b: u64, m: u64) -> u64 {
    match a.checked_mul(b) {
        Some(r) => if r >= m { r % m } else { r },
        None => mod_mul_(a, b, m),
    }
}

fn mod_sqr(a: u64, m: u64) -> u64 {
    if a < (1 << 32) {
        let r = a * a;
        if r >= m {
            r % m
        } else {
            r
        }
    } else {
        mod_mul_(a, a, m)
    }
}

fn mod_exp(mut x: u64, mut d: u64, n: u64) -> u64 {
    let mut ret: u64 = 1;
    while d != 0 {
        if d % 2 == 1 {
            ret = mod_mul(ret, x, n)
        }
        d /= 2;
        x = mod_sqr(x, n);
    }
    ret
}

/// Test if `n` is prime, using the deterministic version of the
/// Miller-Rabin test.
///
/// Doing a lot of primality tests with numbers strictly below some
/// upper bound will be faster using the `is_prime` method of a
/// `Sieve` instance.
///
/// # Examples
///
/// ```rust
/// assert_eq!(primal::is_prime(1), false);
/// assert_eq!(primal::is_prime(2), true);
/// assert_eq!(primal::is_prime(3), true);
/// assert_eq!(primal::is_prime(4), false);
/// assert_eq!(primal::is_prime(5), true);
///
/// assert_eq!(primal::is_prime(22_801_763_487), false);
/// assert_eq!(primal::is_prime(22_801_763_489), true);
/// assert_eq!(primal::is_prime(22_801_763_491), false);
/// ```
pub fn miller_rabin(n: u64) -> bool {
    const HINT: &[u64] = &[2];

    // we have a strict upper bound, so we can just use the witness
    // table of Pomerance, Selfridge & Wagstaff and Jeaschke to be as
    // efficient as possible, without having to fall back to
    // randomness.
    const WITNESSES: &[(u64, &[u64])] =
        &[(2_046, HINT),
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
    let mut s = 0;
    while d % 2 == 0 { d /= 2; s += 1 }

    let witnesses =
        WITNESSES.iter().find(|&&(hi, _)| hi >= n)
            .map(|&(_, wtnss)| wtnss).unwrap();
    'next_witness: for &a in witnesses.iter() {
        let mut power = mod_exp(a, d, n);
        assert!(power < n);
        if power == 1 || power == n - 1 { continue 'next_witness }

        for _r in 0..s {
            power = mod_sqr(power, n);
            assert!(power < n);
            if power == 1 { return false }
            if power == n - 1 {
                continue 'next_witness
            }
        }
        return false
    }

    true
}

#[cfg(test)]
mod tests {
    use primal::Sieve;

    #[test]
    fn modulo() {
        for i in 0..64 {
            let x = 1 << i;
            for j in 0..10 {
                let m = 11 + j * 2;
                assert_eq!(super::modulo(super::U128 { lo: x, hi: 0}, m), x % m);
            }
        }

        let x = 1 << 63;
        assert_eq!(super::modulo(super::U128 { lo: 0, hi: x}, x + 1), 2);
        assert_eq!(super::modulo(super::U128 { lo: 0, hi: x}, x + 3), 18);
        assert_eq!(super::modulo(super::U128 { lo: 0, hi: x}, x + 5), 50);
    }

    #[test]
    fn mul() {
        for i in 1..64 {
            for j in 1..64 {
                let res = super::mul128((1 << i) + 1, (1 << j) + 1);
                let shift = i + j;
                let high_twiddle = i == 63 && j == 63;
                let mut real = if shift >= 64 {
                    super::U128 { lo: 0, hi: (1 << (shift - 64)) + high_twiddle as u64 }
                } else {
                    super::U128 { lo: 1 << shift, hi: 0 }
                };
                real.lo += if high_twiddle { 0 } else { (1 << i) + (1 << j) } + 1;
                assert!(res == real,
                        "(2**{} + 1)*(2**{} + 1): {:?} should be {:?}",
                        i, j, res, real);
            }
        }
    }
    #[test]
    fn mod_mul() {
        assert_eq!(super::mod_mul(1 << 63, 1 << 32, 3), 2);
        assert_eq!(super::mod_mul(1 << 31, 1 << 31, (1 << 32) - 7), 3221225479);
        assert_eq!(super::mod_mul(1 << 32, 1 << 32, (1 << 32) - 7), 49);
        assert_eq!(super::mod_mul(1 << 32, 1 << 32, (1 << 32) + 7), 49);
        assert_eq!(super::mod_mul(1 << 63, 1 << 32, (1 << 32) + 7), 2_147_483_480);
        assert_eq!(super::mod_mul(1 << 63, 1 << 32, (1 << 63) + 7), 9_223_372_006_790_004_743);
        assert_eq!(super::mod_mul(1 << 32, 1 << 32, !0), 1);
    }

    #[test]
    fn miller_rabin() {
        const LIMIT: usize = 1_000_000;
        let sieve = Sieve::new(LIMIT);
        for x in 0..LIMIT {
            let s = sieve.is_prime(x);
            let mr = super::miller_rabin(x as u64);

            assert!(s == mr, "miller_rabin {} mismatches sieve {} for {}",
                    mr, s, x)
        }
    }
    #[test]
    fn miller_rabin_large() {
        let tests = &[
            (4_294_967_311, true),
            (4_294_967_291, true),
            (4_294_967_291 * 4_294_967_291, false),
            (!0, false),
            ];
        for &(n, is_prime) in tests {
            assert!(super::miller_rabin(n) == is_prime,
                    "mismatch for {} (should be {})", n, is_prime);
        }
    }
}

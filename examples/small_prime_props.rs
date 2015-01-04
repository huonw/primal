#![feature(macro_rules)]

extern crate slow_primes;

use std::cmp;

// prints two Rust statics:
//
// - `SMALL_PRIME_PI` such that `SMALL_PRIME_PI[n] == π(n)`.
// - `SMALL_PRIMES` listing the primes below some small bound

static LINE_LIMIT: uint = 90;

static LARGEST_PI: u64 = 255;
static LARGEST_PRIME: u64 = 255;

fn u_type(max: u64) -> String {
    format!("u{}", match () {
        _ if max < 1 << 8 => 8u,
        _ if max < 1 << 16 => 16,
        _ if max < 1 << 32 => 32,
        _ => 64
    })
}

fn main() {
    let (_, hi) = slow_primes::estimate_nth_prime(LARGEST_PI + 1);
    // we know `hi` is definitely above one prime we're interested in,
    // so just make sure we're above the other one.
    let sieve = slow_primes::Primes::sieve(cmp::max(hi, LARGEST_PRIME) as uint);

    let stop_at = sieve.primes().nth(LARGEST_PI as uint).unwrap();

    println!("// created with small_prime_props.rs");
    print!("pub const SMALL_PRIME_PI: [{}; {}] = [", u_type(LARGEST_PI), stop_at);
    let mut width = LINE_LIMIT;

    macro_rules! check_width {
        ($next_len: expr) => {
            if width + $next_len >= LINE_LIMIT {
                print!("\n    ");
                width = 4;
                false
            } else {
                true
            }
        }
    }
    let mut last = 0;

    for (pi, p) in sieve.primes().enumerate() {
        let text = format!("{},", pi);
        let len = text.len();
        let mut new_prime = true;

        for _ in range(last, p) {
            if check_width!(1 + len) && !new_prime {
                print!(" ");
                width += 1
            }
            print!("{}", text);
            width += len;
            new_prime = false;
        }

        last = p;
        if pi as u64 >= LARGEST_PI { break }
        let p_text = format!("/*{}*/", p);
        if check_width!(1 + p_text.len()) {
            print!(" ");
            width += 1;
        }
        print!("{}", p_text);
        width += p_text.len();
    }

    println!("];");

    let count = sieve.primes().take_while(|p| *p as u64 <= LARGEST_PRIME).count();
    println!("pub const SMALL_PRIME_LIMIT: uint = {};", LARGEST_PRIME);
    print!("pub const SMALL_PRIMES: [{}, .. {}] = [", u_type(LARGEST_PRIME), count);
    width = LINE_LIMIT;

    for p in sieve.primes().take_while(|p| *p as u64 <= LARGEST_PRIME) {
        let text = format!("{},", p);
        if check_width!(1 + text.len()) {
            print!(" ");
            width += 1;
        }
        print!("{}", text);
        width += text.len();
    }
    println!("];")
}

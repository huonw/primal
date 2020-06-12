extern crate primal;
use std::env;
use std::time::Instant;

fn main() {
    let mut args = env::args();
    let max = args
        .nth(1).and_then(|s| s.parse::<f64>().ok().map(|x| x as usize))
        .unwrap_or(1_000_000);

    let start = Instant::now();
    let sieve = primal::Sieve::new(max);
    let time_sieve = start.elapsed();

    let start = Instant::now();
    let count = sieve.prime_pi(max);
    let time_prime_pi = start.elapsed();

    println!("{} primes below {} (est: {:?})\nsieve {:?}, prime_pi {:?}, total: {:?}",
             count, max, primal::estimate_prime_pi(max as u64),
             time_sieve, time_prime_pi, time_sieve + time_prime_pi);
}

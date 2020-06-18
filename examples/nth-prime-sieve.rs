use std::env;
use std::time::Instant;

fn main() {
    let mut args = env::args();
    let n = args
        .nth(1).and_then(|s| s.parse::<f64>().ok().map(|x| x as usize))
        .unwrap_or(1_000_000);

    let start = Instant::now();
    let (_, hi) = primal::estimate_nth_prime(n as u64);
    let sieve = primal::Sieve::new(hi as usize);
    let time_sieve = start.elapsed();

    let start = Instant::now();
    let prime = sieve.nth_prime(n);
    let time_nth_prime = start.elapsed();

    println!("{}th prime is {} (est: {:?})\nsieve {:?}, nth-prime {:?}, total: {:?}",
             n, prime, primal::estimate_nth_prime(n as u64),
             time_sieve, time_nth_prime, time_sieve + time_nth_prime);
}

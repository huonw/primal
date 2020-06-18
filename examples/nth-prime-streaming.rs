use std::env;
use std::time::Instant;

fn main() {
    let mut args = env::args();
    let n = args
        .nth(1).and_then(|s| s.parse::<f64>().ok().map(|x| x as usize))
        .unwrap_or(1_000_000);

    let start = Instant::now();
    let prime = primal::StreamingSieve::nth_prime(n);
    let time = start.elapsed();

    println!("{}th prime is {} (est: {:?})\ntotal: {:?}",
             n, prime, primal::estimate_nth_prime(n as u64), time);
}

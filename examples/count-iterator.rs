use std::env;
use std::time::Instant;

fn main() {
    let mut args = env::args();
    let max = args
        .nth(1).and_then(|s| s.parse::<f64>().ok().map(|x| x as usize))
        .unwrap_or(1_000_000);

    let start = Instant::now();
    let count = primal::Primes::all().take_while(|x| *x <= max).count();
    let time = start.elapsed();

    println!("{} primes below {} in {:?} (est: {:?})",
             count, max, time, primal::estimate_prime_pi(max as u64));
}

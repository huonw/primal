extern crate primal;
extern crate time;
use std::env;
use time::Duration;

fn fmt_time(time: &Duration) -> String {
    let ns = time.num_nanoseconds().unwrap();
    let (s, ns) = (ns / 1_000_000_000, ns % 1_000_000_000);
    format!("{}.{:06}", s, ns / 1000)
}

fn main() {
    let mut args = env::args();
    let max = args
        .nth(1).and_then(|s| s.parse::<f64>().ok().map(|x| x as usize))
        .unwrap_or(1_000_000);

    let mut count = None;
    let mut sieve = None;
    let time_sieve = Duration::span(|| {
        sieve = Some(primal::Sieve::new(max))
    });
    let time_prime_pi = Duration::span(|| {
        count = Some(sieve.unwrap().prime_pi(max))
    });

    println!("{} primes below {} (est: {:?})\nsieve {}s, prime_pi {}s, total: {}s",
             count.unwrap(),
             max,
             primal::estimate_prime_pi(max as u64),
             fmt_time(&time_sieve),
             fmt_time(&time_prime_pi),
             fmt_time(&(time_sieve + time_prime_pi)));
}

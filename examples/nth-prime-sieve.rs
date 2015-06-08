extern crate time;
extern crate primal;
use std::env;
use time::Duration;

fn fmt_time(time: &Duration) -> String {
    let ns = time.num_nanoseconds().unwrap();
    let (s, ns) = (ns / 1_000_000_000, ns % 1_000_000_000);
    format!("{}.{:06}", s, ns / 1000)
}

fn main() {
    let mut args = env::args();
    let n = args
        .nth(1).and_then(|s| s.parse::<f64>().ok().map(|x| x as usize))
        .unwrap_or(1_000_000);

    let mut prime = None;
    let mut sieve = None;
    let time_sieve = Duration::span(|| {
        let (_, hi) = primal::estimate_nth_prime(n as u64);
        sieve = Some(primal::Sieve::new(hi as usize))
    });
    let time_nth_prime = Duration::span(|| {
        prime = Some(sieve.unwrap().nth_prime(n))
    });

    println!("{}th prime is {} (est: {:?})\nsieve {}s, nth-prime {}s, total: {}s",
             n,
             prime.unwrap(),
             primal::estimate_nth_prime(n as u64),
             fmt_time(&time_sieve),
             fmt_time(&time_nth_prime),
             fmt_time(&(time_sieve + time_nth_prime)));
}

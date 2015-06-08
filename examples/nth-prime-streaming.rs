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
    let time = Duration::span(|| {
        prime = Some(primal::StreamingSieve::nth_prime(n))
    });

    println!("{}th prime is {} (est: {:?})\ntotal: {}s",
             n,
             prime.unwrap(),
             primal::estimate_nth_prime(n as u64),
             fmt_time(&time));
}

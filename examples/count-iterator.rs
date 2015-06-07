extern crate primal;
extern crate time;
use std::env;
use time::Duration;

fn main() {
    let mut args = env::args();
    let max = args
        .nth(1).and_then(|s| s.parse::<f64>().ok().map(|x| x as usize))
        .unwrap_or(1_000_000);

    let mut count = None;
    let time = Duration::span(|| {
        count = Some(primal::Primes::all().take_while(|x| *x <= max).count())
    });

    let ns = time.num_nanoseconds().unwrap();
    let (s, ns) = (ns / 1_000_000_000, ns % 1_000_000_000);
    let time = format!("{}.{:06}", s, ns / 1000);
    println!("{} primes below {} in {}s (est: {:?})",
             count.unwrap(),
             max,
             time,
             primal::estimate_prime_pi(max as u64));
}

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
    let zeros = args
        .nth(1).and_then(|s| s.parse::<f64>().ok().map(|x| x as u32))
        .unwrap_or(10);

    let mut p = None;
    let time = Duration::span(|| {
        p = Some(primal::Primes::all().find(|p| (p / 2).trailing_zeros() >= zeros).unwrap())
    });

    println!("{} is the first prime with {} zeros, in {}s",
             p.unwrap(),
             zeros,
             fmt_time(&time));
}

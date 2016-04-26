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
        let mut low = 1 << (zeros + 1);
        loop {
            println!("searching {}--{}", low, low * 2);
            let sieve = primal::Sieve::new(low * 2);
            match sieve.primes_from(low).find(|p| (p / 2).trailing_zeros() >= zeros) {
                Some(p_) => { p = Some(p_); break }
                None => {}
            }
            low *= 2;
        }
    });

    println!("{} is the first prime with {} zeros, in {}s",
             p.unwrap(),
             zeros,
             fmt_time(&time));
}

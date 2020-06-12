extern crate primal;
use std::env;
use std::time::Instant;

fn main() {
    let mut args = env::args();
    let zeros = args
        .nth(1).and_then(|s| s.parse::<f64>().ok().map(|x| x as u32))
        .unwrap_or(10);

    let start = Instant::now();
    let mut low = 1 << (zeros + 1);
    let p = loop {
        println!("searching {}--{}", low, low * 2);
        let sieve = primal::Sieve::new(low * 2);
        match sieve.primes_from(low).find(|p| (p / 2).trailing_zeros() >= zeros) {
            Some(p) => break p,
            None => {}
        }
        low *= 2;
    };
    let time = start.elapsed();

    println!("{} is the first prime with {} zeros, in {:?}",
             p, zeros, time);
}

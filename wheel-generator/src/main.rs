extern crate primal_smallsieve;

use std::io::prelude::*;
use std::io;
use std::collections::BTreeMap;

macro_rules! errln {
    ($($fmt: tt)*) => {
        (writeln!(&mut io::stderr(), $($fmt)*).unwrap())
    }
}

fn gcd(x: usize, y: usize) -> usize {
    if y == 0 { x }
    else { gcd(y, x % y) }
}
fn coprime_to(x: usize, upto: usize) -> Vec<usize> {
    (1..upto).filter(|&n| gcd(n, x) == 1).collect()
}

const LIMIT: usize = 2_000_000;

const WHEEL: usize = 2 * 3 * 5 * 7;
const COUNT: usize = 1 * 2 * 4 * 6;
const START_AT: usize = 11;

fn main() {
    errln!("wheel for {} (count {})", WHEEL, COUNT);
    let sieve = primal_smallsieve::Primes::sieve(LIMIT);

    let coprime = coprime_to(WHEEL, LIMIT).into_iter().enumerate().collect::<Vec<_>>();

    let mut map = BTreeMap::new();

    for p in sieve.primes().filter(|&p| p >= START_AT).take(1000) {
        //println!("prime {} mod {}.", p, p % WHEEL);
        let sq = p * p;
        if sq >= LIMIT {
            break
        }
        let approx = sq / WHEEL * COUNT;
        assert!(approx <= coprime.len());
        assert!(coprime[approx].1 <= sq);
        let bits = coprime[approx..].iter()
                          .filter(|&&(_, x)| x >= sq && x % p == 0)
                          .take(100)
                          .map(|&(i, _)| i)
                          .collect::<Vec<_>>();
        let bit_diffs = bits.iter().zip(&bits[1..]).map(|(&b1, &b2)| b2 - b1).collect::<Vec<_>>();
        map.entry(p % WHEEL).or_insert(vec![]).push((p, bits, bit_diffs));
    }

    for (m, bitss) in &map {
        println!("    // remainder {}", m);
        assert!(bitss.len() >= 2);
        let (p1, _, ref bits1) = bitss[0];
        let (p2, _, ref bits2) = bitss[1];

        let x1 = p1 / WHEEL;
        let x2 = p2 / WHEEL;
        let xdiff = x2 - x1;
        //println!("{} {} {}", x1, x2, xdiff);
        let lines = bits1.iter().zip(bits2).map(|(y1, y2)| {
            let ydiff = y2 - y1;
            assert!(ydiff % xdiff == 0);
            let slope = ydiff / xdiff;
            //println!("\t{} {} {}", y1, y2, ydiff);
            //println!("\t\t{}", slope);
            let shift = y1 - slope * x1;
            (slope, shift)
        }).collect::<Vec<_>>();

        // verify the lines are true
        for &(p, _, ref bits) in bitss {
            let x = p / WHEEL;
            for (&b, &(sl, sh)) in bits.iter().zip(&lines) {
                assert!(sl * x + sh == b, "{} {}", p, b);
            }
        }

        for start_bit in 0..8 {
            let mut bit = start_bit;
            let twiddles = lines[..COUNT].iter().map(|&(sl, sh)| {
                assert!(sl % 8 == 0);
                let sl = sl / 8;
                let old_bit = bit;
                let new_bit = bit + sh;
                let offset = new_bit / 8;
                bit = new_bit % 8;
                (sl, offset, old_bit)
            });

            print!("    ");
            for (i, (sl, offset, bit)) in twiddles.enumerate() {
                print!("elem!({}u8,{},{},{}),", bit, sl, offset,
                       if i == COUNT-1 {-(i as isize)}else{1});
                if i % 4 == 3 {
                    print!("\n        ")
                }
            }
            println!("");
        }
    }
}

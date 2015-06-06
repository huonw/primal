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

fn bit_index(x: usize) -> usize {
    let wheel = x / BYTE_WHEEL * BYTE_COUNT;
    let rem = x % BYTE_WHEEL;
    let bits = (1..rem).filter(|y| gcd(*y, BYTE_WHEEL) == 1).count();
    wheel + bits
}

const LIMIT: usize = 2_000_000;

const BYTE_WHEEL: usize = 2 * 3 * 5;
const BYTE_COUNT: usize = 1 * 2 * 4;

#[cfg(not(feature = "thirty"))]
const WHEEL: usize = 2 * 3 * 5 * 7;
#[cfg(not(feature = "thirty"))]
const COUNT: usize = 1 * 2 * 4 * 6;
#[cfg(not(feature = "thirty"))]
const START_AT: usize = 11;

#[cfg(feature = "thirty")]
const WHEEL: usize = 2 * 3 * 5;
#[cfg(feature = "thirty")]
const COUNT: usize = 1 * 2 * 4;
#[cfg(feature = "thirty")]
const START_AT: usize = 7;

fn main() {
    errln!("wheel for {} (count {})", WHEEL, COUNT);
    let sieve = primal_smallsieve::Primes::sieve(LIMIT);

    let coprime = coprime_to(BYTE_WHEEL, LIMIT).into_iter().enumerate().collect::<Vec<_>>();

    let mut map = BTreeMap::new();

    for p in sieve.primes().filter(|&p| p >= START_AT).take(1000) {
        //println!("prime {} mod {}.", p, p % WHEEL);
        let sq = p * p;
        if sq >= LIMIT {
            break
        }
        let approx = sq / BYTE_WHEEL * BYTE_COUNT;
        assert!(approx <= coprime.len());
        assert!(coprime[approx].1 <= sq);
        let bits = coprime[approx..].iter()
                          .filter(|&&(_, x)| x >= sq && x % p == 0)
                          .take(100)
                          .map(|&(i, _)| i)
                          .collect::<Vec<_>>();
        let bit_diffs = bits.iter().zip(&bits[1..]).map(|(&b1, &b2)| b2 - b1).collect::<Vec<_>>();
        map.entry(p % BYTE_WHEEL).or_insert(vec![]).push((p, bits, bit_diffs));
    }

    println!("// automatically generated
use wheel::{{WheelInit, Wheel, WheelElem}};

#[derive(Debug)]
pub struct Wheel{modulo};
impl Wheel for Wheel{modulo} {{
    #[inline(always)]
    fn modulo(&self) -> usize {{ MODULO }}

    #[inline(always)]
    fn size(&self) -> usize {{ SIZE }}

    #[inline(always)]
    fn wheel(&self) -> &'static [WheelElem] {{ WHEEL }}

    #[inline(always)]
    fn init(&self) -> &'static [WheelInit] {{ INIT }}

    #[inline(always)]
    unsafe fn hardcoded_sieve(&self,
                              bytes: &mut [u8], si_: &mut usize, wi_: &mut usize, prime: usize) {{
        hardcoded_sieve(bytes, si_, wi_, prime)
    }}
}}

pub const SIZE: usize = {size};

pub const MODULO: usize = {modulo};
",
             size = COUNT,
             modulo = WHEEL);

    println!("const INIT: &'static [WheelInit; {}] = &[", WHEEL);
    let mut next = 0;
    for (i, &y) in coprime_to(WHEEL, WHEEL).iter().enumerate() {
        for x in next..y + 1 {
            println!("    WheelInit {{ next_mult_factor: {}, wheel_index: {} }}, // {}",
                     y - x, i, x)
        }
        next = y + 1;
    }
    println!("];");

    let mut all_twiddles = vec![];
    println!("const WHEEL: &'static [WheelElem; {}] = &[", BYTE_COUNT * COUNT);
    for (m, bitss) in &map {
        println!("    // remainder {}", m);
        assert!(bitss.len() >= 2);
        let (p1, _, ref bits1) = bitss[0];
        let (p2, _, ref bits2) = bitss[1];

        let x1 = p1 / BYTE_WHEEL;
        let x2 = p2 / BYTE_WHEEL;
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
            let x = p / BYTE_WHEEL;
            for (&b, &(sl, sh)) in bits.iter().zip(&lines) {
                assert!(sl * x + sh == b, "{} {}", p, b);
            }
        }

        let start_bit = bit_index(p1 * p1) % 8;
        assert_eq!(bit_index(p2 * p2) % 8, start_bit);
        let mut bit = start_bit;
        let twiddles: Vec<_> = lines[..COUNT].iter().map(|&(sl, sh)| {
            assert_eq!(sl % 8, 0);
            let sl = sl / 8;
            let old_bit = bit;
            let new_bit = bit + sh;
            let offset = new_bit / 8;
            bit = new_bit % 8;
            (sl, offset, old_bit)
        }).collect();

        for (i, &(sl, offset, bit)) in twiddles.iter().enumerate() {
            println!("    WheelElem {{ unset_bit: 1u8 << {}u8, next_mult_factor: {}, correction: {}, next: {} }},", bit, sl, offset,
                   if i == COUNT-1 {-(i as isize)}else{1});
        }
        all_twiddles.push((m, twiddles));
    }
    println!("];");

    let big_slope = all_twiddles[0].1[..COUNT-1].iter().fold(0, |a, &(sl, _offset, _)| a + sl);
    let big_step = all_twiddles[BYTE_COUNT - 1].1[..COUNT-1].iter().fold(0, |a, &(_sl, offset, _)| a + offset);
    println!("\
pub unsafe fn hardcoded_sieve(bytes: &mut [u8], si_: &mut usize, wi_: &mut usize, prime: usize) {{
    let bytes = bytes;
    let start = bytes.as_mut_ptr();
    let len = bytes.len() as isize;
    let largest_step = ({big_slope} * prime + {big_step}) as isize;
    let loop_len = len - largest_step;
    let loop_end = start.offset(loop_len);
    let end = start.offset(len);
    let si = *si_ as isize;
    let mut p = start.offset(si);
    let mut wi = *wi_;
    let prime_ = prime as isize;

    'outer: loop {{
    match wi {{",
             big_slope = big_slope,
             big_step = big_step);
    for (i, &(m, ref twiddles)) in all_twiddles.iter().enumerate() {
        let wheel_start = i * COUNT;
        let wheel_end = (i + 1) * COUNT;
        println!("        {}...{} => {{ // {} * x + {}",
                 wheel_start, wheel_end - 1,
                 BYTE_WHEEL, m);
        let mut indent: String = "            ".into();
        println!("\
{indent}loop {{",
                     indent = indent);
        for j in (0..COUNT).rev() {
            indent.push_str(" ");
            println!("{}'label{}: loop {{", indent, wheel_start + j);
        }

        println!("\
{indent} match wi {{", indent = indent);
        for j in 0..COUNT - 1 {
            println!("{}  {1} => break 'label{1},", indent, wheel_start + j);
        }
        println!("{}  _ => break 'label{},", indent, wheel_start + COUNT - 1);
        println!("{} }}", indent);
        println!("{}}}", indent);
        println!("\
{indent}while p < loop_end {{
{indent}    p = ::b(p);",
                 indent = indent);

        let mut sl_so_far = 0;
        let mut offset_so_far = 0;
        for &(sl, offset, bit) in twiddles {
            println!("\
{indent}    *p.offset(prime_ * {} + {}) |= {};",
                     sl_so_far, offset_so_far, 1 << bit, indent = indent);
            sl_so_far += sl;
            offset_so_far += offset;
        }
        println!("
{indent}    p = p.offset(prime_ * {} + {})
{indent}}}",
                 sl_so_far, offset_so_far,
                 indent = indent);

        for (j, &(sl, offset, bit)) in twiddles.iter().enumerate() {
            indent.pop();
            let end = if j + 1 == COUNT {
                format!("wi = {}", wheel_start)
            } else {
                format!("break 'label{}", wheel_start + j + 1)
            };
            println!("\
{indent} if p >= end {{ wi = {val}; break 'outer; }}
{indent} *p |= {}; p = p.offset(prime_ * {} + {});
{indent} {}
{indent}}}",

                     1 << bit,
                     sl, offset,
                     end,
                     val = wheel_start + j,
                     indent = indent);
        }
        println!("        }}");
    }
    println!("        _ => unreachable!(\"{{}}\", wi),
    }}
    }}
    *si_ = (p as usize).wrapping_sub(end as usize);
    *wi_ = wi;
}}");
}

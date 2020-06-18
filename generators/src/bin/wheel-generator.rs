use std::collections::{BTreeMap, HashSet};
use std::env;

fn div_up(x: usize, y: usize) -> usize {
    (x + y - 1) / y
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
fn from_bit_index(x: usize, byte_coprime: &Vec<usize>) -> usize {
    let (byte, bit) = (x / BYTE_COUNT, x % BYTE_COUNT);
    byte * BYTE_WHEEL + byte_coprime[bit]
}

const BYTE_WHEEL: usize = 2 * 3 * 5;
const BYTE_COUNT: usize = 1 * 2 * 4;

const SMALL_LIMIT: usize = 10_000;

fn main() {
    let primes = env::args()
        .skip(1)
        .map(|x| x.parse::<usize>().expect(&format!("could not parse `{}` as a positive integer", x)))
        .collect::<Vec<_>>();

    assert!(primes.len() > 0, "need at least one prime to make a wheel");
    for p in &primes {
        assert!(primal_check::miller_rabin(*p as u64), "{} is not prime", p);
    }
    assert!(primes.len() == primes.iter().collect::<HashSet<_>>().len(),
            "some primes were non-unique");

    let wheel = primes.iter().fold(1, |x, y| x * *y);
    let count = primes.iter().fold(1, |x, y| x * (*y - 1));

    eprintln!("wheel for {} (count {})", wheel, count);
    let byte_coprime = coprime_to(BYTE_WHEEL, BYTE_WHEEL);
    let coprime = coprime_to(wheel, wheel);
    let mut diff_to_next = BTreeMap::new();
    for (i, &c) in coprime.iter().enumerate() {
        let next = coprime[(i + 1) % coprime.len()];
        diff_to_next.insert(c, ((wheel + next) - c) % wheel);
    }

    // The main workhorse.
    //
    // The sieve is driven by crossing off things that are definitely
    // not prime, and performance is driven by minimising how much
    // crossing off happens.
    //
    // Suppose we have a wheel of size 30 = 2 * 3 * 5. Given a prime p
    // = 30n + i, there's only 8 possible values of i: C = {1, 7, 11,
    // 13, 17, 19, 23, 29}. As is standard, we know that all values
    // below p * p will have already been crossed off by the time
    // we're crossing off with p, so we only need to start
    // there... but we can do better. The idea behind the wheel also
    // implies that {p * (30k + 0), p * (30k + 2), p * (30k + 3), ...,
    // (30k + 28)} will have been crossed off already (for all k), in
    // particular, we only need to consider multiplication by 30k + j
    // for j in C.
    //
    // If we have some multiple m = q * p, we need to work out how to
    // get to the next one, m'. One possibility would be to literally
    // store q and then find the next q' (next number congruent mod 30
    // to an element of C), but we can do better (common theme, huh?):
    // m' = m + (j' - j) * p, so just the difference between
    // successive j's is needed (this difference is diff_mult_factor
    // in the following table). These differences are of course cycle
    // with period |C| = 8, conveniently.
    //
    // The last really tricky part is we want to address and
    // manipulate individual bits efficiently, which means storing
    // byte indices and precomputing appropriate bitmasks. This
    // essentially means we divide p and m by 30, rounding down, and
    // hence lose information in two ways: we don't know were to
    // write, nor can we compute the exact right byte
    // automatically. The former is managed by bitmasks that change
    // with j, and the latter is managed by "corrections". Corrections
    // are additive factors that are used like [m'/30] = [m/30] + (j'
    // - j) *[p/30] + correction, where [x] == floor(x). Both these
    // management tools are cyclic with period 8.
    //
    // (Another slight complication is that we can use a computation
    // wheel that is different to the in-memory wheel, which means we
    // skip over more writes by taking bigger steps, but don't get the
    // memory savings.)
    #[derive(Debug)]
    struct Info {
        total_mult_factor: usize,
        diff_mult_factor: usize,
        total_add_factor: usize,
        correction: usize,
        unset_bit: u8,
    }
    let infos = byte_coprime.iter()
        .map(|c| {
            coprime.iter()
                .map(|&cc| {
                    let total_add_factor = c * cc;
                    let diff = diff_to_next[&cc];
                    Info {
                        total_mult_factor: cc,
                        total_add_factor,
                        diff_mult_factor: diff,
                        correction: (total_add_factor + diff * c) / BYTE_WHEEL - total_add_factor / BYTE_WHEEL,
                        unset_bit: !(1 << (bit_index(total_add_factor) % 8))
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();


    println!("// automatically generated
use crate::wheel::{{WheelInit, Wheel, WheelElem}};

#[derive(Debug, Clone)]
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
             size = count,
             modulo = wheel);

    let length_bytes = div_up(SMALL_LIMIT * count, wheel * 8);
    let length_bits = length_bytes * 8;
    let sieve = primal_slowsieve::Primes::sieve(length_bits * wheel / count);
    print!("\
#[allow(dead_code)]
pub const SMALL_BITS: usize = {};
#[allow(dead_code)]
pub const SMALL: &'static [u8; SMALL_BITS / 8] = &[", length_bits);
    let mut bit = 0;
    // precompute the sieve for a little while.
    for byte in 0..length_bytes {
        let mut val = 0;
        for i in 0..8 {
            val |= (sieve.is_prime(from_bit_index(bit, &byte_coprime)) as u8) << i;
            bit += 1;
        }
        if byte % 8 == 0 {
            print!("\n    0b{:08b},", val);
        } else {
            print!(" 0b{:08b},", val);
        }
    }
    println!();
    println!("];");

    // compute the initialisers: push an arbitrary number up to the
    // next one that isn't eliminated by the wheel, indicating which
    // multiple it is (for indexing into the next one, which is
    // ordered by multiple)
    println!("const INIT: &'static [WheelInit; {}] = &[", wheel);
    let mut next = 0;
    for (i, &y) in coprime_to(wheel, wheel).iter().enumerate() {
        for x in next..y + 1 {
            println!("    WheelInit {{ next_mult_factor: {}, wheel_index: {} }}, // {}",
                     y - x, i, x)
        }
        next = y + 1;
    }
    println!("];");

    // now print the full wheel!
    println!("const WHEEL: &'static [WheelElem; {}] = &[", BYTE_COUNT * count);
    for (c, cur_infos) in byte_coprime.iter().zip(&infos) {
        println!("    // remainder {}", c);

        for (i, info) in cur_infos.iter().enumerate() {
            println!("    WheelElem {{ unset_bit: {}, next_mult_factor: {}, correction: {}, next: {} }},",
                     info.unset_bit,
                     info.diff_mult_factor,
                     info.correction,
                     if i == count - 1 { -(i as isize) } else { 1 })
        }
    }
    println!("];");

    // the hard-coded sieve is a code version of WHEEL, designed for
    // cases when `prime` is small and hence a lot of writes are
    // peformed: loops can be unrolled.
    //
    // this (ab)uses labelled breaks to jump around more efficiently ala goto.
    println!("\
pub unsafe fn hardcoded_sieve(bytes: &mut [u8], si_: &mut usize, wi_: &mut usize, prime: usize) {{
    let bytes = bytes;
    let start = bytes.as_mut_ptr();
    let len = bytes.len() as isize;
    let largest_step = ::std::cmp::min(len, {wheel} * (prime as isize + 1) - 1);
    let loop_len = len - largest_step;
    let loop_end = start.offset(loop_len);
    let end = start.offset(len);
    let si = *si_ as isize;
    let mut p = start.offset(si);
    let mut wi = *wi_;
    let prime_ = prime as isize;

    'outer: loop {{
    match wi {{",
             wheel = wheel);
    for (i, (&c, cur_infos)) in coprime.iter().zip(&infos).enumerate() {
        let wheel_start = i * count;
        let wheel_end = (i + 1) * count;
        println!("        {}..={} => {{ // {} * x + {}",
                 wheel_start, wheel_end - 1,
                 BYTE_WHEEL, c);
        let mut indent: String = "            ".into();
        println!("\
{indent}loop {{",
                     indent = indent);
        for j in (0..count).rev() {
            indent.push_str(" ");
            println!("{}'label{}: loop {{", indent, wheel_start + j);
        }

        println!("\
{indent} match wi {{", indent = indent);
        for j in 0..count - 1 {
            println!("{}  {1} => break 'label{1},", indent, wheel_start + j);
        }
        println!("{}  _ => break 'label{},", indent, wheel_start + count - 1);
        println!("{} }}", indent);
        println!("{}}}", indent);
        println!("\
{indent}while p < loop_end {{
{indent}    p = crate::b(p);",
                 indent = indent);

        for info in cur_infos {
            println!("\
{indent}    safe_assert!(start <= p.offset(prime_ * {sl} + {off}) &&
{indent}                 p.offset(prime_ * {sl} + {off}) < end);
{indent}    *p.offset(prime_ * {sl} + {off}) &= {bit};",
                     // p starts at offset 1 * prime_, so strip off
                     // that factor.
                     sl = info.total_mult_factor - 1,
                     off = info.total_add_factor / wheel,
                     bit = info.unset_bit, indent = indent);
        }
        println!("
{indent}    p = p.offset(prime_ * {} + {})
{indent}}}",
                 wheel, c,
                 indent = indent);

        for (j, info) in cur_infos.iter().enumerate() {
            indent.pop();
            let end = if j + 1 == count {
                format!("wi = {}", wheel_start)
            } else {
                format!("break 'label{}", wheel_start + j + 1)
            };
            println!("\
{indent} if p >= end {{ wi = {val}; break 'outer; }}
{indent} safe_assert!(start <= p && p < end);
{indent} *p &= {}; p = p.offset(prime_ * {} + {});
{indent} {}
{indent}}}",

                     info.unset_bit, info.diff_mult_factor, info.correction,
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
}}")
}

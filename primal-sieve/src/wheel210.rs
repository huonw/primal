#![allow(dead_code)]

///
/// @file   WheelFactorization.cpp
/// @brief  Precomputed arrays for wheel factorization.
///
/// Copyright (C) 2013 Kim Walisch, <kim.walisch@gmail.com>
///
/// This file is distributed under the BSD License. See the COPYING
/// file in the top level directory.
///


#[derive(Debug)]
pub struct WheelInfo {
    pub true_prime: usize,
    pub prime: usize,
    pub wheel_index: usize,
    pub sieve_index: usize,
}
impl WheelInfo {
    pub fn bit(&self) -> usize {
        self.sieve_index * 8 + WHEEL[self.wheel_index].unset_bit.trailing_zeros() as usize
    }
    pub fn actual(&self) -> usize {
        let bit = self.bit();

        from_bit_index(bit)
    }
}

pub fn bit_index(n: usize) -> (bool, usize) {
    let init = &INIT[n % MODULO];
    (init.next_mult_factor == 0, (n / MODULO) * SIZE + init.wheel_index as usize)
}
pub fn from_bit_index(bit: usize) -> usize {
    (bit / SIZE) * MODULO + TRUE_AT_BIT[bit % SIZE]
}

pub fn set_bit(x: &mut [u8], si: &mut usize, wi: &mut usize, prime: usize) {
    unsafe {
        let WheelElem { unset_bit, next_mult_factor, correction, next } =
            *WHEEL.get_unchecked(*wi);
        *x.get_unchecked_mut(*si) |= unset_bit;

        *si += prime * next_mult_factor as usize;
        *si += correction as usize;
        *wi = wi.wrapping_add(next as usize);
    }
}

pub fn compute_wheel_elem(p: usize, low: usize) -> WheelInfo {
    let mut mult = p * p;

    let init = &INIT[p % MODULO];
    let next_mult_factor = init.next_mult_factor;
    mult += p * next_mult_factor as usize;

    let init2 = &INIT[mult % MODULO];
    let low_offset = mult - low;
    let subbit_base = init2.wheel_index as usize;

    let wheel_index = init.wheel_index as usize * SIZE;
    let sieve_index = low_offset / MODULO * SIZE / 8 + subbit_base / 8;

    let ret = WheelInfo {
        true_prime: p,
        prime: p / MODULO,
        sieve_index: sieve_index,
        wheel_index: wheel_index,
    };
    ret
}

pub const SIZE: usize = 48;
#[derive(Debug)]
pub struct WheelInit {
    pub next_mult_factor: u8,
    pub wheel_index: u8,
}
macro_rules! init {
    ($nmf: expr, $wi: expr) => { WheelInit { next_mult_factor: $nmf, wheel_index: $wi }}
}
#[derive(Debug)]
pub struct WheelElem {
    pub unset_bit: u8,
    pub next_mult_factor: u8,
    pub correction: u8,
    pub next: i8,
}
macro_rules! elem {
    ($bit: expr, $nmf: expr, $c: expr, $n: expr) => {
        WheelElem {
            unset_bit: 1u8 << $bit,
            next_mult_factor: $nmf,
            correction: $c,
            next: $n,
        }
    }
}

pub const MODULO: usize = 210;

const TRUE_AT_BIT: &'static [usize; 48] = &[
    1, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59,
    61, 67, 71, 73, 79, 83, 89, 97, 101, 103, 107, 109, 113,
    121, 127, 131, 137, 139, 143, 149, 151, 157, 163, 167, 169,
    173, 179, 181, 187, 191, 193, 197, 199, 209
    ];
const INIT: &'static [WheelInit; 210] = &[
  init!{1,  0}, init!{0,  0}, init!{9,  1}, init!{8,  1},
    init!{7,  1}, init!{6,  1}, init!{5,  1}, init!{4,  1},
  init!{3,  1}, init!{2,  1}, init!{1,  1}, init!{0,  1},
    init!{1,  2}, init!{0,  2}, init!{3,  3}, init!{2,  3},
  init!{1,  3}, init!{0,  3}, init!{1,  4}, init!{0,  4},
    init!{3,  5}, init!{2,  5}, init!{1,  5}, init!{0,  5},
  init!{5,  6}, init!{4,  6}, init!{3,  6}, init!{2,  6},
    init!{1,  6}, init!{0,  6}, init!{1,  7}, init!{0,  7},
  init!{5,  8}, init!{4,  8}, init!{3,  8}, init!{2,  8},
    init!{1,  8}, init!{0,  8}, init!{3,  9}, init!{2,  9},
  init!{1,  9}, init!{0,  9}, init!{1, 10}, init!{0, 10},
    init!{3, 11}, init!{2, 11}, init!{1, 11}, init!{0, 11},
  init!{5, 12}, init!{4, 12}, init!{3, 12}, init!{2, 12},
    init!{1, 12}, init!{0, 12}, init!{5, 13}, init!{4, 13},
  init!{3, 13}, init!{2, 13}, init!{1, 13}, init!{0, 13},
    init!{1, 14}, init!{0, 14}, init!{5, 15}, init!{4, 15},
  init!{3, 15}, init!{2, 15}, init!{1, 15}, init!{0, 15},
    init!{3, 16}, init!{2, 16}, init!{1, 16}, init!{0, 16},
  init!{1, 17}, init!{0, 17}, init!{5, 18}, init!{4, 18},
    init!{3, 18}, init!{2, 18}, init!{1, 18}, init!{0, 18},
  init!{3, 19}, init!{2, 19}, init!{1, 19}, init!{0, 19},
    init!{5, 20}, init!{4, 20}, init!{3, 20}, init!{2, 20},
  init!{1, 20}, init!{0, 20}, init!{7, 21}, init!{6, 21},
    init!{5, 21}, init!{4, 21}, init!{3, 21}, init!{2, 21},
  init!{1, 21}, init!{0, 21}, init!{3, 22}, init!{2, 22},
    init!{1, 22}, init!{0, 22}, init!{1, 23}, init!{0, 23},
  init!{3, 24}, init!{2, 24}, init!{1, 24}, init!{0, 24},
    init!{1, 25}, init!{0, 25}, init!{3, 26}, init!{2, 26},
  init!{1, 26}, init!{0, 26}, init!{7, 27}, init!{6, 27},
    init!{5, 27}, init!{4, 27}, init!{3, 27}, init!{2, 27},
  init!{1, 27}, init!{0, 27}, init!{5, 28}, init!{4, 28},
    init!{3, 28}, init!{2, 28}, init!{1, 28}, init!{0, 28},
  init!{3, 29}, init!{2, 29}, init!{1, 29}, init!{0, 29},
    init!{5, 30}, init!{4, 30}, init!{3, 30}, init!{2, 30},
  init!{1, 30}, init!{0, 30}, init!{1, 31}, init!{0, 31},
    init!{3, 32}, init!{2, 32}, init!{1, 32}, init!{0, 32},
  init!{5, 33}, init!{4, 33}, init!{3, 33}, init!{2, 33},
    init!{1, 33}, init!{0, 33}, init!{1, 34}, init!{0, 34},
  init!{5, 35}, init!{4, 35}, init!{3, 35}, init!{2, 35},
    init!{1, 35}, init!{0, 35}, init!{5, 36}, init!{4, 36},
  init!{3, 36}, init!{2, 36}, init!{1, 36}, init!{0, 36},
    init!{3, 37}, init!{2, 37}, init!{1, 37}, init!{0, 37},
  init!{1, 38}, init!{0, 38}, init!{3, 39}, init!{2, 39},
    init!{1, 39}, init!{0, 39}, init!{5, 40}, init!{4, 40},
  init!{3, 40}, init!{2, 40}, init!{1, 40}, init!{0, 40},
    init!{1, 41}, init!{0, 41}, init!{5, 42}, init!{4, 42},
  init!{3, 42}, init!{2, 42}, init!{1, 42}, init!{0, 42},
    init!{3, 43}, init!{2, 43}, init!{1, 43}, init!{0, 43},
  init!{1, 44}, init!{0, 44}, init!{3, 45}, init!{2, 45},
    init!{1, 45}, init!{0, 45}, init!{1, 46}, init!{0, 46},
  init!{9, 47}, init!{8, 47}, init!{7, 47}, init!{6, 47},
    init!{5, 47}, init!{4, 47}, init!{3, 47}, init!{2, 47},
  init!{1, 47}, init!{0, 47}
];

const WHEEL: &'static [WheelElem; 48*48] = &[
    // remainder 1
    elem!(0u8,60,0,1),elem!(1u8,12,0,1),elem!(2u8,24,0,1),elem!(3u8,12,0,1),
        elem!(4u8,24,0,1),elem!(5u8,36,0,1),elem!(6u8,12,0,1),elem!(7u8,36,1,1),
        elem!(0u8,24,0,1),elem!(1u8,12,0,1),elem!(2u8,24,0,1),elem!(3u8,36,0,1),
        elem!(4u8,36,0,1),elem!(5u8,12,0,1),elem!(6u8,36,0,1),elem!(7u8,24,1,1),
        elem!(0u8,12,0,1),elem!(1u8,36,0,1),elem!(2u8,24,0,1),elem!(3u8,36,0,1),
        elem!(4u8,48,0,1),elem!(5u8,24,0,1),elem!(6u8,12,0,1),elem!(7u8,24,1,1),
        elem!(0u8,12,0,1),elem!(1u8,24,0,1),elem!(2u8,48,0,1),elem!(3u8,36,0,1),
        elem!(4u8,24,0,1),elem!(5u8,36,0,1),elem!(6u8,12,0,1),elem!(7u8,24,1,1),
        elem!(0u8,36,0,1),elem!(1u8,12,0,1),elem!(2u8,36,0,1),elem!(3u8,36,0,1),
        elem!(4u8,24,0,1),elem!(5u8,12,0,1),elem!(6u8,24,0,1),elem!(7u8,36,1,1),
        elem!(0u8,12,0,1),elem!(1u8,36,0,1),elem!(2u8,24,0,1),elem!(3u8,12,0,1),
        elem!(4u8,24,0,1),elem!(5u8,12,0,1),elem!(6u8,60,0,1),elem!(7u8,12,1,-47),

    // remainder 11
    elem!(3u8,12,1,1),elem!(0u8,24,1,1),elem!(2u8,12,0,1),elem!(7u8,24,2,1),
        elem!(2u8,36,2,1),elem!(1u8,12,0,1),elem!(5u8,36,2,1),elem!(5u8,24,1,1),
        elem!(7u8,12,1,1),elem!(4u8,24,1,1),elem!(5u8,36,2,1),elem!(4u8,36,2,1),
        elem!(4u8,12,1,1),elem!(1u8,36,2,1),elem!(0u8,24,1,1),elem!(2u8,12,0,1),
        elem!(7u8,36,2,1),elem!(6u8,24,2,1),elem!(1u8,36,1,1),elem!(7u8,48,3,1),
        elem!(3u8,24,1,1),elem!(6u8,12,1,1),elem!(3u8,24,1,1),elem!(4u8,12,1,1),
        elem!(1u8,24,1,1),elem!(4u8,48,3,1),elem!(0u8,36,1,1),elem!(6u8,24,2,1),
        elem!(1u8,36,2,1),elem!(0u8,12,0,1),elem!(5u8,24,1,1),elem!(7u8,36,2,1),
        elem!(6u8,12,1,1),elem!(3u8,36,2,1),elem!(3u8,36,2,1),elem!(2u8,24,1,1),
        elem!(3u8,12,1,1),elem!(0u8,24,1,1),elem!(2u8,36,2,1),elem!(2u8,12,0,1),
        elem!(6u8,36,2,1),elem!(5u8,24,2,1),elem!(0u8,12,0,1),elem!(5u8,24,1,1),
        elem!(7u8,12,1,1),elem!(4u8,60,3,1),elem!(6u8,12,1,1),elem!(1u8,60,3,-47),

    // remainder 13
    elem!(6u8,24,2,1),elem!(1u8,12,1,1),elem!(0u8,24,1,1),elem!(4u8,36,2,1),
        elem!(5u8,12,1,1),elem!(4u8,36,2,1),elem!(6u8,24,2,1),elem!(2u8,12,0,1),
        elem!(7u8,24,2,1),elem!(3u8,36,2,1),elem!(5u8,36,2,1),elem!(6u8,12,1,1),
        elem!(4u8,36,2,1),elem!(7u8,24,2,1),elem!(3u8,12,1,1),elem!(1u8,36,2,1),
        elem!(2u8,24,1,1),elem!(6u8,36,3,1),elem!(0u8,48,3,1),elem!(0u8,24,1,1),
        elem!(4u8,12,1,1),elem!(2u8,24,1,1),elem!(5u8,12,1,1),elem!(3u8,24,1,1),
        elem!(7u8,48,3,1),elem!(7u8,36,3,1),elem!(1u8,24,1,1),elem!(5u8,36,2,1),
        elem!(6u8,12,1,1),elem!(4u8,24,2,1),elem!(0u8,36,2,1),elem!(3u8,12,1,1),
        elem!(1u8,36,2,1),elem!(2u8,36,2,1),elem!(4u8,24,2,1),elem!(0u8,12,0,1),
        elem!(5u8,24,2,1),elem!(1u8,36,2,1),elem!(3u8,12,1,1),elem!(2u8,36,2,1),
        elem!(3u8,24,1,1),elem!(7u8,12,1,1),elem!(6u8,24,2,1),elem!(1u8,12,0,1),
        elem!(7u8,60,4,1),elem!(5u8,12,1,1),elem!(2u8,60,4,1),elem!(0u8,12,0,-47),

    // remainder 17
    elem!(2u8,12,1,1),elem!(2u8,24,2,1),elem!(1u8,36,3,1),elem!(1u8,12,1,1),
        elem!(0u8,36,2,1),elem!(7u8,24,2,1),elem!(7u8,12,1,1),elem!(6u8,24,2,1),
        elem!(6u8,36,3,1),elem!(6u8,36,3,1),elem!(4u8,12,1,1),elem!(5u8,36,3,1),
        elem!(4u8,24,2,1),elem!(3u8,12,1,1),elem!(3u8,36,3,1),elem!(3u8,24,2,1),
        elem!(2u8,36,3,1),elem!(2u8,48,4,1),elem!(0u8,24,2,1),elem!(0u8,12,1,1),
        elem!(0u8,24,1,1),elem!(7u8,12,1,1),elem!(7u8,24,2,1),elem!(7u8,48,4,1),
        elem!(5u8,36,3,1),elem!(5u8,24,2,1),elem!(4u8,36,3,1),elem!(4u8,12,1,1),
        elem!(4u8,24,2,1),elem!(3u8,36,3,1),elem!(2u8,12,1,1),elem!(3u8,36,3,1),
        elem!(1u8,36,3,1),elem!(1u8,24,2,1),elem!(1u8,12,1,1),elem!(0u8,24,2,1),
        elem!(0u8,36,2,1),elem!(7u8,12,1,1),elem!(6u8,36,3,1),elem!(6u8,24,2,1),
        elem!(5u8,12,1,1),elem!(5u8,24,2,1),elem!(6u8,12,1,1),elem!(5u8,60,5,1),
        elem!(4u8,12,1,1),elem!(3u8,60,5,1),elem!(2u8,12,1,1),elem!(1u8,24,2,-47),

    // remainder 19
    elem!(2u8,24,2,1),elem!(3u8,36,3,1),elem!(5u8,12,1,1),elem!(6u8,36,4,1),
        elem!(1u8,24,2,1),elem!(1u8,12,1,1),elem!(2u8,24,2,1),elem!(4u8,36,3,1),
        elem!(5u8,36,4,1),elem!(0u8,12,1,1),elem!(1u8,36,3,1),elem!(2u8,24,2,1),
        elem!(4u8,12,1,1),elem!(4u8,36,3,1),elem!(7u8,24,3,1),elem!(0u8,36,3,1),
        elem!(1u8,48,4,1),elem!(4u8,24,2,1),elem!(6u8,12,1,1),elem!(7u8,24,3,1),
        elem!(0u8,12,1,1),elem!(1u8,24,2,1),elem!(3u8,48,4,1),elem!(6u8,36,3,1),
        elem!(7u8,24,3,1),elem!(0u8,36,3,1),elem!(3u8,12,1,1),elem!(3u8,24,2,1),
        elem!(5u8,36,3,1),elem!(6u8,12,1,1),elem!(7u8,36,4,1),elem!(2u8,36,3,1),
        elem!(3u8,24,2,1),elem!(5u8,12,1,1),elem!(6u8,24,2,1),elem!(6u8,36,4,1),
        elem!(1u8,12,1,1),elem!(2u8,36,3,1),elem!(4u8,24,2,1),elem!(5u8,12,1,1),
        elem!(5u8,24,2,1),elem!(7u8,12,2,1),elem!(0u8,60,5,1),elem!(3u8,12,1,1),
        elem!(4u8,60,5,1),elem!(7u8,12,2,1),elem!(0u8,24,2,1),elem!(2u8,12,1,-47),

    // remainder 23
    elem!(1u8,36,4,1),elem!(0u8,12,1,1),elem!(3u8,36,4,1),elem!(1u8,24,2,1),
        elem!(7u8,12,2,1),elem!(1u8,24,2,1),elem!(7u8,36,4,1),elem!(6u8,36,4,1),
        elem!(5u8,12,2,1),elem!(0u8,36,4,1),elem!(0u8,24,2,1),elem!(4u8,12,1,1),
        elem!(7u8,36,4,1),elem!(6u8,24,3,1),elem!(4u8,36,4,1),elem!(3u8,48,5,1),
        elem!(5u8,24,3,1),elem!(2u8,12,1,1),elem!(5u8,24,3,1),elem!(2u8,12,1,1),
        elem!(5u8,24,3,1),elem!(2u8,48,5,1),elem!(4u8,36,4,1),elem!(3u8,24,3,1),
        elem!(1u8,36,4,1),elem!(0u8,12,1,1),elem!(3u8,24,2,1),elem!(7u8,36,4,1),
        elem!(7u8,12,2,1),elem!(2u8,36,4,1),elem!(1u8,36,4,1),elem!(0u8,24,2,1),
        elem!(6u8,12,2,1),elem!(0u8,24,2,1),elem!(6u8,36,4,1),elem!(4u8,12,1,1),
        elem!(7u8,36,4,1),elem!(6u8,24,3,1),elem!(4u8,12,1,1),elem!(6u8,24,3,1),
        elem!(3u8,12,1,1),elem!(5u8,60,7,1),elem!(2u8,12,1,1),elem!(5u8,60,7,1),
        elem!(2u8,12,1,1),elem!(4u8,24,3,1),elem!(1u8,12,1,1),elem!(3u8,24,3,-47),

    // remainder 29
    elem!(0u8,12,1,1),elem!(5u8,36,5,1),elem!(5u8,24,3,1),elem!(7u8,12,2,1),
        elem!(5u8,24,3,1),elem!(7u8,36,5,1),elem!(7u8,36,5,1),elem!(7u8,12,2,1),
        elem!(4u8,36,5,1),elem!(4u8,24,3,1),elem!(6u8,12,2,1),elem!(3u8,36,5,1),
        elem!(3u8,24,3,1),elem!(5u8,36,5,1),elem!(6u8,48,7,1),elem!(3u8,24,3,1),
        elem!(6u8,12,2,1),elem!(3u8,24,3,1),elem!(4u8,12,2,1),elem!(1u8,24,3,1),
        elem!(4u8,48,7,1),elem!(1u8,36,5,1),elem!(2u8,24,3,1),elem!(4u8,36,5,1),
        elem!(4u8,12,2,1),elem!(1u8,24,3,1),elem!(3u8,36,5,1),elem!(3u8,12,2,1),
        elem!(0u8,36,5,1),elem!(0u8,36,5,1),elem!(0u8,24,3,1),elem!(2u8,12,2,1),
        elem!(0u8,24,3,1),elem!(2u8,36,5,1),elem!(2u8,12,1,1),elem!(7u8,36,5,1),
        elem!(7u8,24,4,1),elem!(2u8,12,1,1),elem!(6u8,24,4,1),elem!(2u8,12,1,1),
        elem!(6u8,60,9,1),elem!(1u8,12,1,1),elem!(6u8,60,9,1),elem!(1u8,12,1,1),
        elem!(5u8,24,4,1),elem!(1u8,12,1,1),elem!(5u8,24,4,1),elem!(0u8,36,5,-47),

    // remainder 31
    elem!(3u8,36,5,1),elem!(5u8,24,4,1),elem!(1u8,12,2,1),elem!(1u8,24,3,1),
        elem!(5u8,36,5,1),elem!(7u8,36,6,1),elem!(1u8,12,2,1),elem!(0u8,36,5,1),
        elem!(2u8,24,3,1),elem!(6u8,12,2,1),elem!(4u8,36,5,1),elem!(7u8,24,4,1),
        elem!(4u8,36,5,1),elem!(6u8,48,7,1),elem!(7u8,24,4,1),elem!(3u8,12,2,1),
        elem!(2u8,24,3,1),elem!(5u8,12,2,1),elem!(4u8,24,4,1),elem!(0u8,48,7,1),
        elem!(1u8,36,5,1),elem!(3u8,24,4,1),elem!(0u8,36,5,1),elem!(3u8,12,2,1),
        elem!(1u8,24,3,1),elem!(5u8,36,5,1),elem!(7u8,12,2,1),elem!(6u8,36,6,1),
        elem!(0u8,36,5,1),elem!(2u8,24,3,1),elem!(6u8,12,2,1),elem!(6u8,24,4,1),
        elem!(2u8,36,5,1),elem!(4u8,12,2,1),elem!(2u8,36,5,1),elem!(4u8,24,4,1),
        elem!(1u8,12,1,1),elem!(7u8,24,4,1),elem!(3u8,12,2,1),elem!(2u8,60,9,1),
        elem!(0u8,12,1,1),elem!(7u8,60,9,1),elem!(5u8,12,2,1),elem!(4u8,24,4,1),
        elem!(0u8,12,1,1),elem!(6u8,24,4,1),elem!(3u8,36,5,1),elem!(5u8,12,2,-47),

    // remainder 37
    elem!(1u8,24,4,1),elem!(3u8,12,2,1),elem!(3u8,24,4,1),elem!(5u8,36,7,1),
        elem!(0u8,36,6,1),elem!(3u8,12,2,1),elem!(3u8,36,6,1),elem!(6u8,24,5,1),
        elem!(0u8,12,2,1),elem!(1u8,36,6,1),elem!(4u8,24,4,1),elem!(5u8,36,7,1),
        elem!(0u8,48,8,1),elem!(4u8,24,4,1),elem!(5u8,12,2,1),elem!(7u8,24,5,1),
        elem!(0u8,12,2,1),elem!(2u8,24,4,1),elem!(3u8,48,8,1),elem!(7u8,36,7,1),
        elem!(2u8,24,4,1),elem!(3u8,36,6,1),elem!(6u8,12,2,1),elem!(7u8,24,5,1),
        elem!(1u8,36,6,1),elem!(4u8,12,2,1),elem!(4u8,36,6,1),elem!(7u8,36,7,1),
        elem!(2u8,24,4,1),elem!(4u8,12,2,1),elem!(4u8,24,4,1),elem!(6u8,36,7,1),
        elem!(2u8,12,2,1),elem!(2u8,36,6,1),elem!(6u8,24,4,1),elem!(6u8,12,3,1),
        elem!(0u8,24,4,1),elem!(1u8,12,2,1),elem!(2u8,60,10,1),elem!(7u8,12,3,1),
        elem!(0u8,60,10,1),elem!(5u8,12,2,1),elem!(6u8,24,4,1),elem!(7u8,12,3,1),
        elem!(1u8,24,4,1),elem!(1u8,36,6,1),elem!(5u8,12,2,1),elem!(5u8,36,7,-47),

    // remainder 41
    elem!(0u8,12,2,1),elem!(3u8,24,5,1),elem!(0u8,36,7,1),elem!(1u8,36,7,1),
        elem!(1u8,12,2,1),elem!(3u8,36,7,1),elem!(3u8,24,5,1),elem!(1u8,12,2,1),
        elem!(4u8,36,7,1),elem!(4u8,24,5,1),elem!(2u8,36,7,1),elem!(2u8,48,9,1),
        elem!(5u8,24,5,1),elem!(2u8,12,2,1),elem!(5u8,24,5,1),elem!(2u8,12,2,1),
        elem!(5u8,24,5,1),elem!(2u8,48,9,1),elem!(5u8,36,7,1),elem!(5u8,24,5,1),
        elem!(3u8,36,7,1),elem!(3u8,12,2,1),elem!(6u8,24,5,1),elem!(4u8,36,7,1),
        elem!(4u8,12,2,1),elem!(6u8,36,7,1),elem!(6u8,36,7,1),elem!(7u8,24,5,1),
        elem!(4u8,12,2,1),elem!(7u8,24,5,1),elem!(4u8,36,7,1),elem!(6u8,12,3,1),
        elem!(0u8,36,7,1),elem!(0u8,24,4,1),elem!(6u8,12,3,1),elem!(0u8,24,4,1),
        elem!(5u8,12,3,1),elem!(0u8,60,11,1),elem!(6u8,12,3,1),elem!(1u8,60,11,1),
        elem!(7u8,12,3,1),elem!(2u8,24,4,1),elem!(7u8,12,3,1),elem!(1u8,24,4,1),
        elem!(7u8,36,7,1),elem!(7u8,12,3,1),elem!(1u8,36,7,1),elem!(3u8,24,5,-47),

    // remainder 43
    elem!(6u8,24,5,1),elem!(5u8,36,8,1),elem!(0u8,36,7,1),elem!(3u8,12,2,1),
        elem!(7u8,36,8,1),elem!(2u8,24,5,1),elem!(2u8,12,2,1),elem!(6u8,36,8,1),
        elem!(0u8,24,4,1),elem!(7u8,36,8,1),elem!(3u8,48,10,1),elem!(1u8,24,5,1),
        elem!(0u8,12,2,1),elem!(4u8,24,5,1),elem!(3u8,12,2,1),elem!(7u8,24,5,1),
        elem!(6u8,48,10,1),elem!(4u8,36,8,1),elem!(0u8,24,4,1),elem!(7u8,36,8,1),
        elem!(1u8,12,2,1),elem!(5u8,24,5,1),elem!(5u8,36,8,1),elem!(0u8,12,2,1),
        elem!(4u8,36,7,1),elem!(7u8,36,8,1),elem!(2u8,24,5,1),elem!(1u8,12,2,1),
        elem!(4u8,24,5,1),elem!(4u8,36,7,1),elem!(6u8,12,3,1),elem!(2u8,36,7,1),
        elem!(6u8,24,5,1),elem!(5u8,12,3,1),elem!(1u8,24,5,1),elem!(0u8,12,2,1),
        elem!(3u8,60,12,1),elem!(5u8,12,3,1),elem!(2u8,60,12,1),elem!(4u8,12,2,1),
        elem!(7u8,24,5,1),elem!(6u8,12,3,1),elem!(2u8,24,5,1),elem!(1u8,36,7,1),
        elem!(5u8,12,3,1),elem!(1u8,36,7,1),elem!(3u8,24,5,1),elem!(3u8,12,2,-47),

    // remainder 47
    elem!(1u8,36,8,1),elem!(1u8,36,8,1),elem!(2u8,12,2,1),elem!(6u8,36,8,1),
        elem!(7u8,24,6,1),elem!(2u8,12,3,1),elem!(0u8,36,8,1),elem!(0u8,24,5,1),
        elem!(3u8,36,8,1),elem!(4u8,48,11,1),elem!(1u8,24,5,1),elem!(4u8,12,3,1),
        elem!(1u8,24,5,1),elem!(6u8,12,3,1),elem!(3u8,24,5,1),elem!(6u8,48,11,1),
        elem!(3u8,36,8,1),elem!(4u8,24,5,1),elem!(7u8,36,8,1),elem!(7u8,12,3,1),
        elem!(5u8,24,6,1),elem!(0u8,36,8,1),elem!(1u8,12,2,1),elem!(5u8,36,8,1),
        elem!(6u8,36,8,1),elem!(6u8,24,6,1),elem!(2u8,12,2,1),elem!(7u8,24,6,1),
        elem!(2u8,36,8,1),elem!(2u8,12,3,1),elem!(0u8,36,8,1),elem!(0u8,24,5,1),
        elem!(3u8,12,3,1),elem!(1u8,24,5,1),elem!(4u8,12,3,1),elem!(2u8,60,13,1),
        elem!(4u8,12,3,1),elem!(3u8,60,13,1),elem!(5u8,12,3,1),elem!(3u8,24,5,1),
        elem!(6u8,12,3,1),elem!(4u8,24,5,1),elem!(7u8,36,8,1),elem!(7u8,12,3,1),
        elem!(5u8,36,8,1),elem!(5u8,24,6,1),elem!(0u8,12,2,1),elem!(5u8,24,6,-47),

    // remainder 53
    elem!(2u8,36,9,1),elem!(2u8,12,3,1),elem!(3u8,36,9,1),elem!(3u8,24,6,1),
        elem!(4u8,12,3,1),elem!(4u8,36,9,1),elem!(5u8,24,6,1),elem!(6u8,36,9,1),
        elem!(5u8,48,12,1),elem!(6u8,24,6,1),elem!(7u8,12,3,1),elem!(7u8,24,7,1),
        elem!(0u8,12,3,1),elem!(0u8,24,6,1),elem!(1u8,48,12,1),elem!(2u8,36,9,1),
        elem!(1u8,24,6,1),elem!(2u8,36,9,1),elem!(3u8,12,3,1),elem!(3u8,24,6,1),
        elem!(4u8,36,9,1),elem!(4u8,12,3,1),elem!(5u8,36,9,1),elem!(5u8,36,9,1),
        elem!(6u8,24,6,1),elem!(7u8,12,3,1),elem!(6u8,24,6,1),elem!(7u8,36,10,1),
        elem!(0u8,12,3,1),elem!(0u8,36,9,1),elem!(1u8,24,6,1),elem!(2u8,12,3,1),
        elem!(1u8,24,6,1),elem!(2u8,12,3,1),elem!(3u8,60,15,1),elem!(3u8,12,3,1),
        elem!(4u8,60,15,1),elem!(4u8,12,3,1),elem!(5u8,24,6,1),elem!(6u8,12,3,1),
        elem!(5u8,24,6,1),elem!(6u8,36,9,1),elem!(7u8,12,3,1),elem!(7u8,36,10,1),
        elem!(0u8,24,6,1),elem!(1u8,12,3,1),elem!(0u8,24,6,1),elem!(1u8,36,9,-47),

    // remainder 59
    elem!(3u8,12,3,1),elem!(6u8,36,10,1),elem!(7u8,24,7,1),elem!(6u8,12,4,1),
        elem!(0u8,36,10,1),elem!(1u8,24,6,1),elem!(7u8,36,11,1),elem!(0u8,48,13,1),
        elem!(4u8,24,7,1),elem!(2u8,12,3,1),elem!(5u8,24,7,1),elem!(2u8,12,3,1),
        elem!(5u8,24,7,1),elem!(3u8,48,13,1),elem!(7u8,36,11,1),elem!(0u8,24,6,1),
        elem!(6u8,36,10,1),elem!(7u8,12,4,1),elem!(1u8,24,7,1),elem!(0u8,36,10,1),
        elem!(1u8,12,3,1),elem!(4u8,36,10,1),elem!(5u8,36,10,1),elem!(5u8,24,7,1),
        elem!(4u8,12,3,1),elem!(6u8,24,7,1),elem!(4u8,36,10,1),elem!(6u8,12,4,1),
        elem!(0u8,36,10,1),elem!(2u8,24,6,1),elem!(7u8,12,4,1),elem!(3u8,24,7,1),
        elem!(1u8,12,3,1),elem!(3u8,60,17,1),elem!(2u8,12,3,1),elem!(5u8,60,17,1),
        elem!(4u8,12,3,1),elem!(6u8,24,7,1),elem!(4u8,12,4,1),elem!(0u8,24,6,1),
        elem!(5u8,36,10,1),elem!(7u8,12,4,1),elem!(1u8,36,10,1),elem!(3u8,24,7,1),
        elem!(1u8,12,3,1),elem!(3u8,24,7,1),elem!(2u8,36,10,1),elem!(2u8,36,10,-47),

    // remainder 61
    elem!(2u8,36,10,1),elem!(5u8,24,7,1),elem!(5u8,12,4,1),elem!(2u8,36,10,1),
        elem!(6u8,24,7,1),elem!(5u8,36,11,1),elem!(0u8,48,14,1),elem!(0u8,24,7,1),
        elem!(0u8,12,3,1),elem!(4u8,24,7,1),elem!(3u8,12,3,1),elem!(7u8,24,7,1),
        elem!(7u8,48,14,1),elem!(7u8,36,11,1),elem!(2u8,24,7,1),elem!(1u8,36,10,1),
        elem!(5u8,12,4,1),elem!(2u8,24,7,1),elem!(2u8,36,10,1),elem!(5u8,12,4,1),
        elem!(1u8,36,10,1),elem!(4u8,36,11,1),elem!(1u8,24,7,1),elem!(0u8,12,3,1),
        elem!(4u8,24,7,1),elem!(4u8,36,10,1),elem!(7u8,12,4,1),elem!(3u8,36,10,1),
        elem!(7u8,24,7,1),elem!(6u8,12,4,1),elem!(2u8,24,7,1),elem!(3u8,12,3,1),
        elem!(6u8,60,18,1),elem!(1u8,12,3,1),elem!(6u8,60,18,1),elem!(1u8,12,3,1),
        elem!(4u8,24,7,1),elem!(5u8,12,4,1),elem!(1u8,24,7,1),elem!(0u8,36,10,1),
        elem!(4u8,12,4,1),elem!(0u8,36,10,1),elem!(3u8,24,7,1),elem!(3u8,12,3,1),
        elem!(7u8,24,7,1),elem!(6u8,36,11,1),elem!(3u8,36,10,1),elem!(6u8,12,4,-47),

    // remainder 67
    elem!(2u8,24,7,1),elem!(6u8,12,4,1),elem!(6u8,36,12,1),elem!(2u8,24,7,1),
        elem!(6u8,36,12,1),elem!(3u8,48,15,1),elem!(6u8,24,8,1),elem!(3u8,12,4,1),
        elem!(1u8,24,7,1),elem!(6u8,12,4,1),elem!(4u8,24,8,1),elem!(1u8,48,15,1),
        elem!(4u8,36,12,1),elem!(1u8,24,7,1),elem!(5u8,36,12,1),elem!(1u8,12,4,1),
        elem!(1u8,24,7,1),elem!(5u8,36,12,1),elem!(2u8,12,4,1),elem!(0u8,36,11,1),
        elem!(4u8,36,12,1),elem!(0u8,24,7,1),elem!(5u8,12,4,1),elem!(4u8,24,8,1),
        elem!(1u8,36,11,1),elem!(5u8,12,4,1),elem!(3u8,36,11,1),elem!(7u8,24,8,1),
        elem!(5u8,12,4,1),elem!(3u8,24,8,1),elem!(0u8,12,3,1),elem!(7u8,60,20,1),
        elem!(0u8,12,3,1),elem!(7u8,60,20,1),elem!(0u8,12,3,1),elem!(7u8,24,8,1),
        elem!(4u8,12,4,1),elem!(2u8,24,8,1),elem!(0u8,36,11,1),elem!(4u8,12,4,1),
        elem!(2u8,36,11,1),elem!(6u8,24,8,1),elem!(3u8,12,4,1),elem!(2u8,24,7,1),
        elem!(7u8,36,12,1),elem!(3u8,36,11,1),elem!(7u8,12,4,1),elem!(5u8,36,12,-47),

    // remainder 71
    elem!(0u8,12,4,1),elem!(0u8,36,12,1),elem!(1u8,24,8,1),elem!(2u8,36,12,1),
        elem!(4u8,48,16,1),elem!(5u8,24,8,1),elem!(7u8,12,4,1),elem!(7u8,24,9,1),
        elem!(0u8,12,4,1),elem!(0u8,24,8,1),elem!(2u8,48,16,1),elem!(3u8,36,12,1),
        elem!(5u8,24,8,1),elem!(6u8,36,12,1),elem!(7u8,12,4,1),elem!(7u8,24,9,1),
        elem!(1u8,36,12,1),elem!(2u8,12,4,1),elem!(1u8,36,12,1),elem!(3u8,36,12,1),
        elem!(5u8,24,8,1),elem!(5u8,12,4,1),elem!(6u8,24,8,1),elem!(7u8,36,13,1),
        elem!(1u8,12,4,1),elem!(1u8,36,12,1),elem!(3u8,24,8,1),elem!(3u8,12,4,1),
        elem!(4u8,24,8,1),elem!(4u8,12,4,1),elem!(5u8,60,20,1),elem!(7u8,12,5,1),
        elem!(0u8,60,20,1),elem!(2u8,12,4,1),elem!(3u8,24,8,1),elem!(3u8,12,4,1),
        elem!(4u8,24,8,1),elem!(4u8,36,12,1),elem!(6u8,12,4,1),elem!(6u8,36,13,1),
        elem!(0u8,24,8,1),elem!(1u8,12,4,1),elem!(2u8,24,8,1),elem!(2u8,36,12,1),
        elem!(4u8,36,12,1),elem!(6u8,12,4,1),elem!(5u8,36,12,1),elem!(6u8,24,9,-47),

    // remainder 73
    elem!(2u8,36,12,1),elem!(5u8,24,9,1),elem!(0u8,36,12,1),elem!(5u8,48,17,1),
        elem!(2u8,24,8,1),elem!(5u8,12,4,1),elem!(6u8,24,9,1),elem!(1u8,12,4,1),
        elem!(2u8,24,8,1),elem!(5u8,48,17,1),elem!(2u8,36,12,1),elem!(7u8,24,9,1),
        elem!(2u8,36,12,1),elem!(5u8,12,4,1),elem!(7u8,24,9,1),elem!(1u8,36,12,1),
        elem!(5u8,12,4,1),elem!(7u8,36,13,1),elem!(3u8,36,12,1),elem!(7u8,24,9,1),
        elem!(1u8,12,4,1),elem!(3u8,24,8,1),elem!(6u8,36,13,1),elem!(3u8,12,4,1),
        elem!(4u8,36,13,1),elem!(0u8,24,8,1),elem!(3u8,12,4,1),elem!(4u8,24,8,1),
        elem!(6u8,12,5,1),elem!(0u8,60,20,1),elem!(6u8,12,5,1),elem!(1u8,60,20,1),
        elem!(7u8,12,5,1),elem!(1u8,24,8,1),elem!(3u8,12,4,1),elem!(4u8,24,8,1),
        elem!(7u8,36,13,1),elem!(3u8,12,4,1),elem!(4u8,36,13,1),elem!(1u8,24,8,1),
        elem!(4u8,12,4,1),elem!(6u8,24,9,1),elem!(0u8,36,12,1),elem!(4u8,36,13,1),
        elem!(0u8,12,4,1),elem!(2u8,36,12,1),elem!(6u8,24,9,1),elem!(0u8,12,4,-47),

    // remainder 79
    elem!(2u8,24,9,1),elem!(3u8,36,13,1),elem!(6u8,48,18,1),elem!(7u8,24,9,1),
        elem!(7u8,12,5,1),elem!(3u8,24,9,1),elem!(4u8,12,5,1),elem!(0u8,24,9,1),
        elem!(0u8,48,18,1),elem!(1u8,36,13,1),elem!(4u8,24,9,1),elem!(5u8,36,14,1),
        elem!(2u8,12,4,1),elem!(6u8,24,9,1),elem!(5u8,36,14,1),elem!(1u8,12,4,1),
        elem!(6u8,36,14,1),elem!(2u8,36,13,1),elem!(7u8,24,9,1),elem!(7u8,12,5,1),
        elem!(3u8,24,9,1),elem!(3u8,36,14,1),elem!(0u8,12,4,1),elem!(4u8,36,14,1),
        elem!(1u8,24,9,1),elem!(0u8,12,4,1),elem!(4u8,24,9,1),elem!(5u8,12,5,1),
        elem!(1u8,60,22,1),elem!(5u8,12,5,1),elem!(2u8,60,22,1),elem!(6u8,12,5,1),
        elem!(2u8,24,9,1),elem!(3u8,12,4,1),elem!(7u8,24,9,1),elem!(6u8,36,14,1),
        elem!(3u8,12,4,1),elem!(7u8,36,14,1),elem!(4u8,24,9,1),elem!(4u8,12,5,1),
        elem!(0u8,24,9,1),elem!(0u8,36,13,1),elem!(5u8,36,14,1),elem!(1u8,12,4,1),
        elem!(6u8,36,14,1),elem!(2u8,24,9,1),elem!(1u8,12,4,1),elem!(5u8,36,14,-47),

    // remainder 83
    elem!(6u8,36,15,1),elem!(0u8,48,19,1),elem!(0u8,24,9,1),elem!(4u8,12,5,1),
        elem!(1u8,24,9,1),elem!(6u8,12,5,1),elem!(3u8,24,9,1),elem!(7u8,48,19,1),
        elem!(7u8,36,15,1),elem!(1u8,24,9,1),elem!(4u8,36,14,1),elem!(7u8,12,5,1),
        elem!(5u8,24,10,1),elem!(1u8,36,14,1),elem!(2u8,12,5,1),elem!(0u8,36,14,1),
        elem!(1u8,36,14,1),elem!(4u8,24,10,1),elem!(0u8,12,4,1),elem!(5u8,24,10,1),
        elem!(2u8,36,14,1),elem!(3u8,12,5,1),elem!(2u8,36,14,1),elem!(3u8,24,9,1),
        elem!(7u8,12,5,1),elem!(5u8,24,10,1),elem!(1u8,12,4,1),elem!(6u8,60,24,1),
        elem!(4u8,12,5,1),elem!(3u8,60,24,1),elem!(1u8,12,4,1),elem!(6u8,24,10,1),
        elem!(2u8,12,5,1),elem!(0u8,24,9,1),elem!(4u8,36,14,1),elem!(5u8,12,5,1),
        elem!(4u8,36,14,1),elem!(5u8,24,10,1),elem!(2u8,12,4,1),elem!(7u8,24,10,1),
        elem!(3u8,36,14,1),elem!(6u8,36,14,1),elem!(7u8,12,5,1),elem!(5u8,36,14,1),
        elem!(6u8,24,10,1),elem!(2u8,12,5,1),elem!(0u8,36,14,1),elem!(3u8,24,9,-47),

    // remainder 89
    elem!(2u8,48,20,1),elem!(5u8,24,10,1),elem!(6u8,12,5,1),elem!(6u8,24,11,1),
        elem!(1u8,12,5,1),elem!(1u8,24,10,1),elem!(2u8,48,20,1),elem!(5u8,36,15,1),
        elem!(7u8,24,11,1),elem!(1u8,36,15,1),elem!(2u8,12,5,1),elem!(3u8,24,10,1),
        elem!(4u8,36,15,1),elem!(7u8,12,5,1),elem!(7u8,36,16,1),elem!(2u8,36,15,1),
        elem!(3u8,24,10,1),elem!(4u8,12,5,1),elem!(5u8,24,10,1),elem!(7u8,36,16,1),
        elem!(1u8,12,5,1),elem!(1u8,36,15,1),elem!(4u8,24,10,1),elem!(6u8,12,5,1),
        elem!(5u8,24,10,1),elem!(7u8,12,6,1),elem!(0u8,60,25,1),elem!(3u8,12,5,1),
        elem!(4u8,60,25,1),elem!(7u8,12,6,1),elem!(0u8,24,10,1),elem!(2u8,12,5,1),
        elem!(1u8,24,10,1),elem!(3u8,36,15,1),elem!(6u8,12,5,1),elem!(6u8,36,16,1),
        elem!(0u8,24,10,1),elem!(2u8,12,5,1),elem!(3u8,24,10,1),elem!(4u8,36,15,1),
        elem!(5u8,36,16,1),elem!(0u8,12,5,1),elem!(0u8,36,15,1),elem!(3u8,24,10,1),
        elem!(4u8,12,5,1),elem!(5u8,36,15,1),elem!(6u8,24,11,1),elem!(0u8,36,15,-47),

    // remainder 97
    elem!(6u8,24,11,1),elem!(6u8,12,6,1),elem!(3u8,24,11,1),elem!(4u8,12,6,1),
        elem!(1u8,24,11,1),elem!(1u8,48,22,1),elem!(2u8,36,16,1),elem!(7u8,24,12,1),
        elem!(0u8,36,16,1),elem!(5u8,12,6,1),elem!(2u8,24,11,1),elem!(1u8,36,16,1),
        elem!(7u8,12,6,1),elem!(3u8,36,17,1),elem!(1u8,36,16,1),elem!(6u8,24,11,1),
        elem!(6u8,12,6,1),elem!(2u8,24,11,1),elem!(3u8,36,17,1),elem!(0u8,12,5,1),
        elem!(4u8,36,17,1),elem!(2u8,24,11,1),elem!(3u8,12,5,1),elem!(7u8,24,11,1),
        elem!(7u8,12,6,1),elem!(4u8,60,28,1),elem!(2u8,12,5,1),elem!(5u8,60,28,1),
        elem!(3u8,12,6,1),elem!(0u8,24,11,1),elem!(0u8,12,5,1),elem!(4u8,24,11,1),
        elem!(5u8,36,17,1),elem!(3u8,12,5,1),elem!(7u8,36,17,1),elem!(4u8,24,11,1),
        elem!(5u8,12,6,1),elem!(1u8,24,11,1),elem!(1u8,36,16,1),elem!(6u8,36,17,1),
        elem!(4u8,12,6,1),elem!(0u8,36,16,1),elem!(6u8,24,11,1),elem!(5u8,12,6,1),
        elem!(2u8,36,16,1),elem!(7u8,24,12,1),elem!(0u8,36,16,1),elem!(5u8,48,22,-47),

    // remainder 101
    elem!(3u8,12,6,1),elem!(2u8,24,11,1),elem!(5u8,12,6,1),elem!(4u8,24,12,1),
        elem!(1u8,48,23,1),elem!(1u8,36,17,1),elem!(3u8,24,12,1),elem!(0u8,36,17,1),
        elem!(2u8,12,6,1),elem!(0u8,24,11,1),elem!(4u8,36,17,1),elem!(7u8,12,6,1),
        elem!(5u8,36,18,1),elem!(0u8,36,17,1),elem!(3u8,24,11,1),elem!(7u8,12,6,1),
        elem!(5u8,24,12,1),elem!(2u8,36,17,1),elem!(4u8,12,6,1),elem!(1u8,36,17,1),
        elem!(5u8,24,12,1),elem!(1u8,12,5,1),elem!(7u8,24,12,1),elem!(3u8,12,6,1),
        elem!(1u8,60,29,1),elem!(1u8,12,5,1),elem!(6u8,60,29,1),elem!(6u8,12,6,1),
        elem!(4u8,24,12,1),elem!(0u8,12,5,1),elem!(6u8,24,12,1),elem!(2u8,36,17,1),
        elem!(6u8,12,6,1),elem!(3u8,36,17,1),elem!(5u8,24,12,1),elem!(2u8,12,6,1),
        elem!(0u8,24,11,1),elem!(4u8,36,17,1),elem!(7u8,36,18,1),elem!(2u8,12,6,1),
        elem!(0u8,36,17,1),elem!(3u8,24,11,1),elem!(7u8,12,6,1),elem!(5u8,36,17,1),
        elem!(7u8,24,12,1),elem!(4u8,36,17,1),elem!(6u8,48,23,1),elem!(6u8,24,12,-47),

    // remainder 103
    elem!(1u8,24,11,1),elem!(6u8,12,6,1),elem!(5u8,24,12,1),elem!(4u8,48,24,1),
        elem!(1u8,36,17,1),elem!(6u8,24,12,1),elem!(4u8,36,18,1),elem!(1u8,12,6,1),
        elem!(0u8,24,11,1),elem!(6u8,36,18,1),elem!(3u8,12,6,1),elem!(2u8,36,18,1),
        elem!(0u8,36,17,1),elem!(6u8,24,12,1),elem!(3u8,12,6,1),elem!(2u8,24,12,1),
        elem!(0u8,36,17,1),elem!(5u8,12,6,1),elem!(4u8,36,18,1),elem!(2u8,24,12,1),
        elem!(0u8,12,5,1),elem!(7u8,24,12,1),elem!(5u8,12,6,1),elem!(4u8,60,30,1),
        elem!(0u8,12,5,1),elem!(7u8,60,30,1),elem!(3u8,12,6,1),elem!(2u8,24,12,1),
        elem!(0u8,12,5,1),elem!(7u8,24,12,1),elem!(5u8,36,18,1),elem!(3u8,12,6,1),
        elem!(2u8,36,17,1),elem!(7u8,24,12,1),elem!(5u8,12,6,1),elem!(4u8,24,12,1),
        elem!(1u8,36,17,1),elem!(7u8,36,18,1),elem!(5u8,12,6,1),elem!(4u8,36,18,1),
        elem!(1u8,24,11,1),elem!(7u8,12,6,1),elem!(6u8,36,18,1),elem!(3u8,24,12,1),
        elem!(1u8,36,17,1),elem!(6u8,48,24,1),elem!(3u8,24,12,1),elem!(2u8,12,6,-47),

    // remainder 107
    elem!(1u8,12,6,1),elem!(2u8,24,12,1),elem!(3u8,48,24,1),elem!(6u8,36,19,1),
        elem!(1u8,24,12,1),elem!(3u8,36,18,1),elem!(6u8,12,6,1),elem!(7u8,24,13,1),
        elem!(1u8,36,18,1),elem!(4u8,12,6,1),elem!(5u8,36,18,1),elem!(7u8,36,19,1),
        elem!(1u8,24,12,1),elem!(4u8,12,6,1),elem!(5u8,24,12,1),elem!(7u8,36,19,1),
        elem!(2u8,12,6,1),elem!(3u8,36,18,1),elem!(5u8,24,12,1),elem!(7u8,12,7,1),
        elem!(0u8,24,12,1),elem!(2u8,12,6,1),elem!(3u8,60,30,1),elem!(7u8,12,7,1),
        elem!(0u8,60,30,1),elem!(4u8,12,6,1),elem!(5u8,24,12,1),elem!(7u8,12,7,1),
        elem!(0u8,24,12,1),elem!(2u8,36,18,1),elem!(4u8,12,6,1),elem!(5u8,36,19,1),
        elem!(0u8,24,12,1),elem!(2u8,12,6,1),elem!(3u8,24,12,1),elem!(6u8,36,19,1),
        elem!(0u8,36,18,1),elem!(2u8,12,6,1),elem!(3u8,36,18,1),elem!(6u8,24,13,1),
        elem!(0u8,12,6,1),elem!(1u8,36,18,1),elem!(4u8,24,12,1),elem!(6u8,36,19,1),
        elem!(1u8,48,24,1),elem!(4u8,24,12,1),elem!(5u8,12,6,1),elem!(6u8,24,13,-47),

    // remainder 109
    elem!(3u8,24,12,1),elem!(6u8,48,25,1),elem!(6u8,36,19,1),elem!(4u8,24,12,1),
        elem!(7u8,36,19,1),elem!(5u8,12,6,1),elem!(7u8,24,13,1),elem!(3u8,36,19,1),
        elem!(0u8,12,6,1),elem!(2u8,36,18,1),elem!(7u8,36,19,1),elem!(4u8,24,13,1),
        elem!(0u8,12,6,1),elem!(2u8,24,12,1),elem!(5u8,36,19,1),elem!(3u8,12,6,1),
        elem!(6u8,36,19,1),elem!(2u8,24,12,1),elem!(6u8,12,7,1),elem!(0u8,24,12,1),
        elem!(4u8,12,6,1),elem!(6u8,60,31,1),elem!(6u8,12,7,1),elem!(1u8,60,31,1),
        elem!(1u8,12,6,1),elem!(3u8,24,12,1),elem!(7u8,12,7,1),elem!(1u8,24,12,1),
        elem!(5u8,36,19,1),elem!(1u8,12,6,1),elem!(4u8,36,19,1),elem!(2u8,24,12,1),
        elem!(5u8,12,6,1),elem!(7u8,24,13,1),elem!(3u8,36,19,1),elem!(0u8,36,18,1),
        elem!(5u8,12,6,1),elem!(7u8,36,19,1),elem!(4u8,24,13,1),elem!(0u8,12,6,1),
        elem!(2u8,36,19,1),elem!(0u8,24,12,1),elem!(3u8,36,19,1),elem!(1u8,48,25,1),
        elem!(1u8,24,12,1),elem!(4u8,12,6,1),elem!(5u8,24,13,1),elem!(2u8,12,6,-47),

    // remainder 113
    elem!(6u8,48,26,1),elem!(5u8,36,20,1),elem!(0u8,24,12,1),elem!(7u8,36,20,1),
        elem!(2u8,12,6,1),elem!(5u8,24,13,1),elem!(6u8,36,20,1),elem!(0u8,12,6,1),
        elem!(4u8,36,19,1),elem!(6u8,36,20,1),elem!(1u8,24,13,1),elem!(1u8,12,6,1),
        elem!(5u8,24,13,1),elem!(4u8,36,19,1),elem!(7u8,12,7,1),elem!(3u8,36,19,1),
        elem!(5u8,24,13,1),elem!(4u8,12,7,1),elem!(0u8,24,13,1),elem!(0u8,12,6,1),
        elem!(3u8,60,32,1),elem!(5u8,12,7,1),elem!(2u8,60,32,1),elem!(4u8,12,6,1),
        elem!(7u8,24,13,1),elem!(7u8,12,7,1),elem!(3u8,24,13,1),elem!(2u8,36,19,1),
        elem!(4u8,12,7,1),elem!(0u8,36,19,1),elem!(3u8,24,13,1),elem!(2u8,12,6,1),
        elem!(6u8,24,13,1),elem!(6u8,36,20,1),elem!(1u8,36,19,1),elem!(3u8,12,6,1),
        elem!(7u8,36,20,1),elem!(1u8,24,13,1),elem!(2u8,12,6,1),elem!(5u8,36,20,1),
        elem!(0u8,24,12,1),elem!(7u8,36,20,1),elem!(2u8,48,26,1),elem!(1u8,24,13,1),
        elem!(1u8,12,6,1),elem!(4u8,24,13,1),elem!(3u8,12,6,1),elem!(6u8,24,13,-47),

    // remainder 121
    elem!(2u8,36,21,1),elem!(0u8,24,13,1),elem!(6u8,36,21,1),elem!(5u8,12,7,1),
        elem!(4u8,24,14,1),elem!(3u8,36,21,1),elem!(0u8,12,7,1),elem!(0u8,36,20,1),
        elem!(5u8,36,21,1),elem!(4u8,24,14,1),elem!(3u8,12,7,1),elem!(2u8,24,14,1),
        elem!(0u8,36,20,1),elem!(6u8,12,7,1),elem!(6u8,36,21,1),elem!(3u8,24,14,1),
        elem!(1u8,12,7,1),elem!(2u8,24,14,1),elem!(0u8,12,6,1),elem!(7u8,60,35,1),
        elem!(4u8,12,7,1),elem!(3u8,60,35,1),elem!(0u8,12,6,1),elem!(7u8,24,14,1),
        elem!(5u8,12,7,1),elem!(6u8,24,14,1),elem!(4u8,36,21,1),elem!(1u8,12,7,1),
        elem!(1u8,36,20,1),elem!(7u8,24,14,1),elem!(5u8,12,7,1),elem!(4u8,24,14,1),
        elem!(3u8,36,21,1),elem!(2u8,36,20,1),elem!(7u8,12,7,1),elem!(7u8,36,21,1),
        elem!(4u8,24,14,1),elem!(3u8,12,7,1),elem!(2u8,36,21,1),elem!(1u8,24,13,1),
        elem!(7u8,36,21,1),elem!(5u8,48,28,1),elem!(2u8,24,14,1),elem!(1u8,12,7,1),
        elem!(1u8,24,13,1),elem!(6u8,12,7,1),elem!(6u8,24,14,1),elem!(5u8,48,28,-47),

    // remainder 127
    elem!(6u8,24,15,1),elem!(3u8,36,22,1),elem!(0u8,12,7,1),elem!(2u8,24,14,1),
        elem!(6u8,36,22,1),elem!(5u8,12,7,1),elem!(7u8,36,22,1),elem!(6u8,36,22,1),
        elem!(3u8,24,14,1),elem!(7u8,12,8,1),elem!(2u8,24,14,1),elem!(5u8,36,22,1),
        elem!(4u8,12,7,1),elem!(5u8,36,22,1),elem!(4u8,24,15,1),elem!(0u8,12,7,1),
        elem!(2u8,24,14,1),elem!(6u8,12,8,1),elem!(1u8,60,36,1),elem!(3u8,12,7,1),
        elem!(4u8,60,36,1),elem!(6u8,12,8,1),elem!(1u8,24,14,1),elem!(5u8,12,7,1),
        elem!(7u8,24,15,1),elem!(3u8,36,22,1),elem!(2u8,12,7,1),elem!(3u8,36,22,1),
        elem!(2u8,24,14,1),elem!(5u8,12,8,1),elem!(0u8,24,14,1),elem!(4u8,36,22,1),
        elem!(1u8,36,22,1),elem!(0u8,12,7,1),elem!(2u8,36,22,1),elem!(1u8,24,14,1),
        elem!(5u8,12,7,1),elem!(7u8,36,22,1),elem!(4u8,24,15,1),elem!(1u8,36,21,1),
        elem!(7u8,48,29,1),elem!(7u8,24,15,1),elem!(3u8,12,7,1),elem!(6u8,24,15,1),
        elem!(1u8,12,7,1),elem!(4u8,24,15,1),elem!(0u8,48,29,1),elem!(0u8,36,21,-47),

    // remainder 131
    elem!(2u8,36,22,1),elem!(5u8,12,8,1),elem!(1u8,24,15,1),elem!(2u8,36,22,1),
        elem!(6u8,12,8,1),elem!(1u8,36,22,1),elem!(5u8,36,23,1),elem!(0u8,24,15,1),
        elem!(0u8,12,7,1),elem!(4u8,24,15,1),elem!(4u8,36,22,1),elem!(7u8,12,8,1),
        elem!(3u8,36,22,1),elem!(6u8,24,15,1),elem!(7u8,12,8,1),elem!(3u8,24,15,1),
        elem!(2u8,12,7,1),elem!(6u8,60,38,1),elem!(2u8,12,7,1),elem!(5u8,60,38,1),
        elem!(1u8,12,7,1),elem!(5u8,24,15,1),elem!(4u8,12,8,1),elem!(0u8,24,15,1),
        elem!(1u8,36,22,1),elem!(4u8,12,8,1),elem!(0u8,36,22,1),elem!(3u8,24,15,1),
        elem!(3u8,12,7,1),elem!(7u8,24,15,1),elem!(7u8,36,23,1),elem!(2u8,36,22,1),
        elem!(6u8,12,8,1),elem!(1u8,36,22,1),elem!(5u8,24,15,1),elem!(6u8,12,8,1),
        elem!(2u8,36,22,1),elem!(5u8,24,15,1),elem!(4u8,36,23,1),elem!(1u8,48,30,1),
        elem!(0u8,24,15,1),elem!(0u8,12,7,1),elem!(4u8,24,15,1),elem!(3u8,12,7,1),
        elem!(7u8,24,15,1),elem!(7u8,48,30,1),elem!(6u8,36,23,1),elem!(3u8,24,15,-47),

    // remainder 137
    elem!(2u8,12,8,1),elem!(0u8,24,15,1),elem!(6u8,36,24,1),elem!(2u8,12,8,1),
        elem!(0u8,36,23,1),elem!(4u8,36,24,1),elem!(0u8,24,15,1),elem!(6u8,12,8,1),
        elem!(4u8,24,16,1),elem!(1u8,36,23,1),elem!(4u8,12,8,1),elem!(3u8,36,23,1),
        elem!(7u8,24,16,1),elem!(4u8,12,8,1),elem!(3u8,24,16,1),elem!(1u8,12,7,1),
        elem!(7u8,60,40,1),elem!(1u8,12,7,1),elem!(6u8,60,40,1),elem!(0u8,12,7,1),
        elem!(6u8,24,16,1),elem!(4u8,12,8,1),elem!(3u8,24,16,1),elem!(0u8,36,23,1),
        elem!(4u8,12,8,1),elem!(3u8,36,23,1),elem!(6u8,24,16,1),elem!(3u8,12,8,1),
        elem!(1u8,24,15,1),elem!(7u8,36,24,1),elem!(3u8,36,23,1),elem!(7u8,12,8,1),
        elem!(5u8,36,24,1),elem!(1u8,24,15,1),elem!(7u8,12,8,1),elem!(5u8,36,24,1),
        elem!(2u8,24,15,1),elem!(7u8,36,24,1),elem!(2u8,48,31,1),elem!(5u8,24,16,1),
        elem!(2u8,12,8,1),elem!(1u8,24,15,1),elem!(6u8,12,8,1),elem!(5u8,24,16,1),
        elem!(2u8,48,31,1),elem!(5u8,36,24,1),elem!(0u8,24,15,1),elem!(5u8,36,24,-47),

    // remainder 139
    elem!(0u8,24,15,1),elem!(6u8,36,24,1),elem!(5u8,12,8,1),elem!(6u8,36,24,1),
        elem!(4u8,36,24,1),elem!(2u8,24,16,1),elem!(2u8,12,8,1),elem!(1u8,24,16,1),
        elem!(0u8,36,23,1),elem!(6u8,12,8,1),elem!(6u8,36,24,1),elem!(4u8,24,16,1),
        elem!(4u8,12,8,1),elem!(3u8,24,16,1),elem!(3u8,12,8,1),elem!(2u8,60,40,1),
        elem!(0u8,12,7,1),elem!(7u8,60,40,1),elem!(5u8,12,8,1),elem!(4u8,24,16,1),
        elem!(4u8,12,8,1),elem!(3u8,24,16,1),elem!(3u8,36,24,1),elem!(1u8,12,8,1),
        elem!(1u8,36,23,1),elem!(7u8,24,16,1),elem!(6u8,12,8,1),elem!(5u8,24,16,1),
        elem!(5u8,36,24,1),elem!(3u8,36,24,1),elem!(1u8,12,8,1),elem!(2u8,36,24,1),
        elem!(1u8,24,15,1),elem!(7u8,12,8,1),elem!(7u8,36,24,1),elem!(6u8,24,16,1),
        elem!(5u8,36,24,1),elem!(3u8,48,32,1),elem!(2u8,24,16,1),elem!(0u8,12,8,1),
        elem!(0u8,24,15,1),elem!(7u8,12,8,1),elem!(7u8,24,16,1),elem!(5u8,48,32,1),
        elem!(4u8,36,24,1),elem!(2u8,24,16,1),elem!(1u8,36,24,1),elem!(0u8,12,8,-47),

    // remainder 143
    elem!(2u8,36,24,1),elem!(5u8,12,8,1),elem!(7u8,36,25,1),elem!(3u8,36,24,1),
        elem!(7u8,24,17,1),elem!(2u8,12,8,1),elem!(3u8,24,16,1),elem!(6u8,36,25,1),
        elem!(2u8,12,8,1),elem!(4u8,36,25,1),elem!(0u8,24,16,1),elem!(2u8,12,8,1),
        elem!(4u8,24,16,1),elem!(7u8,12,9,1),elem!(0u8,60,40,1),elem!(7u8,12,9,1),
        elem!(0u8,60,40,1),elem!(7u8,12,9,1),elem!(0u8,24,16,1),elem!(3u8,12,8,1),
        elem!(5u8,24,16,1),elem!(7u8,36,25,1),elem!(3u8,12,8,1),elem!(5u8,36,25,1),
        elem!(1u8,24,16,1),elem!(4u8,12,8,1),elem!(5u8,24,17,1),elem!(0u8,36,24,1),
        elem!(4u8,36,25,1),elem!(0u8,12,8,1),elem!(2u8,36,24,1),elem!(5u8,24,17,1),
        elem!(1u8,12,8,1),elem!(1u8,36,24,1),elem!(5u8,24,17,1),elem!(1u8,36,24,1),
        elem!(4u8,48,33,1),elem!(1u8,24,16,1),elem!(4u8,12,8,1),elem!(6u8,24,17,1),
        elem!(1u8,12,8,1),elem!(3u8,24,16,1),elem!(6u8,48,33,1),elem!(3u8,36,24,1),
        elem!(6u8,24,17,1),elem!(2u8,36,24,1),elem!(6u8,12,8,1),elem!(6u8,24,17,-47),

    // remainder 149
    elem!(2u8,12,8,1),elem!(6u8,36,26,1),elem!(3u8,36,25,1),elem!(6u8,24,17,1),
        elem!(7u8,12,9,1),elem!(3u8,24,17,1),elem!(3u8,36,26,1),elem!(0u8,12,8,1),
        elem!(4u8,36,26,1),elem!(0u8,24,17,1),elem!(1u8,12,8,1),elem!(5u8,24,17,1),
        elem!(4u8,12,9,1),elem!(1u8,60,42,1),elem!(6u8,12,9,1),elem!(1u8,60,42,1),
        elem!(6u8,12,9,1),elem!(3u8,24,17,1),elem!(2u8,12,8,1),elem!(6u8,24,17,1),
        elem!(7u8,36,26,1),elem!(3u8,12,8,1),elem!(7u8,36,26,1),elem!(4u8,24,17,1),
        elem!(4u8,12,9,1),elem!(0u8,24,17,1),elem!(1u8,36,25,1),elem!(4u8,36,26,1),
        elem!(1u8,12,8,1),elem!(5u8,36,26,1),elem!(2u8,24,17,1),elem!(2u8,12,8,1),
        elem!(5u8,36,26,1),elem!(1u8,24,17,1),elem!(2u8,36,25,1),elem!(7u8,48,34,1),
        elem!(7u8,24,17,1),elem!(7u8,12,9,1),elem!(3u8,24,17,1),elem!(4u8,12,9,1),
        elem!(0u8,24,17,1),elem!(0u8,48,34,1),elem!(0u8,36,25,1),elem!(5u8,24,17,1),
        elem!(6u8,36,26,1),elem!(2u8,12,8,1),elem!(5u8,24,17,1),elem!(5u8,36,26,-47),

    // remainder 151
    elem!(3u8,36,26,1),elem!(2u8,36,26,1),elem!(2u8,24,17,1),elem!(3u8,12,9,1),
        elem!(1u8,24,17,1),elem!(3u8,36,26,1),elem!(1u8,12,8,1),elem!(7u8,36,26,1),
        elem!(5u8,24,18,1),elem!(0u8,12,8,1),elem!(4u8,24,17,1),elem!(6u8,12,9,1),
        elem!(4u8,60,43,1),elem!(5u8,12,9,1),elem!(2u8,60,43,1),elem!(3u8,12,9,1),
        elem!(1u8,24,17,1),elem!(3u8,12,8,1),elem!(7u8,24,18,1),elem!(2u8,36,26,1),
        elem!(0u8,12,8,1),elem!(6u8,36,26,1),elem!(4u8,24,17,1),elem!(6u8,12,9,1),
        elem!(4u8,24,17,1),elem!(5u8,36,26,1),elem!(5u8,36,26,1),elem!(4u8,12,9,1),
        elem!(1u8,36,26,1),elem!(0u8,24,17,1),elem!(1u8,12,8,1),elem!(7u8,36,26,1),
        elem!(6u8,24,18,1),elem!(0u8,36,25,1),elem!(7u8,48,35,1),elem!(3u8,24,17,1),
        elem!(5u8,12,9,1),elem!(2u8,24,17,1),elem!(5u8,12,9,1),elem!(2u8,24,17,1),
        elem!(4u8,48,35,1),elem!(0u8,36,25,1),elem!(7u8,24,18,1),elem!(1u8,36,26,1),
        elem!(0u8,12,8,1),elem!(6u8,24,17,1),elem!(7u8,36,26,1),elem!(6u8,12,9,-47),

    // remainder 157
    elem!(2u8,36,27,1),elem!(1u8,24,18,1),elem!(0u8,12,9,1),elem!(1u8,24,18,1),
        elem!(0u8,36,26,1),elem!(7u8,12,9,1),elem!(7u8,36,27,1),elem!(6u8,24,18,1),
        elem!(5u8,12,9,1),elem!(6u8,24,18,1),elem!(5u8,12,9,1),elem!(4u8,60,45,1),
        elem!(4u8,12,9,1),elem!(3u8,60,45,1),elem!(3u8,12,9,1),elem!(2u8,24,18,1),
        elem!(1u8,12,9,1),elem!(2u8,24,18,1),elem!(1u8,36,27,1),elem!(0u8,12,9,1),
        elem!(0u8,36,26,1),elem!(7u8,24,18,1),elem!(6u8,12,9,1),elem!(7u8,24,18,1),
        elem!(6u8,36,27,1),elem!(5u8,36,27,1),elem!(5u8,12,9,1),elem!(4u8,36,27,1),
        elem!(4u8,24,18,1),elem!(3u8,12,9,1),elem!(3u8,36,27,1),elem!(2u8,24,18,1),
        elem!(1u8,36,27,1),elem!(2u8,48,36,1),elem!(1u8,24,18,1),elem!(0u8,12,9,1),
        elem!(0u8,24,17,1),elem!(7u8,12,9,1),elem!(7u8,24,18,1),elem!(6u8,48,36,1),
        elem!(5u8,36,27,1),elem!(6u8,24,18,1),elem!(5u8,36,27,1),elem!(4u8,12,9,1),
        elem!(4u8,24,18,1),elem!(3u8,36,27,1),elem!(3u8,12,9,1),elem!(2u8,36,27,-47),

    // remainder 163
    elem!(1u8,24,18,1),elem!(5u8,12,10,1),elem!(0u8,24,18,1),elem!(5u8,36,28,1),
        elem!(5u8,12,9,1),elem!(7u8,36,28,1),elem!(7u8,24,19,1),elem!(4u8,12,9,1),
        elem!(6u8,24,19,1),elem!(3u8,12,9,1),elem!(5u8,60,47,1),elem!(3u8,12,9,1),
        elem!(4u8,60,47,1),elem!(2u8,12,9,1),elem!(4u8,24,19,1),elem!(1u8,12,9,1),
        elem!(3u8,24,19,1),elem!(0u8,36,28,1),elem!(0u8,12,9,1),elem!(2u8,36,28,1),
        elem!(2u8,24,18,1),elem!(7u8,12,10,1),elem!(2u8,24,18,1),elem!(6u8,36,28,1),
        elem!(6u8,36,28,1),elem!(5u8,12,10,1),elem!(1u8,36,28,1),elem!(0u8,24,18,1),
        elem!(5u8,12,9,1),elem!(7u8,36,28,1),elem!(7u8,24,19,1),elem!(4u8,36,28,1),
        elem!(3u8,48,37,1),elem!(6u8,24,19,1),elem!(3u8,12,9,1),elem!(6u8,24,19,1),
        elem!(1u8,12,9,1),elem!(4u8,24,19,1),elem!(1u8,48,37,1),elem!(4u8,36,28,1),
        elem!(3u8,24,19,1),elem!(0u8,36,28,1),elem!(0u8,12,9,1),elem!(2u8,24,18,1),
        elem!(7u8,36,28,1),elem!(6u8,12,10,1),elem!(2u8,36,28,1),elem!(1u8,36,28,-47),

    // remainder 167
    elem!(6u8,12,10,1),elem!(3u8,24,19,1),elem!(3u8,36,29,1),elem!(1u8,12,9,1),
        elem!(5u8,36,29,1),elem!(1u8,24,19,1),elem!(2u8,12,9,1),elem!(6u8,24,19,1),
        elem!(7u8,12,10,1),elem!(4u8,60,48,1),elem!(2u8,12,9,1),elem!(5u8,60,48,1),
        elem!(3u8,12,10,1),elem!(0u8,24,19,1),elem!(1u8,12,9,1),elem!(5u8,24,19,1),
        elem!(6u8,36,29,1),elem!(2u8,12,9,1),elem!(6u8,36,29,1),elem!(4u8,24,19,1),
        elem!(4u8,12,10,1),elem!(1u8,24,19,1),elem!(2u8,36,28,1),elem!(7u8,36,29,1),
        elem!(4u8,12,10,1),elem!(0u8,36,28,1),elem!(5u8,24,19,1),elem!(5u8,12,10,1),
        elem!(1u8,36,28,1),elem!(7u8,24,20,1),elem!(0u8,36,28,1),elem!(4u8,48,38,1),
        elem!(6u8,24,19,1),elem!(7u8,12,10,1),elem!(3u8,24,19,1),elem!(4u8,12,10,1),
        elem!(0u8,24,19,1),elem!(1u8,48,38,1),elem!(3u8,36,28,1),elem!(7u8,24,20,1),
        elem!(0u8,36,28,1),elem!(6u8,12,10,1),elem!(2u8,24,19,1),elem!(2u8,36,28,1),
        elem!(7u8,12,10,1),elem!(3u8,36,29,1),elem!(0u8,36,28,1),elem!(5u8,24,19,-47),

    // remainder 169
    elem!(0u8,24,19,1),elem!(3u8,36,29,1),elem!(1u8,12,9,1),elem!(7u8,36,29,1),
        elem!(7u8,24,20,1),elem!(1u8,12,9,1),elem!(7u8,24,20,1),elem!(2u8,12,9,1),
        elem!(7u8,60,49,1),elem!(1u8,12,9,1),elem!(6u8,60,49,1),elem!(0u8,12,9,1),
        elem!(5u8,24,20,1),elem!(0u8,12,9,1),elem!(6u8,24,20,1),elem!(0u8,36,29,1),
        elem!(0u8,12,9,1),elem!(6u8,36,29,1),elem!(4u8,24,19,1),elem!(7u8,12,10,1),
        elem!(4u8,24,19,1),elem!(7u8,36,29,1),elem!(6u8,36,29,1),elem!(6u8,12,10,1),
        elem!(4u8,36,29,1),elem!(4u8,24,19,1),elem!(6u8,12,10,1),elem!(3u8,36,29,1),
        elem!(3u8,24,19,1),elem!(5u8,36,29,1),elem!(5u8,48,39,1),elem!(2u8,24,19,1),
        elem!(5u8,12,10,1),elem!(2u8,24,19,1),elem!(5u8,12,10,1),elem!(2u8,24,19,1),
        elem!(5u8,48,39,1),elem!(2u8,36,29,1),elem!(2u8,24,19,1),elem!(4u8,36,29,1),
        elem!(4u8,12,10,1),elem!(1u8,24,19,1),elem!(3u8,36,29,1),elem!(3u8,12,10,1),
        elem!(1u8,36,29,1),elem!(1u8,36,29,1),elem!(0u8,24,19,1),elem!(3u8,12,10,-47),

    // remainder 173
    elem!(1u8,36,29,1),elem!(5u8,12,10,1),elem!(5u8,36,30,1),elem!(1u8,24,20,1),
        elem!(1u8,12,9,1),elem!(7u8,24,20,1),elem!(6u8,12,10,1),elem!(5u8,60,50,1),
        elem!(0u8,12,9,1),elem!(7u8,60,50,1),elem!(2u8,12,10,1),elem!(1u8,24,20,1),
        elem!(0u8,12,9,1),elem!(6u8,24,20,1),elem!(6u8,36,30,1),elem!(2u8,12,10,1),
        elem!(2u8,36,29,1),elem!(6u8,24,20,1),elem!(4u8,12,10,1),elem!(4u8,24,20,1),
        elem!(2u8,36,29,1),elem!(7u8,36,30,1),elem!(4u8,12,10,1),elem!(4u8,36,30,1),
        elem!(1u8,24,19,1),elem!(7u8,12,10,1),elem!(6u8,36,30,1),elem!(3u8,24,20,1),
        elem!(2u8,36,29,1),elem!(7u8,48,40,1),elem!(3u8,24,20,1),elem!(2u8,12,10,1),
        elem!(0u8,24,19,1),elem!(7u8,12,10,1),elem!(5u8,24,20,1),elem!(4u8,48,40,1),
        elem!(0u8,36,29,1),elem!(5u8,24,20,1),elem!(4u8,36,30,1),elem!(1u8,12,10,1),
        elem!(0u8,24,19,1),elem!(6u8,36,30,1),elem!(3u8,12,10,1),elem!(3u8,36,30,1),
        elem!(0u8,36,29,1),elem!(5u8,24,20,1),elem!(3u8,12,10,1),elem!(3u8,24,20,-47),

    // remainder 179
    elem!(3u8,12,10,1),elem!(5u8,36,31,1),elem!(3u8,24,20,1),elem!(6u8,12,11,1),
        elem!(0u8,24,20,1),elem!(4u8,12,10,1),elem!(5u8,60,51,1),elem!(7u8,12,11,1),
        elem!(0u8,60,51,1),elem!(2u8,12,10,1),elem!(3u8,24,20,1),elem!(7u8,12,11,1),
        elem!(1u8,24,20,1),elem!(4u8,36,31,1),elem!(2u8,12,10,1),elem!(4u8,36,31,1),
        elem!(2u8,24,20,1),elem!(6u8,12,10,1),elem!(6u8,24,21,1),elem!(2u8,36,31,1),
        elem!(0u8,36,30,1),elem!(6u8,12,10,1),elem!(7u8,36,31,1),elem!(5u8,24,21,1),
        elem!(1u8,12,10,1),elem!(3u8,36,31,1),elem!(0u8,24,20,1),elem!(3u8,36,31,1),
        elem!(1u8,48,41,1),elem!(0u8,24,20,1),elem!(4u8,12,10,1),elem!(5u8,24,21,1),
        elem!(2u8,12,10,1),elem!(3u8,24,20,1),elem!(7u8,48,41,1),elem!(6u8,36,31,1),
        elem!(4u8,24,20,1),elem!(7u8,36,31,1),elem!(4u8,12,10,1),elem!(6u8,24,21,1),
        elem!(2u8,36,31,1),elem!(0u8,12,10,1),elem!(1u8,36,30,1),elem!(7u8,36,31,1),
        elem!(5u8,24,21,1),elem!(1u8,12,10,1),elem!(1u8,24,20,1),elem!(5u8,36,31,-47),

    // remainder 181
    elem!(0u8,36,31,1),elem!(0u8,24,20,1),elem!(5u8,12,11,1),elem!(1u8,24,20,1),
        elem!(5u8,12,11,1),elem!(1u8,60,51,1),elem!(6u8,12,11,1),elem!(1u8,60,51,1),
        elem!(6u8,12,11,1),elem!(2u8,24,20,1),elem!(6u8,12,11,1),elem!(2u8,24,20,1),
        elem!(7u8,36,31,1),elem!(7u8,12,11,1),elem!(2u8,36,31,1),elem!(2u8,24,21,1),
        elem!(0u8,12,10,1),elem!(2u8,24,21,1),elem!(0u8,36,31,1),elem!(0u8,36,31,1),
        elem!(0u8,12,10,1),elem!(3u8,36,31,1),elem!(3u8,24,21,1),elem!(1u8,12,10,1),
        elem!(4u8,36,31,1),elem!(4u8,24,21,1),elem!(2u8,36,31,1),elem!(1u8,48,41,1),
        elem!(4u8,24,21,1),elem!(1u8,12,10,1),elem!(4u8,24,21,1),elem!(3u8,12,10,1),
        elem!(6u8,24,21,1),elem!(3u8,48,41,1),elem!(6u8,36,31,1),elem!(5u8,24,21,1),
        elem!(3u8,36,31,1),elem!(3u8,12,10,1),elem!(6u8,24,21,1),elem!(4u8,36,31,1),
        elem!(4u8,12,10,1),elem!(7u8,36,31,1),elem!(7u8,36,31,1),elem!(7u8,24,21,1),
        elem!(5u8,12,10,1),elem!(7u8,24,21,1),elem!(5u8,36,31,1),elem!(5u8,12,11,-47),

    // remainder 187
    elem!(1u8,24,21,1),elem!(3u8,12,11,1),elem!(1u8,24,21,1),elem!(4u8,12,11,1),
        elem!(2u8,60,53,1),elem!(5u8,12,11,1),elem!(2u8,60,53,1),elem!(5u8,12,11,1),
        elem!(3u8,24,21,1),elem!(6u8,12,11,1),elem!(4u8,24,21,1),elem!(6u8,36,32,1),
        elem!(7u8,12,11,1),elem!(4u8,36,32,1),elem!(6u8,24,22,1),elem!(0u8,12,10,1),
        elem!(6u8,24,22,1),elem!(0u8,36,32,1),elem!(1u8,36,32,1),elem!(2u8,12,10,1),
        elem!(7u8,36,32,1),elem!(7u8,24,22,1),elem!(3u8,12,11,1),elem!(0u8,36,32,1),
        elem!(1u8,24,21,1),elem!(3u8,36,32,1),elem!(4u8,48,43,1),elem!(2u8,24,21,1),
        elem!(5u8,12,11,1),elem!(2u8,24,21,1),elem!(5u8,12,11,1),elem!(2u8,24,21,1),
        elem!(5u8,48,43,1),elem!(3u8,36,32,1),elem!(4u8,24,21,1),elem!(6u8,36,32,1),
        elem!(7u8,12,11,1),elem!(4u8,24,22,1),elem!(0u8,36,32,1),elem!(0u8,12,10,1),
        elem!(5u8,36,32,1),elem!(6u8,36,32,1),elem!(7u8,24,22,1),elem!(1u8,12,10,1),
        elem!(7u8,24,22,1),elem!(1u8,36,32,1),elem!(3u8,12,11,1),elem!(0u8,36,32,-47),

    // remainder 191
    elem!(2u8,12,11,1),elem!(2u8,24,22,1),elem!(0u8,12,10,1),elem!(7u8,60,55,1),
        elem!(4u8,12,11,1),elem!(3u8,60,55,1),elem!(0u8,12,10,1),elem!(7u8,24,22,1),
        elem!(5u8,12,11,1),elem!(5u8,24,22,1),elem!(4u8,36,33,1),elem!(2u8,12,11,1),
        elem!(1u8,36,32,1),elem!(6u8,24,22,1),elem!(6u8,12,11,1),elem!(5u8,24,22,1),
        elem!(3u8,36,33,1),elem!(2u8,36,32,1),elem!(7u8,12,11,1),elem!(6u8,36,33,1),
        elem!(5u8,24,22,1),elem!(3u8,12,11,1),elem!(3u8,36,33,1),elem!(0u8,24,21,1),
        elem!(7u8,36,33,1),elem!(6u8,48,44,1),elem!(3u8,24,22,1),elem!(1u8,12,11,1),
        elem!(0u8,24,21,1),elem!(7u8,12,11,1),elem!(6u8,24,22,1),elem!(4u8,48,44,1),
        elem!(1u8,36,33,1),elem!(0u8,24,21,1),elem!(7u8,36,33,1),elem!(4u8,12,11,1),
        elem!(4u8,24,22,1),elem!(2u8,36,33,1),elem!(1u8,12,11,1),elem!(0u8,36,32,1),
        elem!(5u8,36,33,1),elem!(4u8,24,22,1),elem!(2u8,12,11,1),elem!(1u8,24,22,1),
        elem!(1u8,36,32,1),elem!(6u8,12,11,1),elem!(5u8,36,33,1),elem!(3u8,24,22,-47),

    // remainder 193
    elem!(2u8,24,22,1),elem!(1u8,12,11,1),elem!(2u8,60,55,1),elem!(3u8,12,11,1),
        elem!(4u8,60,55,1),elem!(5u8,12,11,1),elem!(6u8,24,22,1),elem!(5u8,12,11,1),
        elem!(5u8,24,22,1),elem!(6u8,36,33,1),elem!(6u8,12,11,1),elem!(7u8,36,34,1),
        elem!(0u8,24,22,1),elem!(0u8,12,11,1),elem!(1u8,24,22,1),elem!(1u8,36,33,1),
        elem!(1u8,36,33,1),elem!(3u8,12,11,1),elem!(2u8,36,33,1),elem!(3u8,24,22,1),
        elem!(4u8,12,11,1),elem!(4u8,36,33,1),elem!(4u8,24,22,1),elem!(5u8,36,33,1),
        elem!(5u8,48,44,1),elem!(7u8,24,22,1),elem!(7u8,12,11,1),elem!(7u8,24,23,1),
        elem!(0u8,12,11,1),elem!(0u8,24,22,1),elem!(0u8,48,44,1),elem!(2u8,36,33,1),
        elem!(2u8,24,22,1),elem!(3u8,36,33,1),elem!(3u8,12,11,1),elem!(3u8,24,22,1),
        elem!(4u8,36,33,1),elem!(5u8,12,11,1),elem!(4u8,36,33,1),elem!(6u8,36,33,1),
        elem!(6u8,24,22,1),elem!(6u8,12,11,1),elem!(7u8,24,22,1),elem!(7u8,36,34,1),
        elem!(0u8,12,11,1),elem!(1u8,36,33,1),elem!(1u8,24,22,1),elem!(2u8,12,11,-47),

    // remainder 197
    elem!(6u8,12,12,1),elem!(0u8,60,56,1),elem!(2u8,12,11,1),elem!(5u8,60,56,1),
        elem!(7u8,12,12,1),elem!(1u8,24,22,1),elem!(6u8,12,11,1),elem!(7u8,24,23,1),
        elem!(3u8,36,34,1),elem!(2u8,12,11,1),elem!(3u8,36,34,1),elem!(1u8,24,22,1),
        elem!(5u8,12,12,1),elem!(0u8,24,22,1),elem!(4u8,36,34,1),elem!(2u8,36,34,1),
        elem!(1u8,12,11,1),elem!(3u8,36,34,1),elem!(0u8,24,22,1),elem!(4u8,12,11,1),
        elem!(6u8,36,34,1),elem!(5u8,24,23,1),elem!(1u8,36,33,1),elem!(7u8,48,45,1),
        elem!(7u8,24,23,1),elem!(3u8,12,11,1),elem!(5u8,24,23,1),elem!(2u8,12,11,1),
        elem!(4u8,24,23,1),elem!(0u8,48,45,1),elem!(0u8,36,33,1),elem!(6u8,24,23,1),
        elem!(2u8,36,34,1),elem!(1u8,12,11,1),elem!(3u8,24,22,1),elem!(7u8,36,34,1),
        elem!(4u8,12,11,1),elem!(6u8,36,34,1),elem!(5u8,36,34,1),elem!(3u8,24,22,1),
        elem!(7u8,12,12,1),elem!(2u8,24,22,1),elem!(6u8,36,34,1),elem!(4u8,12,11,1),
        elem!(5u8,36,34,1),elem!(4u8,24,23,1),elem!(0u8,12,11,1),elem!(1u8,24,22,-47),

    // remainder 199
    elem!(3u8,60,57,1),elem!(1u8,12,11,1),elem!(6u8,60,57,1),elem!(4u8,12,11,1),
        elem!(7u8,24,23,1),elem!(5u8,12,12,1),elem!(0u8,24,22,1),elem!(5u8,36,34,1),
        elem!(6u8,12,12,1),elem!(2u8,36,34,1),elem!(2u8,24,23,1),elem!(0u8,12,11,1),
        elem!(3u8,24,23,1),elem!(2u8,36,34,1),elem!(3u8,36,34,1),elem!(3u8,12,11,1),
        elem!(6u8,36,34,1),elem!(7u8,24,23,1),elem!(5u8,12,12,1),elem!(0u8,36,34,1),
        elem!(1u8,24,22,1),elem!(6u8,36,35,1),elem!(0u8,48,45,1),elem!(4u8,24,23,1),
        elem!(1u8,12,11,1),elem!(4u8,24,23,1),elem!(3u8,12,11,1),elem!(6u8,24,23,1),
        elem!(3u8,48,45,1),elem!(7u8,36,35,1),elem!(1u8,24,22,1),elem!(6u8,36,34,1),
        elem!(7u8,12,12,1),elem!(2u8,24,23,1),elem!(0u8,36,34,1),elem!(1u8,12,11,1),
        elem!(4u8,36,34,1),elem!(4u8,36,34,1),elem!(5u8,24,23,1),elem!(4u8,12,11,1),
        elem!(7u8,24,23,1),elem!(5u8,36,34,1),elem!(5u8,12,12,1),elem!(1u8,36,34,1),
        elem!(2u8,24,22,1),elem!(7u8,12,12,1),elem!(2u8,24,23,1),elem!(0u8,12,11,-47),

    // remainder 209
    elem!(0u8,12,11,1),elem!(7u8,60,60,1),elem!(6u8,12,12,1),elem!(5u8,24,24,1),
        elem!(4u8,12,12,1),elem!(3u8,24,24,1),elem!(2u8,36,36,1),elem!(1u8,12,12,1),
        elem!(0u8,36,35,1),elem!(7u8,24,24,1),elem!(6u8,12,12,1),elem!(5u8,24,24,1),
        elem!(4u8,36,36,1),elem!(3u8,36,36,1),elem!(2u8,12,12,1),elem!(1u8,36,36,1),
        elem!(0u8,24,23,1),elem!(7u8,12,12,1),elem!(6u8,36,36,1),elem!(5u8,24,24,1),
        elem!(4u8,36,36,1),elem!(3u8,48,48,1),elem!(2u8,24,24,1),elem!(1u8,12,12,1),
        elem!(0u8,24,23,1),elem!(7u8,12,12,1),elem!(6u8,24,24,1),elem!(5u8,48,48,1),
        elem!(4u8,36,36,1),elem!(3u8,24,24,1),elem!(2u8,36,36,1),elem!(1u8,12,12,1),
        elem!(0u8,24,23,1),elem!(7u8,36,36,1),elem!(6u8,12,12,1),elem!(5u8,36,36,1),
        elem!(4u8,36,36,1),elem!(3u8,24,24,1),elem!(2u8,12,12,1),elem!(1u8,24,24,1),
        elem!(0u8,36,35,1),elem!(7u8,12,12,1),elem!(6u8,36,36,1),elem!(5u8,24,24,1),
        elem!(4u8,12,12,1),elem!(3u8,24,24,1),elem!(2u8,12,12,1),elem!(1u8,60,60,-47),
    ];

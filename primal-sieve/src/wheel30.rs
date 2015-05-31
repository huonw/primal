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
    let first_bit = subbit_base % 8;

    let wheel_index = (init.wheel_index as usize * 8 + first_bit) * SIZE;
    let sieve_index = low_offset / MODULO * SIZE / 8 + subbit_base / 8;

    let ret = WheelInfo {
        true_prime: p,
        prime: p / MODULO,
        sieve_index: sieve_index,
        wheel_index: wheel_index,
    };
    ret
}

pub const SIZE: usize = 8;
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
            unset_bit: 1u8 << $bit as u8,
            next_mult_factor: $nmf,
            correction: $c,
            next: $n,
        }
    }
}

pub const MODULO: usize = 30;

const TRUE_AT_BIT: &'static [usize; 8] = &[
    1, 7, 11, 13, 17, 19, 23, 29
    ];
const INIT: &'static [WheelInit; 30] = &[
    // 0
    init!{1,  0}, init!{0,  0}, init!{5,  1}, init!{4,  1},
    // 4
    init!{3,  1}, init!{2,  1}, init!{1,  1}, init!{0,  1},
    // 8
    init!{3,  2}, init!{2,  2}, init!{1,  2}, init!{0,  2},
    // 12
    init!{1,  3}, init!{0,  3}, init!{3,  4}, init!{2,  4},
    // 16
    init!{1,  4}, init!{0,  4}, init!{1,  5}, init!{0,  5},
    // 20
    init!{3,  6}, init!{2,  6}, init!{1,  6}, init!{0,  6},
    // 24
    init!{5,  7}, init!{4,  7}, init!{3,  7}, init!{2,  7},
    // 28
    init!{1,  7}, init!{0,  7},
];

const WHEEL: &'static [WheelElem; 8*8*8] = &[
    // remainder 1
    elem!(0,6,0,1),elem!(1,4,0,1),elem!(2,2,0,1),elem!(3,4,0,1),
        elem!(4,2,0,1),elem!(5,4,0,1),elem!(6,6,0,1),elem!(7,2,1,-7),

    elem!(1,6,0,1),elem!(2,4,0,1),elem!(3,2,0,1),elem!(4,4,0,1),
        elem!(5,2,0,1),elem!(6,4,0,1),elem!(7,6,1,1),elem!(0,2,0,-7),

    elem!(2,6,0,1),elem!(3,4,0,1),elem!(4,2,0,1),elem!(5,4,0,1),
        elem!(6,2,0,1),elem!(7,4,1,1),elem!(0,6,0,1),elem!(1,2,0,-7),

    elem!(3,6,0,1),elem!(4,4,0,1),elem!(5,2,0,1),elem!(6,4,0,1),
        elem!(7,2,1,1),elem!(0,4,0,1),elem!(1,6,0,1),elem!(2,2,0,-7),

    elem!(4,6,0,1),elem!(5,4,0,1),elem!(6,2,0,1),elem!(7,4,1,1),
        elem!(0,2,0,1),elem!(1,4,0,1),elem!(2,6,0,1),elem!(3,2,0,-7),

    elem!(5,6,0,1),elem!(6,4,0,1),elem!(7,2,1,1),elem!(0,4,0,1),
        elem!(1,2,0,1),elem!(2,4,0,1),elem!(3,6,0,1),elem!(4,2,0,-7),

    elem!(6,6,0,1),elem!(7,4,1,1),elem!(0,2,0,1),elem!(1,4,0,1),
        elem!(2,2,0,1),elem!(3,4,0,1),elem!(4,6,0,1),elem!(5,2,0,-7),

    elem!(7,6,1,1),elem!(0,4,0,1),elem!(1,2,0,1),elem!(2,4,0,1),
        elem!(3,2,0,1),elem!(4,4,0,1),elem!(5,6,0,1),elem!(6,2,0,-7),

    // remainder 7
    elem!(0,4,0,1),elem!(7,2,1,1),elem!(3,4,1,1),elem!(2,2,0,1),
        elem!(6,4,1,1),elem!(5,6,2,1),elem!(1,2,0,1),elem!(4,6,2,-7),

    elem!(1,4,1,1),elem!(0,2,0,1),elem!(4,4,1,1),elem!(3,2,0,1),
        elem!(7,4,1,1),elem!(6,6,2,1),elem!(2,2,0,1),elem!(5,6,2,-7),

    elem!(2,4,1,1),elem!(1,2,0,1),elem!(5,4,1,1),elem!(4,2,1,1),
        elem!(0,4,0,1),elem!(7,6,2,1),elem!(3,2,0,1),elem!(6,6,2,-7),

    elem!(3,4,1,1),elem!(2,2,0,1),elem!(6,4,1,1),elem!(5,2,1,1),
        elem!(1,4,1,1),elem!(0,6,1,1),elem!(4,2,0,1),elem!(7,6,2,-7),

    elem!(4,4,1,1),elem!(3,2,0,1),elem!(7,4,1,1),elem!(6,2,1,1),
        elem!(2,4,1,1),elem!(1,6,1,1),elem!(5,2,1,1),elem!(0,6,1,-7),

    elem!(5,4,1,1),elem!(4,2,1,1),elem!(0,4,0,1),elem!(7,2,1,1),
        elem!(3,4,1,1),elem!(2,6,1,1),elem!(6,2,1,1),elem!(1,6,1,-7),

    elem!(6,4,1,1),elem!(5,2,1,1),elem!(1,4,1,1),elem!(0,2,0,1),
        elem!(4,4,1,1),elem!(3,6,1,1),elem!(7,2,1,1),elem!(2,6,1,-7),

    elem!(7,4,1,1),elem!(6,2,1,1),elem!(2,4,1,1),elem!(1,2,0,1),
        elem!(5,4,1,1),elem!(4,6,2,1),elem!(0,2,0,1),elem!(3,6,1,-7),

    // remainder 11
    elem!(0,2,0,1),elem!(6,4,2,1),elem!(1,2,0,1),elem!(7,4,2,1),
        elem!(3,6,2,1),elem!(5,2,1,1),elem!(2,6,2,1),elem!(4,4,2,-7),

    elem!(1,2,0,1),elem!(7,4,2,1),elem!(2,2,1,1),elem!(0,4,1,1),
        elem!(4,6,2,1),elem!(6,2,1,1),elem!(3,6,2,1),elem!(5,4,2,-7),

    elem!(2,2,1,1),elem!(0,4,1,1),elem!(3,2,1,1),elem!(1,4,1,1),
        elem!(5,6,2,1),elem!(7,2,1,1),elem!(4,6,2,1),elem!(6,4,2,-7),

    elem!(3,2,1,1),elem!(1,4,1,1),elem!(4,2,1,1),elem!(2,4,1,1),
        elem!(6,6,3,1),elem!(0,2,0,1),elem!(5,6,2,1),elem!(7,4,2,-7),

    elem!(4,2,1,1),elem!(2,4,1,1),elem!(5,2,1,1),elem!(3,4,1,1),
        elem!(7,6,3,1),elem!(1,2,0,1),elem!(6,6,3,1),elem!(0,4,1,-7),

    elem!(5,2,1,1),elem!(3,4,1,1),elem!(6,2,1,1),elem!(4,4,2,1),
        elem!(0,6,2,1),elem!(2,2,0,1),elem!(7,6,3,1),elem!(1,4,1,-7),

    elem!(6,2,1,1),elem!(4,4,1,1),elem!(7,2,1,1),elem!(5,4,2,1),
        elem!(1,6,2,1),elem!(3,2,1,1),elem!(0,6,2,1),elem!(2,4,1,-7),

    elem!(7,2,1,1),elem!(5,4,2,1),elem!(0,2,0,1),elem!(6,4,2,1),
        elem!(2,6,2,1),elem!(4,2,1,1),elem!(1,6,2,1),elem!(3,4,1,-7),

    // remainder 13
    elem!(0,4,1,1),elem!(5,2,1,1),elem!(4,4,2,1),elem!(2,6,2,1),
        elem!(7,2,1,1),elem!(6,6,3,1),elem!(3,4,2,1),elem!(1,2,1,-7),

    elem!(1,4,1,1),elem!(6,2,1,1),elem!(5,4,2,1),elem!(3,6,3,1),
        elem!(0,2,0,1),elem!(7,6,3,1),elem!(4,4,2,1),elem!(2,2,1,-7),

    elem!(2,4,1,1),elem!(7,2,1,1),elem!(6,4,2,1),elem!(4,6,3,1),
        elem!(1,2,1,1),elem!(0,6,2,1),elem!(5,4,2,1),elem!(3,2,1,-7),

    elem!(3,4,2,1),elem!(0,2,0,1),elem!(7,4,2,1),elem!(5,6,3,1),
        elem!(2,2,1,1),elem!(1,6,2,1),elem!(6,4,2,1),elem!(4,2,1,-7),

    elem!(4,4,2,1),elem!(1,2,1,1),elem!(0,4,1,1),elem!(6,6,3,1),
        elem!(3,2,1,1),elem!(2,6,2,1),elem!(7,4,2,1),elem!(5,2,1,-7),

    elem!(5,4,2,1),elem!(2,2,1,1),elem!(1,4,1,1),elem!(7,6,3,1),
        elem!(4,2,1,1),elem!(3,6,3,1),elem!(0,4,1,1),elem!(6,2,1,-7),

    elem!(6,4,2,1),elem!(3,2,1,1),elem!(2,4,2,1),elem!(0,6,2,1),
        elem!(5,2,1,1),elem!(4,6,3,1),elem!(1,4,1,1),elem!(7,2,1,-7),

    elem!(7,4,2,1),elem!(4,2,1,1),elem!(3,4,2,1),elem!(1,6,2,1),
        elem!(6,2,1,1),elem!(5,6,3,1),elem!(2,4,2,1),elem!(0,2,0,-7),

    // remainder 17
    elem!(0,2,1,1),elem!(1,4,2,1),elem!(3,6,3,1),elem!(6,2,1,1),
        elem!(7,6,4,1),elem!(2,4,2,1),elem!(4,2,1,1),elem!(5,4,3,-7),

    elem!(1,2,1,1),elem!(2,4,2,1),elem!(4,6,3,1),elem!(7,2,2,1),
        elem!(0,6,3,1),elem!(3,4,2,1),elem!(5,2,1,1),elem!(6,4,3,-7),

    elem!(2,2,1,1),elem!(3,4,2,1),elem!(5,6,4,1),elem!(0,2,1,1),
        elem!(1,6,3,1),elem!(4,4,2,1),elem!(6,2,1,1),elem!(7,4,3,-7),

    elem!(3,2,1,1),elem!(4,4,2,1),elem!(6,6,4,1),elem!(1,2,1,1),
        elem!(2,6,3,1),elem!(5,4,2,1),elem!(7,2,2,1),elem!(0,4,2,-7),

    elem!(4,2,1,1),elem!(5,4,2,1),elem!(7,6,4,1),elem!(2,2,1,1),
        elem!(3,6,3,1),elem!(6,4,3,1),elem!(0,2,1,1),elem!(1,4,2,-7),

    elem!(5,2,1,1),elem!(6,4,3,1),elem!(0,6,3,1),elem!(3,2,1,1),
        elem!(4,6,3,1),elem!(7,4,3,1),elem!(1,2,1,1),elem!(2,4,2,-7),

    elem!(6,2,1,1),elem!(7,4,3,1),elem!(1,6,3,1),elem!(4,2,1,1),
        elem!(5,6,4,1),elem!(0,4,2,1),elem!(2,2,1,1),elem!(3,4,2,-7),

    elem!(7,2,2,1),elem!(0,4,2,1),elem!(2,6,3,1),elem!(5,2,1,1),
        elem!(6,6,4,1),elem!(1,4,2,1),elem!(3,2,1,1),elem!(4,4,2,-7),

    // remainder 19
    elem!(0,4,2,1),elem!(4,6,4,1),elem!(2,2,1,1),elem!(5,6,4,1),
        elem!(3,4,2,1),elem!(7,2,2,1),elem!(1,4,2,1),elem!(6,2,2,-7),

    elem!(1,4,2,1),elem!(5,6,4,1),elem!(3,2,1,1),elem!(6,6,4,1),
        elem!(4,4,3,1),elem!(0,2,1,1),elem!(2,4,2,1),elem!(7,2,2,-7),

    elem!(2,4,2,1),elem!(6,6,4,1),elem!(4,2,1,1),elem!(7,6,4,1),
        elem!(5,4,3,1),elem!(1,2,1,1),elem!(3,4,3,1),elem!(0,2,1,-7),

    elem!(3,4,2,1),elem!(7,6,4,1),elem!(5,2,2,1),elem!(0,6,3,1),
        elem!(6,4,3,1),elem!(2,2,1,1),elem!(4,4,3,1),elem!(1,2,1,-7),

    elem!(4,4,3,1),elem!(0,6,3,1),elem!(6,2,2,1),elem!(1,6,3,1),
        elem!(7,4,3,1),elem!(3,2,1,1),elem!(5,4,3,1),elem!(2,2,1,-7),

    elem!(5,4,3,1),elem!(1,6,3,1),elem!(7,2,2,1),elem!(2,6,4,1),
        elem!(0,4,2,1),elem!(4,2,1,1),elem!(6,4,3,1),elem!(3,2,1,-7),

    elem!(6,4,3,1),elem!(2,6,4,1),elem!(0,2,1,1),elem!(3,6,4,1),
        elem!(1,4,2,1),elem!(5,2,1,1),elem!(7,4,3,1),elem!(4,2,1,-7),

    elem!(7,4,3,1),elem!(3,6,4,1),elem!(1,2,1,1),elem!(4,6,4,1),
        elem!(2,4,2,1),elem!(6,2,2,1),elem!(0,4,2,1),elem!(5,2,1,-7),

    // remainder 23
    elem!(0,6,4,1),elem!(4,2,2,1),elem!(1,6,4,1),elem!(5,4,3,1),
        elem!(6,2,2,1),elem!(2,4,3,1),elem!(3,2,1,1),elem!(7,4,4,-7),

    elem!(1,6,4,1),elem!(5,2,2,1),elem!(2,6,4,1),elem!(6,4,3,1),
        elem!(7,2,2,1),elem!(3,4,3,1),elem!(4,2,2,1),elem!(0,4,3,-7),

    elem!(2,6,4,1),elem!(6,2,2,1),elem!(3,6,4,1),elem!(7,4,4,1),
        elem!(0,2,1,1),elem!(4,4,3,1),elem!(5,2,2,1),elem!(1,4,3,-7),

    elem!(3,6,4,1),elem!(7,2,2,1),elem!(4,6,5,1),elem!(0,4,3,1),
        elem!(1,2,1,1),elem!(5,4,3,1),elem!(6,2,2,1),elem!(2,4,3,-7),

    elem!(4,6,5,1),elem!(0,2,1,1),elem!(5,6,5,1),elem!(1,4,3,1),
        elem!(2,2,1,1),elem!(6,4,3,1),elem!(7,2,2,1),elem!(3,4,3,-7),

    elem!(5,6,5,1),elem!(1,2,1,1),elem!(6,6,5,1),elem!(2,4,3,1),
        elem!(3,2,1,1),elem!(7,4,4,1),elem!(0,2,1,1),elem!(4,4,3,-7),

    elem!(6,6,5,1),elem!(2,2,1,1),elem!(7,6,5,1),elem!(3,4,3,1),
        elem!(4,2,2,1),elem!(0,4,3,1),elem!(1,2,1,1),elem!(5,4,3,-7),

    elem!(7,6,5,1),elem!(3,2,2,1),elem!(0,6,4,1),elem!(4,4,3,1),
        elem!(5,2,2,1),elem!(1,4,3,1),elem!(2,2,1,1),elem!(6,4,3,-7),

    // remainder 29
    elem!(0,2,1,1),elem!(7,6,6,1),elem!(6,4,4,1),elem!(5,2,2,1),
        elem!(4,4,4,1),elem!(3,2,2,1),elem!(2,4,4,1),elem!(1,6,6,-7),

    elem!(1,2,2,1),elem!(0,6,5,1),elem!(7,4,4,1),elem!(6,2,2,1),
        elem!(5,4,4,1),elem!(4,2,2,1),elem!(3,4,4,1),elem!(2,6,6,-7),

    elem!(2,2,2,1),elem!(1,6,6,1),elem!(0,4,3,1),elem!(7,2,2,1),
        elem!(6,4,4,1),elem!(5,2,2,1),elem!(4,4,4,1),elem!(3,6,6,-7),

    elem!(3,2,2,1),elem!(2,6,6,1),elem!(1,4,4,1),elem!(0,2,1,1),
        elem!(7,4,4,1),elem!(6,2,2,1),elem!(5,4,4,1),elem!(4,6,6,-7),

    elem!(4,2,2,1),elem!(3,6,6,1),elem!(2,4,4,1),elem!(1,2,2,1),
        elem!(0,4,3,1),elem!(7,2,2,1),elem!(6,4,4,1),elem!(5,6,6,-7),

    elem!(5,2,2,1),elem!(4,6,6,1),elem!(3,4,4,1),elem!(2,2,2,1),
        elem!(1,4,4,1),elem!(0,2,1,1),elem!(7,4,4,1),elem!(6,6,6,-7),

    elem!(6,2,2,1),elem!(5,6,6,1),elem!(4,4,4,1),elem!(3,2,2,1),
        elem!(2,4,4,1),elem!(1,2,2,1),elem!(0,4,3,1),elem!(7,6,6,-7),

    elem!(7,2,2,1),elem!(6,6,6,1),elem!(5,4,4,1),elem!(4,2,2,1),
        elem!(3,4,4,1),elem!(2,2,2,1),elem!(1,4,4,1),elem!(0,6,5,-7),
    ];

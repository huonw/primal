#![allow(dead_code)]

use wheel::{WheelInfo, WheelElem, WheelInit};

///
/// @file   WheelFactorization.cpp
/// @brief  Precomputed arrays for wheel factorization.
///
/// Copyright (C) 2013 Kim Walisch, <kim.walisch@gmail.com>
///
/// This file is distributed under the BSD License. See the COPYING
/// file in the top level directory.
///


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
    let low_offset = mult - low;

    let wheel_index = init.wheel_index as usize * SIZE;
    let sieve_index = low_offset / MODULO * SIZE / 8;

    let ret = WheelInfo {
        true_prime: p,
        prime: p / MODULO,
        sieve_index: sieve_index,
        wheel_index: wheel_index,
    };
    ret
}

pub const SIZE: usize = 8;

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

const WHEEL: &'static [WheelElem; 8*8] = &[
    // remainder 1
    elem!(0u8,6,0,1),elem!(1u8,4,0,1),elem!(2u8,2,0,1),elem!(3u8,4,0,1),
        elem!(4u8,2,0,1),elem!(5u8,4,0,1),elem!(6u8,6,0,1),elem!(7u8,2,1,-7),

    // remainder 7
    elem!(5u8,4,1,1),elem!(4u8,2,1,1),elem!(0u8,4,0,1),elem!(7u8,2,1,1),
        elem!(3u8,4,1,1),elem!(2u8,6,1,1),elem!(6u8,2,1,1),elem!(1u8,6,1,-7),

    // remainder 11
    elem!(0u8,2,0,1),elem!(6u8,4,2,1),elem!(1u8,2,0,1),elem!(7u8,4,2,1),
        elem!(3u8,6,2,1),elem!(5u8,2,1,1),elem!(2u8,6,2,1),elem!(4u8,4,2,-7),

    // remainder 13
    elem!(5u8,4,2,1),elem!(2u8,2,1,1),elem!(1u8,4,1,1),elem!(7u8,6,3,1),
        elem!(4u8,2,1,1),elem!(3u8,6,3,1),elem!(0u8,4,1,1),elem!(6u8,2,1,-7),

    // remainder 17
    elem!(5u8,2,1,1),elem!(6u8,4,3,1),elem!(0u8,6,3,1),elem!(3u8,2,1,1),
        elem!(4u8,6,3,1),elem!(7u8,4,3,1),elem!(1u8,2,1,1),elem!(2u8,4,2,-7),

    // remainder 19
    elem!(0u8,4,2,1),elem!(4u8,6,4,1),elem!(2u8,2,1,1),elem!(5u8,6,4,1),
        elem!(3u8,4,2,1),elem!(7u8,2,2,1),elem!(1u8,4,2,1),elem!(6u8,2,2,-7),

    // remainder 23
    elem!(5u8,6,5,1),elem!(1u8,2,1,1),elem!(6u8,6,5,1),elem!(2u8,4,3,1),
        elem!(3u8,2,1,1),elem!(7u8,4,4,1),elem!(0u8,2,1,1),elem!(4u8,4,3,-7),

    // remainder 29
    elem!(0u8,2,1,1),elem!(7u8,6,6,1),elem!(6u8,4,4,1),elem!(5u8,2,2,1),
        elem!(4u8,4,4,1),elem!(3u8,2,2,1),elem!(2u8,4,4,1),elem!(1u8,6,6,-7),
    ];

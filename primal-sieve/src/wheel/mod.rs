pub const BYTE_SIZE: usize = 8;
pub const BYTE_MODULO: usize = 30;

#[derive(Debug)]
pub struct WheelInfo {
    pub true_prime: usize,
    pub prime: usize,
    pub wheel_index: usize,
    pub sieve_index: usize,
}
#[derive(Debug)]
pub struct WheelInit {
    pub next_mult_factor: u8,
    pub wheel_index: u8,
}
#[macro_escape]
macro_rules! init {
    ($nmf: expr, $wi: expr) => { ::wheel::WheelInit { next_mult_factor: $nmf, wheel_index: $wi }}
}
#[derive(Debug)]
pub struct WheelElem {
    pub unset_bit: u8,
    pub next_mult_factor: u8,
    pub correction: u8,
    pub next: i8,
}

#[macro_escape]
macro_rules! count_ints {
    // count non-recursively, to avoid recursion limits
    ($($x: expr, )*) => { sum!($(0usize * $x as usize + 1usize,)*) }
}
#[macro_escape]
macro_rules! sum {
    ($($x: expr, )*) => { 0 $(+ $x)* }
}

#[macro_escape]
macro_rules! elems {
    ($([$($bit: expr, $nmf: expr, $c: expr, $n: expr;)*],)*) => {
        const WHEEL: &'static [::wheel::WheelElem; sum!($(count_ints!($($bit,)*),)*)] = &[
            $($(::wheel::WheelElem {
                unset_bit: 1u8 << $bit,
                next_mult_factor: $nmf,
                correction: $c,
                next: $n,
            },)*)*
                ];
    }
}

#[inline(always)]
fn raw_set_bit(wheel: &'static [WheelElem],
               x: &mut [u8], si: &mut usize, wi: &mut usize, prime: usize) {
    unsafe {
        let WheelElem { unset_bit, next_mult_factor, correction, next } =
            *wheel.get_unchecked(*wi);
        *x.get_unchecked_mut(*si) |= unset_bit;

        *si += prime * next_mult_factor as usize;
        *si += correction as usize;
        *wi = wi.wrapping_add(next as usize);
    }
}

const WHEEL_OFFSETS: &'static [usize; BYTE_MODULO] = &[
    0, 0, 0, 0, 0, 0,
    0, 1, 0, 0, 0, 2,
    0, 3, 0, 0, 0, 4,
    0, 5, 0, 0, 0, 6,
    0, 0, 0, 0, 0, 7,
    ];

#[inline(always)]
fn raw_compute_elem(init: &'static [WheelInit],
                    modulo: usize, size: usize,
                    p: usize, low: usize) -> WheelInfo {
    let mut mult = p * p;

    let init = &init[p % modulo];
    let next_mult_factor = init.next_mult_factor;
    mult += p * next_mult_factor as usize;

    let low_offset = mult - low;

    let wheel_index = WHEEL_OFFSETS[p % BYTE_MODULO] * size;
    let sieve_index = low_offset * BYTE_SIZE / BYTE_MODULO / 8;

    let ret = WheelInfo {
        true_prime: p,
        prime: p / BYTE_MODULO,
        sieve_index: sieve_index,
        wheel_index: wheel_index,
    };
    ret

}

pub use self::wheel30::{bit_index, from_bit_index};
pub use self::wheel210::{set_bit, compute_wheel_elem, MODULO, SIZE};

mod wheel30;
mod wheel210;

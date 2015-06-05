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

pub use self::wheel30::{bit_index, from_bit_index};
pub use self::wheel210::{set_bit, compute_wheel_elem, MODULO, SIZE};

mod wheel30;
mod wheel210;

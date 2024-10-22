use core::cmp;
use primal_bit::BitVec;

#[cfg(not(feature = "std"))]
use alloc::borrow::ToOwned;

pub const BYTE_SIZE: usize = 8;
pub const BYTE_MODULO: usize = 30;

const WHEEL_OFFSETS: &[usize; BYTE_MODULO] = &[
    0, 0, 0, 0, 0, 0,
    0, 1, 0, 0, 0, 2,
    0, 3, 0, 0, 0, 4,
    0, 5, 0, 0, 0, 6,
    0, 0, 0, 0, 0, 7,
    ];

pub fn small_for(x: usize) -> Option<BitVec> {
    let bits = bits_for(x);
    if bits < wheel30::SMALL_BITS {
        let bytes = (bits + 8 - 1) / 8;
        Some(BitVec::from_bytes(wheel30::SMALL[..bytes].to_owned(), bits))
    } else {
        None
    }
}

pub fn bits_for(x: usize) -> usize {
    // ceil((x * BYTE_SIZE) / BYTE_MODULO)
    // computed using the remainder to avoid overflow
    let d = x / BYTE_MODULO;
    let r = x % BYTE_MODULO;
    d * BYTE_SIZE + (r * BYTE_SIZE + BYTE_MODULO - 1) / BYTE_MODULO
}

pub fn bit_index(n: usize) -> (bool, usize) {
    const POS: &[(bool, u8); BYTE_MODULO] = &[
        // 0
        (false, 0), (true, 0), (false, 1), (false, 1), (false, 1), (false, 1),
        // 6
        (false, 1), (true, 1), (false, 2), (false, 2), (false, 2), (true, 2),
        // 12
        (false, 3), (true, 3), (false, 4), (false, 4), (false, 4), (true, 4),
        // 18
        (false, 5), (true, 5), (false, 6), (false, 6), (false, 6), (true, 6),
        // 24
        (false, 7), (false, 7), (false, 7), (false, 7), (false, 7), (true, 7),
        ];
    let init = &POS[n % BYTE_MODULO];
    (init.0, (n / BYTE_MODULO) * BYTE_SIZE + init.1 as usize)
}

pub fn upper_bound(nbits: usize) -> usize {
    // basically from_bit_index(nbits)-1, but without overflow
    (nbits / BYTE_SIZE)
        .saturating_mul(BYTE_MODULO)
        .saturating_add(TRUE_AT_BIT[nbits % BYTE_SIZE] - 1)
}

#[inline]
pub fn from_bit_index(bit: usize) -> usize {
    (bit / BYTE_SIZE) * BYTE_MODULO + TRUE_AT_BIT[bit % BYTE_SIZE]
}

const TRUE_AT_BIT: &[usize; 8] = &[1, 7, 11, 13, 17, 19, 23, 29];

pub use self::wheel30::Wheel30;
pub use self::wheel210::Wheel210;

pub trait Wheel {
    fn modulo(&self) -> usize;
    fn size(&self) -> usize;
    fn wheel(&self) -> &'static [WheelElem];
    fn init(&self) -> &'static [WheelInit];
    unsafe fn hardcoded_sieve(&self,
                              bytes: &mut [u8], si_: &mut usize, wi_: &mut usize, prime: usize);
}

type SI = u32;
type WI = u16;
#[derive(Debug)]
pub struct State<W> {
    wheel: W,
    prime: u32,
    wheel_index: WI,
    sieve_index: SI,
}
impl<W: Wheel> State<W> {
    pub fn new(w: W, p: usize, low: usize) -> State<W> {
        let q = cmp::max(low / p + 1, p);
        // the smallest (interesting) multiple of p larger than low
        let mut mult = p * q;

        let init = &w.init()[q % w.modulo()];
        // push it up to the smallest multiple that is in the wheel
        mult += p * init.next_mult_factor as usize;

        // find the memory location to write to
        let low_offset = mult - low;
        let sieve_index = low_offset / BYTE_MODULO;
        // and now the right info to write
        let wheel_index = WHEEL_OFFSETS[p % BYTE_MODULO] * w.size() + init.wheel_index as usize;

        let prime = p / BYTE_MODULO;
        State {
            wheel: w,
            prime: prime as u32,
            sieve_index: sieve_index as SI,
            wheel_index: wheel_index as WI,
        }
    }

    #[inline]
    pub fn sieve(&mut self, bytes: &mut [u8]) {
        let bytes = bytes;
        let top = bytes.len();

        let mut si = self.sieve_index as usize;
        let mut wi = self.wheel_index as usize;
        let p = self.prime as usize;
        while si < top {
            raw_set_bit(self.wheel.wheel(),
                        bytes, &mut si, &mut wi, p)
        }
        self.sieve_index = si.wrapping_sub(top) as SI;
        self.wheel_index = wi as WI;
    }
    #[inline]
    pub fn sieve_pair(&mut self, self2: &mut State<W>, bytes: &mut [u8]) {
        let bytes = bytes;
        let top = bytes.len();
        let wheel = self.wheel.wheel();

        let mut si1 = self.sieve_index as usize;
        let mut wi1 = self.wheel_index as usize;
        let p1 = self.prime as usize;
        let mut si2 = self2.sieve_index as usize;
        let mut wi2 = self2.wheel_index as usize;
        let p2 = self2.prime as usize;

        while si1 < top && si2 < top {
            raw_set_bit(wheel,
                        bytes, &mut si1, &mut wi1, p1);
            raw_set_bit(wheel,
                        bytes, &mut si2, &mut wi2, p2);
        }
        while si1 < top {
            raw_set_bit(wheel,
                        bytes, &mut si1, &mut wi1, p1);
        }
        while si2 < top {
            raw_set_bit(wheel,
                        bytes, &mut si2, &mut wi2, p2);
        }

        // if this wraps, we've hit the limit, and so won't be
        // continuing, so whatever, it can be junk.
        self.sieve_index = si1.wrapping_sub(top) as SI;
        self.wheel_index = wi1 as WI;
        self2.sieve_index = si2.wrapping_sub(top) as SI;
        self2.wheel_index = wi2 as WI;
    }
    pub fn sieve_triple(&mut self, self2: &mut State<W>, self3: &mut State<W>,
                        bytes: &mut [u8]) {
        let bytes = bytes;
        let top = bytes.len();
        let wheel = self.wheel.wheel();

        let mut si1 = self.sieve_index as usize;
        let mut wi1 = self.wheel_index as usize;
        let p1 = self.prime as usize;
        let mut si2 = self2.sieve_index as usize;
        let mut wi2 = self2.wheel_index as usize;
        let p2 = self2.prime as usize;
        let mut si3 = self3.sieve_index as usize;
        let mut wi3 = self3.wheel_index as usize;
        let p3 = self3.prime as usize;

        while si1 < top && si2 < top && si3 < top {
            raw_set_bit(wheel,
                        bytes, &mut si1, &mut wi1, p1);
            raw_set_bit(wheel,
                        bytes, &mut si2, &mut wi2, p2);
            raw_set_bit(wheel,
                        bytes, &mut si3, &mut wi3, p3);
        }
        while si1 < top {
            raw_set_bit(wheel,
                        bytes, &mut si1, &mut wi1, p1);
        }
        while si2 < top {
            raw_set_bit(wheel,
                        bytes, &mut si2, &mut wi2, p2);
        }
        while si3 < top {
            raw_set_bit(wheel,
                        bytes, &mut si3, &mut wi3, p3);
        }
        // if this wraps, we've hit the limit, and so won't be
        // continuing, so whatever, it can be junk.
        self.sieve_index = si1.wrapping_sub(top) as SI;
        self.wheel_index = wi1 as WI;
        self2.sieve_index = si2.wrapping_sub(top) as SI;
        self2.wheel_index = wi2 as WI;
        self3.sieve_index = si3.wrapping_sub(top) as SI;
        self3.wheel_index = wi3 as WI;
    }

    pub fn sieve_hardcoded(&mut self, bytes: &mut [u8]) {
        let mut si = self.sieve_index as usize;
        let mut wi = self.wheel_index as usize;
        unsafe {
            self.wheel.hardcoded_sieve(bytes,
                                       &mut si, &mut wi, self.prime as usize)
        }
        self.sieve_index = si as SI;
        self.wheel_index = wi as WI;
    }
}

#[derive(Debug)]
pub struct WheelInit {
    next_mult_factor: u8,
    wheel_index: u8,
}
#[derive(Debug)]
pub struct WheelElem {
    unset_bit: u8,
    next_mult_factor: u8,
    correction: u8,
    next: i8,
}

#[inline(always)]
#[cfg(not(feature = "safe"))]
fn raw_set_bit(wheel: &'static [WheelElem],
               x: &mut [u8], si: &mut usize, wi: &mut usize, prime: usize) {
    unsafe {
        let WheelElem { unset_bit, next_mult_factor, correction, next } =
            *wheel.get_unchecked(*wi);
        *x.get_unchecked_mut(*si) &= unset_bit;
        *si += prime * next_mult_factor as usize;
        *si += correction as usize;
        *wi = wi.wrapping_add(next as usize);
    }
}
#[inline(always)]
#[cfg(feature = "safe")]
fn raw_set_bit(wheel: &'static [WheelElem],
               x: &mut [u8], si: &mut usize, wi: &mut usize, prime: usize) {
    let WheelElem { unset_bit, next_mult_factor, correction, next } = wheel[*wi];
    x[*si] &= unset_bit;
    *si += prime * next_mult_factor as usize;
    *si += correction as usize;
    *wi = wi.wrapping_add(next as usize);
}

mod wheel30;
mod wheel210;

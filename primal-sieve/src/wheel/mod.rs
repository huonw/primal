use primal_bit::BitVec;

pub const BYTE_SIZE: usize = 8;
pub const BYTE_MODULO: usize = 30;

pub fn small_for(x: usize) -> Option<BitVec> {
    let bits = bits_for(x);
    if bits < wheel30::SMALL_BITS {
        let u64s = (bits + 64 - 1) / 64;
        Some(BitVec::from_u64s(wheel30::SMALL[..u64s].to_owned(), bits))
    } else {
        None
    }
}

pub fn bits_for(x: usize) -> usize {
    (x * BYTE_SIZE + BYTE_MODULO - 1) / BYTE_MODULO
}

pub fn bit_index(n: usize) -> (bool, usize) {
    const POS: &'static [(bool, u8); BYTE_MODULO] = &[
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
pub fn from_bit_index(bit: usize) -> usize {
    const TRUE_AT_BIT: &'static [usize; 8] = &[
        1, 7, 11, 13, 17, 19, 23, 29
            ];
    (bit / BYTE_SIZE) * BYTE_MODULO + TRUE_AT_BIT[bit % BYTE_SIZE]
}

pub const TRUE_AT_BIT_64: &'static [usize; 64] = &[
    1 + 30*0, 7 + 30*0, 11 + 30*0, 13 + 30*0, 17 + 30*0, 19 + 30*0, 23 + 30*0, 29 + 30*0,
    1 + 30*1, 7 + 30*1, 11 + 30*1, 13 + 30*1, 17 + 30*1, 19 + 30*1, 23 + 30*1, 29 + 30*1,
    1 + 30*2, 7 + 30*2, 11 + 30*2, 13 + 30*2, 17 + 30*2, 19 + 30*2, 23 + 30*2, 29 + 30*2,
    1 + 30*3, 7 + 30*3, 11 + 30*3, 13 + 30*3, 17 + 30*3, 19 + 30*3, 23 + 30*3, 29 + 30*3,

    1 + 30*4, 7 + 30*4, 11 + 30*4, 13 + 30*4, 17 + 30*4, 19 + 30*4, 23 + 30*4, 29 + 30*4,
    1 + 30*5, 7 + 30*5, 11 + 30*5, 13 + 30*5, 17 + 30*5, 19 + 30*5, 23 + 30*5, 29 + 30*5,
    1 + 30*6, 7 + 30*6, 11 + 30*6, 13 + 30*6, 17 + 30*6, 19 + 30*6, 23 + 30*6, 29 + 30*6,
    1 + 30*7, 7 + 30*7, 11 + 30*7, 13 + 30*7, 17 + 30*7, 19 + 30*7, 23 + 30*7, 29 + 30*7,
    ];

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

const WHEEL_OFFSETS: &'static [usize; BYTE_MODULO] = &[
    0, 0, 0, 0, 0, 0,
    0, 1, 0, 0, 0, 2,
    0, 3, 0, 0, 0, 4,
    0, 5, 0, 0, 0, 6,
    0, 0, 0, 0, 0, 7,
    ];

#[inline(never)]
pub fn compute_wheel_elem<W: Wheel>(w: W, p: usize, low: usize) -> State<W> {
    let mut mult = p * p;

    let init = &w.init()[p % w.modulo()];
    let next_mult_factor = init.next_mult_factor;
    mult += p * next_mult_factor as usize;

    let mut wheel_index = WHEEL_OFFSETS[p % BYTE_MODULO] * w.size();
    let mut sieve_index = mult * BYTE_SIZE / BYTE_MODULO / 8;

    let prime = p / BYTE_MODULO;
    // run the wheel until its above `low`... this is ugly and should be done analytically.
    while sieve_index * BYTE_MODULO < low {
        let WheelElem { next_mult_factor, correction, next, .. } = w.wheel()[wheel_index];
        sieve_index += prime * next_mult_factor as usize;
        sieve_index += correction as usize;
        wheel_index = wheel_index.wrapping_add(next as usize);
    }
    sieve_index -= low / BYTE_MODULO;
    let ret = State {
        wheel: w,
        prime: prime as u32,
        sieve_index: sieve_index as SI,
        wheel_index: wheel_index as WI,
    };
    ret

}

mod wheel30;
mod wheel210;

// automatically generated
use wheel::{WheelInit, Wheel, WheelElem};

#[derive(Debug)]
pub struct Wheel30;
impl Wheel for Wheel30 {
    #[inline(always)]
    fn modulo(&self) -> usize { MODULO }

    #[inline(always)]
    fn size(&self) -> usize { SIZE }

    #[inline(always)]
    fn wheel(&self) -> &'static [WheelElem] { WHEEL }

    #[inline(always)]
    fn init(&self) -> &'static [WheelInit] { INIT }

    #[inline(always)]
    unsafe fn hardcoded_sieve(&self,
                              bytes: &mut [u8], si_: &mut usize, wi_: &mut usize, prime: usize) {
        hardcoded_sieve(bytes, si_, wi_, prime)
    }
}

pub const SIZE: usize = 8;

pub const MODULO: usize = 30;

#[allow(dead_code)]
pub const SMALL_BITS: usize = 2688;
#[allow(dead_code)]
pub const SMALL: &'static [u64; SMALL_BITS / 64] = &[
    0b1111100100111101110110111011011001111110111011111101111111111110,
    0b1001111011101101101001101110101011110011000111100100111111010101,
    0b1010010101011001110111010011101111010011110100110000110011100110,
    0b0101011010100110000111100111100010111101100100100110011101101010,
    0b0101010101001100001010101101111000101101101011011110001101010110,
    0b1111100010100001010101000000001110011111111100001010001111011001,
    0b0011101000010011111101100110011011101001010001001111110100101110,
    0b0101010010111111000100010100010100111010001010110100110010111000,
    0b0100111110001100101111001100100010110011011110101100000110001100,
    0b1110111100010111110000011001101101110001011100010101100000100001,
    0b0100011010001100100000111110010100001000000110101001011001010100,
    0b1000011101011000100011111001001001100101101011101111101101110010,
    0b1010000011100011001001100110010110000001110110001001001011010010,
    0b1001100111101011100000010011110000100110110001110011100000010001,
    0b0100110100110011111100110010010000111110100010000101000110001101,
    0b0100110001011000101101000010101010100111000111001000101101011010,
    0b1100001110000011110111001000001000011001111101100010011001001110,
    0b0000001011001101110011011011010100000010001110001111000100101100,
    0b0011000001111010010011000101011100001100100101000100101010110010,
    0b1111100000100100011011000100010011001011111100010000101101000011,
    0b1000110111101010011100110101110010101000100101010000000100011001,
    0b1100010000011110001000101010011001010000001010111001011000100100,
    0b1001110001110100001011110011101011010100000001100100100011010001,
    0b0010111000010101011010001011111110001000000001010110101000000111,
    0b0001010000001000100110000101000110110111111000110101010101100000,
    0b0010011101110000010010010100110101000101101010100101101010000110,
    0b0110000110000011000000101010101111001010110101011001001111010010,
    0b1010110110101001110000100010001010000111110011100010010000000101,
    0b1011000000010110100010011101000101111000010011011000110000011000,
    0b0101001000100100001101001100000010100010011000101100011101010111,
    0b0100001100001000001000011000110100110010010000000101101010101110,
    0b0110000011100001000110011101100110110110110100101011011000110100,
    0b1001010001111010010001001101000001100000001110010001101001100111,
    0b0001000001010101011101001010100010000011100010000000100110011010,
    0b0011001011001000001000110001111001101000010111011010000100100111,
    0b0011100010110001010010000111001101000100000000110001100111100000,
    0b0001110010110101100110000110000101010111001010101110011011000011,
    0b0010100100000010101011001000111110000001110001010110100000001010,
    0b0010111001100100010011100001000110010100111000110100011100011010,
    0b0001000000000110110001010001010011011100001111011100101100010100,
    0b1110001101000111001100001110100110000010101100010010100111101001,
    0b1011010000110000001100000000101000100101110000110001100100110100,
];
const INIT: &'static [WheelInit; 30] = &[
    WheelInit { next_mult_factor: 1, wheel_index: 0 }, // 0
    WheelInit { next_mult_factor: 0, wheel_index: 0 }, // 1
    WheelInit { next_mult_factor: 5, wheel_index: 1 }, // 2
    WheelInit { next_mult_factor: 4, wheel_index: 1 }, // 3
    WheelInit { next_mult_factor: 3, wheel_index: 1 }, // 4
    WheelInit { next_mult_factor: 2, wheel_index: 1 }, // 5
    WheelInit { next_mult_factor: 1, wheel_index: 1 }, // 6
    WheelInit { next_mult_factor: 0, wheel_index: 1 }, // 7
    WheelInit { next_mult_factor: 3, wheel_index: 2 }, // 8
    WheelInit { next_mult_factor: 2, wheel_index: 2 }, // 9
    WheelInit { next_mult_factor: 1, wheel_index: 2 }, // 10
    WheelInit { next_mult_factor: 0, wheel_index: 2 }, // 11
    WheelInit { next_mult_factor: 1, wheel_index: 3 }, // 12
    WheelInit { next_mult_factor: 0, wheel_index: 3 }, // 13
    WheelInit { next_mult_factor: 3, wheel_index: 4 }, // 14
    WheelInit { next_mult_factor: 2, wheel_index: 4 }, // 15
    WheelInit { next_mult_factor: 1, wheel_index: 4 }, // 16
    WheelInit { next_mult_factor: 0, wheel_index: 4 }, // 17
    WheelInit { next_mult_factor: 1, wheel_index: 5 }, // 18
    WheelInit { next_mult_factor: 0, wheel_index: 5 }, // 19
    WheelInit { next_mult_factor: 3, wheel_index: 6 }, // 20
    WheelInit { next_mult_factor: 2, wheel_index: 6 }, // 21
    WheelInit { next_mult_factor: 1, wheel_index: 6 }, // 22
    WheelInit { next_mult_factor: 0, wheel_index: 6 }, // 23
    WheelInit { next_mult_factor: 5, wheel_index: 7 }, // 24
    WheelInit { next_mult_factor: 4, wheel_index: 7 }, // 25
    WheelInit { next_mult_factor: 3, wheel_index: 7 }, // 26
    WheelInit { next_mult_factor: 2, wheel_index: 7 }, // 27
    WheelInit { next_mult_factor: 1, wheel_index: 7 }, // 28
    WheelInit { next_mult_factor: 0, wheel_index: 7 }, // 29
];
const WHEEL: &'static [WheelElem; 64] = &[
    // remainder 1
    WheelElem { unset_bit: 254, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 1, next: -7 },
    // remainder 7
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 6, correction: 1, next: -7 },
    // remainder 11
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 2, next: -7 },
    // remainder 13
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 1, next: -7 },
    // remainder 17
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 2, next: -7 },
    // remainder 19
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 2, next: -7 },
    // remainder 23
    WheelElem { unset_bit: 223, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 3, next: -7 },
    // remainder 29
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 6, correction: 6, next: -7 },
];
pub unsafe fn hardcoded_sieve(bytes: &mut [u8], si_: &mut usize, wi_: &mut usize, prime: usize) {
    let bytes = bytes;
    let start = bytes.as_mut_ptr();
    let len = bytes.len() as isize;
    let largest_step = ::std::cmp::min(len, (28 * prime + 23) as isize);
    let loop_len = len - largest_step;
    let loop_end = start.offset(loop_len);
    let end = start.offset(len);
    let si = *si_ as isize;
    let mut p = start.offset(si);
    let mut wi = *wi_;
    let prime_ = prime as isize;

    'outer: loop {
    match wi {
        0...7 => { // 30 * x + 1
            loop {
             'label7: loop {
              'label6: loop {
               'label5: loop {
                'label4: loop {
                 'label3: loop {
                  'label2: loop {
                   'label1: loop {
                    'label0: loop {
                     match wi {
                      0 => break 'label0,
                      1 => break 'label1,
                      2 => break 'label2,
                      3 => break 'label3,
                      4 => break 'label4,
                      5 => break 'label5,
                      6 => break 'label6,
                      _ => break 'label7,
                     }
                    }
                    while p < loop_end {
                        p = ::b(p);
                        safe_assert!(start <= p.offset(prime_ * 0 + 0) &&
                                     p.offset(prime_ * 0 + 0) < end);
                        *p.offset(prime_ * 0 + 0) &= 254;
                        safe_assert!(start <= p.offset(prime_ * 6 + 0) &&
                                     p.offset(prime_ * 6 + 0) < end);
                        *p.offset(prime_ * 6 + 0) &= 253;
                        safe_assert!(start <= p.offset(prime_ * 10 + 0) &&
                                     p.offset(prime_ * 10 + 0) < end);
                        *p.offset(prime_ * 10 + 0) &= 251;
                        safe_assert!(start <= p.offset(prime_ * 12 + 0) &&
                                     p.offset(prime_ * 12 + 0) < end);
                        *p.offset(prime_ * 12 + 0) &= 247;
                        safe_assert!(start <= p.offset(prime_ * 16 + 0) &&
                                     p.offset(prime_ * 16 + 0) < end);
                        *p.offset(prime_ * 16 + 0) &= 239;
                        safe_assert!(start <= p.offset(prime_ * 18 + 0) &&
                                     p.offset(prime_ * 18 + 0) < end);
                        *p.offset(prime_ * 18 + 0) &= 223;
                        safe_assert!(start <= p.offset(prime_ * 22 + 0) &&
                                     p.offset(prime_ * 22 + 0) < end);
                        *p.offset(prime_ * 22 + 0) &= 191;
                        safe_assert!(start <= p.offset(prime_ * 28 + 0) &&
                                     p.offset(prime_ * 28 + 0) < end);
                        *p.offset(prime_ * 28 + 0) &= 127;

                        p = p.offset(prime_ * 30 + 1)
                    }
                    if p >= end { wi = 0; break 'outer; }
                    safe_assert!(start <= p && p < end);
                    *p &= 254; p = p.offset(prime_ * 6 + 0);
                    break 'label1
                   }
                   if p >= end { wi = 1; break 'outer; }
                   safe_assert!(start <= p && p < end);
                   *p &= 253; p = p.offset(prime_ * 4 + 0);
                   break 'label2
                  }
                  if p >= end { wi = 2; break 'outer; }
                  safe_assert!(start <= p && p < end);
                  *p &= 251; p = p.offset(prime_ * 2 + 0);
                  break 'label3
                 }
                 if p >= end { wi = 3; break 'outer; }
                 safe_assert!(start <= p && p < end);
                 *p &= 247; p = p.offset(prime_ * 4 + 0);
                 break 'label4
                }
                if p >= end { wi = 4; break 'outer; }
                safe_assert!(start <= p && p < end);
                *p &= 239; p = p.offset(prime_ * 2 + 0);
                break 'label5
               }
               if p >= end { wi = 5; break 'outer; }
               safe_assert!(start <= p && p < end);
               *p &= 223; p = p.offset(prime_ * 4 + 0);
               break 'label6
              }
              if p >= end { wi = 6; break 'outer; }
              safe_assert!(start <= p && p < end);
              *p &= 191; p = p.offset(prime_ * 6 + 0);
              break 'label7
             }
             if p >= end { wi = 7; break 'outer; }
             safe_assert!(start <= p && p < end);
             *p &= 127; p = p.offset(prime_ * 2 + 1);
             wi = 0
            }
        }
        8...15 => { // 30 * x + 7
            loop {
             'label15: loop {
              'label14: loop {
               'label13: loop {
                'label12: loop {
                 'label11: loop {
                  'label10: loop {
                   'label9: loop {
                    'label8: loop {
                     match wi {
                      8 => break 'label8,
                      9 => break 'label9,
                      10 => break 'label10,
                      11 => break 'label11,
                      12 => break 'label12,
                      13 => break 'label13,
                      14 => break 'label14,
                      _ => break 'label15,
                     }
                    }
                    while p < loop_end {
                        p = ::b(p);
                        safe_assert!(start <= p.offset(prime_ * 0 + 0) &&
                                     p.offset(prime_ * 0 + 0) < end);
                        *p.offset(prime_ * 0 + 0) &= 223;
                        safe_assert!(start <= p.offset(prime_ * 4 + 1) &&
                                     p.offset(prime_ * 4 + 1) < end);
                        *p.offset(prime_ * 4 + 1) &= 239;
                        safe_assert!(start <= p.offset(prime_ * 6 + 2) &&
                                     p.offset(prime_ * 6 + 2) < end);
                        *p.offset(prime_ * 6 + 2) &= 254;
                        safe_assert!(start <= p.offset(prime_ * 10 + 2) &&
                                     p.offset(prime_ * 10 + 2) < end);
                        *p.offset(prime_ * 10 + 2) &= 127;
                        safe_assert!(start <= p.offset(prime_ * 12 + 3) &&
                                     p.offset(prime_ * 12 + 3) < end);
                        *p.offset(prime_ * 12 + 3) &= 247;
                        safe_assert!(start <= p.offset(prime_ * 16 + 4) &&
                                     p.offset(prime_ * 16 + 4) < end);
                        *p.offset(prime_ * 16 + 4) &= 251;
                        safe_assert!(start <= p.offset(prime_ * 22 + 5) &&
                                     p.offset(prime_ * 22 + 5) < end);
                        *p.offset(prime_ * 22 + 5) &= 191;
                        safe_assert!(start <= p.offset(prime_ * 24 + 6) &&
                                     p.offset(prime_ * 24 + 6) < end);
                        *p.offset(prime_ * 24 + 6) &= 253;

                        p = p.offset(prime_ * 30 + 7)
                    }
                    if p >= end { wi = 8; break 'outer; }
                    safe_assert!(start <= p && p < end);
                    *p &= 223; p = p.offset(prime_ * 4 + 1);
                    break 'label9
                   }
                   if p >= end { wi = 9; break 'outer; }
                   safe_assert!(start <= p && p < end);
                   *p &= 239; p = p.offset(prime_ * 2 + 1);
                   break 'label10
                  }
                  if p >= end { wi = 10; break 'outer; }
                  safe_assert!(start <= p && p < end);
                  *p &= 254; p = p.offset(prime_ * 4 + 0);
                  break 'label11
                 }
                 if p >= end { wi = 11; break 'outer; }
                 safe_assert!(start <= p && p < end);
                 *p &= 127; p = p.offset(prime_ * 2 + 1);
                 break 'label12
                }
                if p >= end { wi = 12; break 'outer; }
                safe_assert!(start <= p && p < end);
                *p &= 247; p = p.offset(prime_ * 4 + 1);
                break 'label13
               }
               if p >= end { wi = 13; break 'outer; }
               safe_assert!(start <= p && p < end);
               *p &= 251; p = p.offset(prime_ * 6 + 1);
               break 'label14
              }
              if p >= end { wi = 14; break 'outer; }
              safe_assert!(start <= p && p < end);
              *p &= 191; p = p.offset(prime_ * 2 + 1);
              break 'label15
             }
             if p >= end { wi = 15; break 'outer; }
             safe_assert!(start <= p && p < end);
             *p &= 253; p = p.offset(prime_ * 6 + 1);
             wi = 8
            }
        }
        16...23 => { // 30 * x + 11
            loop {
             'label23: loop {
              'label22: loop {
               'label21: loop {
                'label20: loop {
                 'label19: loop {
                  'label18: loop {
                   'label17: loop {
                    'label16: loop {
                     match wi {
                      16 => break 'label16,
                      17 => break 'label17,
                      18 => break 'label18,
                      19 => break 'label19,
                      20 => break 'label20,
                      21 => break 'label21,
                      22 => break 'label22,
                      _ => break 'label23,
                     }
                    }
                    while p < loop_end {
                        p = ::b(p);
                        safe_assert!(start <= p.offset(prime_ * 0 + 0) &&
                                     p.offset(prime_ * 0 + 0) < end);
                        *p.offset(prime_ * 0 + 0) &= 254;
                        safe_assert!(start <= p.offset(prime_ * 2 + 0) &&
                                     p.offset(prime_ * 2 + 0) < end);
                        *p.offset(prime_ * 2 + 0) &= 191;
                        safe_assert!(start <= p.offset(prime_ * 6 + 2) &&
                                     p.offset(prime_ * 6 + 2) < end);
                        *p.offset(prime_ * 6 + 2) &= 253;
                        safe_assert!(start <= p.offset(prime_ * 8 + 2) &&
                                     p.offset(prime_ * 8 + 2) < end);
                        *p.offset(prime_ * 8 + 2) &= 127;
                        safe_assert!(start <= p.offset(prime_ * 12 + 4) &&
                                     p.offset(prime_ * 12 + 4) < end);
                        *p.offset(prime_ * 12 + 4) &= 247;
                        safe_assert!(start <= p.offset(prime_ * 18 + 6) &&
                                     p.offset(prime_ * 18 + 6) < end);
                        *p.offset(prime_ * 18 + 6) &= 223;
                        safe_assert!(start <= p.offset(prime_ * 20 + 7) &&
                                     p.offset(prime_ * 20 + 7) < end);
                        *p.offset(prime_ * 20 + 7) &= 251;
                        safe_assert!(start <= p.offset(prime_ * 26 + 9) &&
                                     p.offset(prime_ * 26 + 9) < end);
                        *p.offset(prime_ * 26 + 9) &= 239;

                        p = p.offset(prime_ * 30 + 11)
                    }
                    if p >= end { wi = 16; break 'outer; }
                    safe_assert!(start <= p && p < end);
                    *p &= 254; p = p.offset(prime_ * 2 + 0);
                    break 'label17
                   }
                   if p >= end { wi = 17; break 'outer; }
                   safe_assert!(start <= p && p < end);
                   *p &= 191; p = p.offset(prime_ * 4 + 2);
                   break 'label18
                  }
                  if p >= end { wi = 18; break 'outer; }
                  safe_assert!(start <= p && p < end);
                  *p &= 253; p = p.offset(prime_ * 2 + 0);
                  break 'label19
                 }
                 if p >= end { wi = 19; break 'outer; }
                 safe_assert!(start <= p && p < end);
                 *p &= 127; p = p.offset(prime_ * 4 + 2);
                 break 'label20
                }
                if p >= end { wi = 20; break 'outer; }
                safe_assert!(start <= p && p < end);
                *p &= 247; p = p.offset(prime_ * 6 + 2);
                break 'label21
               }
               if p >= end { wi = 21; break 'outer; }
               safe_assert!(start <= p && p < end);
               *p &= 223; p = p.offset(prime_ * 2 + 1);
               break 'label22
              }
              if p >= end { wi = 22; break 'outer; }
              safe_assert!(start <= p && p < end);
              *p &= 251; p = p.offset(prime_ * 6 + 2);
              break 'label23
             }
             if p >= end { wi = 23; break 'outer; }
             safe_assert!(start <= p && p < end);
             *p &= 239; p = p.offset(prime_ * 4 + 2);
             wi = 16
            }
        }
        24...31 => { // 30 * x + 13
            loop {
             'label31: loop {
              'label30: loop {
               'label29: loop {
                'label28: loop {
                 'label27: loop {
                  'label26: loop {
                   'label25: loop {
                    'label24: loop {
                     match wi {
                      24 => break 'label24,
                      25 => break 'label25,
                      26 => break 'label26,
                      27 => break 'label27,
                      28 => break 'label28,
                      29 => break 'label29,
                      30 => break 'label30,
                      _ => break 'label31,
                     }
                    }
                    while p < loop_end {
                        p = ::b(p);
                        safe_assert!(start <= p.offset(prime_ * 0 + 0) &&
                                     p.offset(prime_ * 0 + 0) < end);
                        *p.offset(prime_ * 0 + 0) &= 223;
                        safe_assert!(start <= p.offset(prime_ * 4 + 2) &&
                                     p.offset(prime_ * 4 + 2) < end);
                        *p.offset(prime_ * 4 + 2) &= 251;
                        safe_assert!(start <= p.offset(prime_ * 6 + 3) &&
                                     p.offset(prime_ * 6 + 3) < end);
                        *p.offset(prime_ * 6 + 3) &= 253;
                        safe_assert!(start <= p.offset(prime_ * 10 + 4) &&
                                     p.offset(prime_ * 10 + 4) < end);
                        *p.offset(prime_ * 10 + 4) &= 127;
                        safe_assert!(start <= p.offset(prime_ * 16 + 7) &&
                                     p.offset(prime_ * 16 + 7) < end);
                        *p.offset(prime_ * 16 + 7) &= 239;
                        safe_assert!(start <= p.offset(prime_ * 18 + 8) &&
                                     p.offset(prime_ * 18 + 8) < end);
                        *p.offset(prime_ * 18 + 8) &= 247;
                        safe_assert!(start <= p.offset(prime_ * 24 + 11) &&
                                     p.offset(prime_ * 24 + 11) < end);
                        *p.offset(prime_ * 24 + 11) &= 254;
                        safe_assert!(start <= p.offset(prime_ * 28 + 12) &&
                                     p.offset(prime_ * 28 + 12) < end);
                        *p.offset(prime_ * 28 + 12) &= 191;

                        p = p.offset(prime_ * 30 + 13)
                    }
                    if p >= end { wi = 24; break 'outer; }
                    safe_assert!(start <= p && p < end);
                    *p &= 223; p = p.offset(prime_ * 4 + 2);
                    break 'label25
                   }
                   if p >= end { wi = 25; break 'outer; }
                   safe_assert!(start <= p && p < end);
                   *p &= 251; p = p.offset(prime_ * 2 + 1);
                   break 'label26
                  }
                  if p >= end { wi = 26; break 'outer; }
                  safe_assert!(start <= p && p < end);
                  *p &= 253; p = p.offset(prime_ * 4 + 1);
                  break 'label27
                 }
                 if p >= end { wi = 27; break 'outer; }
                 safe_assert!(start <= p && p < end);
                 *p &= 127; p = p.offset(prime_ * 6 + 3);
                 break 'label28
                }
                if p >= end { wi = 28; break 'outer; }
                safe_assert!(start <= p && p < end);
                *p &= 239; p = p.offset(prime_ * 2 + 1);
                break 'label29
               }
               if p >= end { wi = 29; break 'outer; }
               safe_assert!(start <= p && p < end);
               *p &= 247; p = p.offset(prime_ * 6 + 3);
               break 'label30
              }
              if p >= end { wi = 30; break 'outer; }
              safe_assert!(start <= p && p < end);
              *p &= 254; p = p.offset(prime_ * 4 + 1);
              break 'label31
             }
             if p >= end { wi = 31; break 'outer; }
             safe_assert!(start <= p && p < end);
             *p &= 191; p = p.offset(prime_ * 2 + 1);
             wi = 24
            }
        }
        32...39 => { // 30 * x + 17
            loop {
             'label39: loop {
              'label38: loop {
               'label37: loop {
                'label36: loop {
                 'label35: loop {
                  'label34: loop {
                   'label33: loop {
                    'label32: loop {
                     match wi {
                      32 => break 'label32,
                      33 => break 'label33,
                      34 => break 'label34,
                      35 => break 'label35,
                      36 => break 'label36,
                      37 => break 'label37,
                      38 => break 'label38,
                      _ => break 'label39,
                     }
                    }
                    while p < loop_end {
                        p = ::b(p);
                        safe_assert!(start <= p.offset(prime_ * 0 + 0) &&
                                     p.offset(prime_ * 0 + 0) < end);
                        *p.offset(prime_ * 0 + 0) &= 223;
                        safe_assert!(start <= p.offset(prime_ * 2 + 1) &&
                                     p.offset(prime_ * 2 + 1) < end);
                        *p.offset(prime_ * 2 + 1) &= 191;
                        safe_assert!(start <= p.offset(prime_ * 6 + 4) &&
                                     p.offset(prime_ * 6 + 4) < end);
                        *p.offset(prime_ * 6 + 4) &= 254;
                        safe_assert!(start <= p.offset(prime_ * 12 + 7) &&
                                     p.offset(prime_ * 12 + 7) < end);
                        *p.offset(prime_ * 12 + 7) &= 247;
                        safe_assert!(start <= p.offset(prime_ * 14 + 8) &&
                                     p.offset(prime_ * 14 + 8) < end);
                        *p.offset(prime_ * 14 + 8) &= 239;
                        safe_assert!(start <= p.offset(prime_ * 20 + 11) &&
                                     p.offset(prime_ * 20 + 11) < end);
                        *p.offset(prime_ * 20 + 11) &= 127;
                        safe_assert!(start <= p.offset(prime_ * 24 + 14) &&
                                     p.offset(prime_ * 24 + 14) < end);
                        *p.offset(prime_ * 24 + 14) &= 253;
                        safe_assert!(start <= p.offset(prime_ * 26 + 15) &&
                                     p.offset(prime_ * 26 + 15) < end);
                        *p.offset(prime_ * 26 + 15) &= 251;

                        p = p.offset(prime_ * 30 + 17)
                    }
                    if p >= end { wi = 32; break 'outer; }
                    safe_assert!(start <= p && p < end);
                    *p &= 223; p = p.offset(prime_ * 2 + 1);
                    break 'label33
                   }
                   if p >= end { wi = 33; break 'outer; }
                   safe_assert!(start <= p && p < end);
                   *p &= 191; p = p.offset(prime_ * 4 + 3);
                   break 'label34
                  }
                  if p >= end { wi = 34; break 'outer; }
                  safe_assert!(start <= p && p < end);
                  *p &= 254; p = p.offset(prime_ * 6 + 3);
                  break 'label35
                 }
                 if p >= end { wi = 35; break 'outer; }
                 safe_assert!(start <= p && p < end);
                 *p &= 247; p = p.offset(prime_ * 2 + 1);
                 break 'label36
                }
                if p >= end { wi = 36; break 'outer; }
                safe_assert!(start <= p && p < end);
                *p &= 239; p = p.offset(prime_ * 6 + 3);
                break 'label37
               }
               if p >= end { wi = 37; break 'outer; }
               safe_assert!(start <= p && p < end);
               *p &= 127; p = p.offset(prime_ * 4 + 3);
               break 'label38
              }
              if p >= end { wi = 38; break 'outer; }
              safe_assert!(start <= p && p < end);
              *p &= 253; p = p.offset(prime_ * 2 + 1);
              break 'label39
             }
             if p >= end { wi = 39; break 'outer; }
             safe_assert!(start <= p && p < end);
             *p &= 251; p = p.offset(prime_ * 4 + 2);
             wi = 32
            }
        }
        40...47 => { // 30 * x + 19
            loop {
             'label47: loop {
              'label46: loop {
               'label45: loop {
                'label44: loop {
                 'label43: loop {
                  'label42: loop {
                   'label41: loop {
                    'label40: loop {
                     match wi {
                      40 => break 'label40,
                      41 => break 'label41,
                      42 => break 'label42,
                      43 => break 'label43,
                      44 => break 'label44,
                      45 => break 'label45,
                      46 => break 'label46,
                      _ => break 'label47,
                     }
                    }
                    while p < loop_end {
                        p = ::b(p);
                        safe_assert!(start <= p.offset(prime_ * 0 + 0) &&
                                     p.offset(prime_ * 0 + 0) < end);
                        *p.offset(prime_ * 0 + 0) &= 254;
                        safe_assert!(start <= p.offset(prime_ * 4 + 2) &&
                                     p.offset(prime_ * 4 + 2) < end);
                        *p.offset(prime_ * 4 + 2) &= 239;
                        safe_assert!(start <= p.offset(prime_ * 10 + 6) &&
                                     p.offset(prime_ * 10 + 6) < end);
                        *p.offset(prime_ * 10 + 6) &= 251;
                        safe_assert!(start <= p.offset(prime_ * 12 + 7) &&
                                     p.offset(prime_ * 12 + 7) < end);
                        *p.offset(prime_ * 12 + 7) &= 223;
                        safe_assert!(start <= p.offset(prime_ * 18 + 11) &&
                                     p.offset(prime_ * 18 + 11) < end);
                        *p.offset(prime_ * 18 + 11) &= 247;
                        safe_assert!(start <= p.offset(prime_ * 22 + 13) &&
                                     p.offset(prime_ * 22 + 13) < end);
                        *p.offset(prime_ * 22 + 13) &= 127;
                        safe_assert!(start <= p.offset(prime_ * 24 + 15) &&
                                     p.offset(prime_ * 24 + 15) < end);
                        *p.offset(prime_ * 24 + 15) &= 253;
                        safe_assert!(start <= p.offset(prime_ * 28 + 17) &&
                                     p.offset(prime_ * 28 + 17) < end);
                        *p.offset(prime_ * 28 + 17) &= 191;

                        p = p.offset(prime_ * 30 + 19)
                    }
                    if p >= end { wi = 40; break 'outer; }
                    safe_assert!(start <= p && p < end);
                    *p &= 254; p = p.offset(prime_ * 4 + 2);
                    break 'label41
                   }
                   if p >= end { wi = 41; break 'outer; }
                   safe_assert!(start <= p && p < end);
                   *p &= 239; p = p.offset(prime_ * 6 + 4);
                   break 'label42
                  }
                  if p >= end { wi = 42; break 'outer; }
                  safe_assert!(start <= p && p < end);
                  *p &= 251; p = p.offset(prime_ * 2 + 1);
                  break 'label43
                 }
                 if p >= end { wi = 43; break 'outer; }
                 safe_assert!(start <= p && p < end);
                 *p &= 223; p = p.offset(prime_ * 6 + 4);
                 break 'label44
                }
                if p >= end { wi = 44; break 'outer; }
                safe_assert!(start <= p && p < end);
                *p &= 247; p = p.offset(prime_ * 4 + 2);
                break 'label45
               }
               if p >= end { wi = 45; break 'outer; }
               safe_assert!(start <= p && p < end);
               *p &= 127; p = p.offset(prime_ * 2 + 2);
               break 'label46
              }
              if p >= end { wi = 46; break 'outer; }
              safe_assert!(start <= p && p < end);
              *p &= 253; p = p.offset(prime_ * 4 + 2);
              break 'label47
             }
             if p >= end { wi = 47; break 'outer; }
             safe_assert!(start <= p && p < end);
             *p &= 191; p = p.offset(prime_ * 2 + 2);
             wi = 40
            }
        }
        48...55 => { // 30 * x + 23
            loop {
             'label55: loop {
              'label54: loop {
               'label53: loop {
                'label52: loop {
                 'label51: loop {
                  'label50: loop {
                   'label49: loop {
                    'label48: loop {
                     match wi {
                      48 => break 'label48,
                      49 => break 'label49,
                      50 => break 'label50,
                      51 => break 'label51,
                      52 => break 'label52,
                      53 => break 'label53,
                      54 => break 'label54,
                      _ => break 'label55,
                     }
                    }
                    while p < loop_end {
                        p = ::b(p);
                        safe_assert!(start <= p.offset(prime_ * 0 + 0) &&
                                     p.offset(prime_ * 0 + 0) < end);
                        *p.offset(prime_ * 0 + 0) &= 223;
                        safe_assert!(start <= p.offset(prime_ * 6 + 5) &&
                                     p.offset(prime_ * 6 + 5) < end);
                        *p.offset(prime_ * 6 + 5) &= 253;
                        safe_assert!(start <= p.offset(prime_ * 8 + 6) &&
                                     p.offset(prime_ * 8 + 6) < end);
                        *p.offset(prime_ * 8 + 6) &= 191;
                        safe_assert!(start <= p.offset(prime_ * 14 + 11) &&
                                     p.offset(prime_ * 14 + 11) < end);
                        *p.offset(prime_ * 14 + 11) &= 251;
                        safe_assert!(start <= p.offset(prime_ * 18 + 14) &&
                                     p.offset(prime_ * 18 + 14) < end);
                        *p.offset(prime_ * 18 + 14) &= 247;
                        safe_assert!(start <= p.offset(prime_ * 20 + 15) &&
                                     p.offset(prime_ * 20 + 15) < end);
                        *p.offset(prime_ * 20 + 15) &= 127;
                        safe_assert!(start <= p.offset(prime_ * 24 + 19) &&
                                     p.offset(prime_ * 24 + 19) < end);
                        *p.offset(prime_ * 24 + 19) &= 254;
                        safe_assert!(start <= p.offset(prime_ * 26 + 20) &&
                                     p.offset(prime_ * 26 + 20) < end);
                        *p.offset(prime_ * 26 + 20) &= 239;

                        p = p.offset(prime_ * 30 + 23)
                    }
                    if p >= end { wi = 48; break 'outer; }
                    safe_assert!(start <= p && p < end);
                    *p &= 223; p = p.offset(prime_ * 6 + 5);
                    break 'label49
                   }
                   if p >= end { wi = 49; break 'outer; }
                   safe_assert!(start <= p && p < end);
                   *p &= 253; p = p.offset(prime_ * 2 + 1);
                   break 'label50
                  }
                  if p >= end { wi = 50; break 'outer; }
                  safe_assert!(start <= p && p < end);
                  *p &= 191; p = p.offset(prime_ * 6 + 5);
                  break 'label51
                 }
                 if p >= end { wi = 51; break 'outer; }
                 safe_assert!(start <= p && p < end);
                 *p &= 251; p = p.offset(prime_ * 4 + 3);
                 break 'label52
                }
                if p >= end { wi = 52; break 'outer; }
                safe_assert!(start <= p && p < end);
                *p &= 247; p = p.offset(prime_ * 2 + 1);
                break 'label53
               }
               if p >= end { wi = 53; break 'outer; }
               safe_assert!(start <= p && p < end);
               *p &= 127; p = p.offset(prime_ * 4 + 4);
               break 'label54
              }
              if p >= end { wi = 54; break 'outer; }
              safe_assert!(start <= p && p < end);
              *p &= 254; p = p.offset(prime_ * 2 + 1);
              break 'label55
             }
             if p >= end { wi = 55; break 'outer; }
             safe_assert!(start <= p && p < end);
             *p &= 239; p = p.offset(prime_ * 4 + 3);
             wi = 48
            }
        }
        56...63 => { // 30 * x + 29
            loop {
             'label63: loop {
              'label62: loop {
               'label61: loop {
                'label60: loop {
                 'label59: loop {
                  'label58: loop {
                   'label57: loop {
                    'label56: loop {
                     match wi {
                      56 => break 'label56,
                      57 => break 'label57,
                      58 => break 'label58,
                      59 => break 'label59,
                      60 => break 'label60,
                      61 => break 'label61,
                      62 => break 'label62,
                      _ => break 'label63,
                     }
                    }
                    while p < loop_end {
                        p = ::b(p);
                        safe_assert!(start <= p.offset(prime_ * 0 + 0) &&
                                     p.offset(prime_ * 0 + 0) < end);
                        *p.offset(prime_ * 0 + 0) &= 254;
                        safe_assert!(start <= p.offset(prime_ * 2 + 1) &&
                                     p.offset(prime_ * 2 + 1) < end);
                        *p.offset(prime_ * 2 + 1) &= 127;
                        safe_assert!(start <= p.offset(prime_ * 8 + 7) &&
                                     p.offset(prime_ * 8 + 7) < end);
                        *p.offset(prime_ * 8 + 7) &= 191;
                        safe_assert!(start <= p.offset(prime_ * 12 + 11) &&
                                     p.offset(prime_ * 12 + 11) < end);
                        *p.offset(prime_ * 12 + 11) &= 223;
                        safe_assert!(start <= p.offset(prime_ * 14 + 13) &&
                                     p.offset(prime_ * 14 + 13) < end);
                        *p.offset(prime_ * 14 + 13) &= 239;
                        safe_assert!(start <= p.offset(prime_ * 18 + 17) &&
                                     p.offset(prime_ * 18 + 17) < end);
                        *p.offset(prime_ * 18 + 17) &= 247;
                        safe_assert!(start <= p.offset(prime_ * 20 + 19) &&
                                     p.offset(prime_ * 20 + 19) < end);
                        *p.offset(prime_ * 20 + 19) &= 251;
                        safe_assert!(start <= p.offset(prime_ * 24 + 23) &&
                                     p.offset(prime_ * 24 + 23) < end);
                        *p.offset(prime_ * 24 + 23) &= 253;

                        p = p.offset(prime_ * 30 + 29)
                    }
                    if p >= end { wi = 56; break 'outer; }
                    safe_assert!(start <= p && p < end);
                    *p &= 254; p = p.offset(prime_ * 2 + 1);
                    break 'label57
                   }
                   if p >= end { wi = 57; break 'outer; }
                   safe_assert!(start <= p && p < end);
                   *p &= 127; p = p.offset(prime_ * 6 + 6);
                   break 'label58
                  }
                  if p >= end { wi = 58; break 'outer; }
                  safe_assert!(start <= p && p < end);
                  *p &= 191; p = p.offset(prime_ * 4 + 4);
                  break 'label59
                 }
                 if p >= end { wi = 59; break 'outer; }
                 safe_assert!(start <= p && p < end);
                 *p &= 223; p = p.offset(prime_ * 2 + 2);
                 break 'label60
                }
                if p >= end { wi = 60; break 'outer; }
                safe_assert!(start <= p && p < end);
                *p &= 239; p = p.offset(prime_ * 4 + 4);
                break 'label61
               }
               if p >= end { wi = 61; break 'outer; }
               safe_assert!(start <= p && p < end);
               *p &= 247; p = p.offset(prime_ * 2 + 2);
               break 'label62
              }
              if p >= end { wi = 62; break 'outer; }
              safe_assert!(start <= p && p < end);
              *p &= 251; p = p.offset(prime_ * 4 + 4);
              break 'label63
             }
             if p >= end { wi = 63; break 'outer; }
             safe_assert!(start <= p && p < end);
             *p &= 253; p = p.offset(prime_ * 6 + 6);
             wi = 56
            }
        }
        _ => unreachable!("{}", wi),
    }
    }
    *si_ = (p as usize).wrapping_sub(end as usize);
    *wi_ = wi;
}

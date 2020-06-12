// automatically generated
use wheel::{WheelInit, Wheel, WheelElem};

#[derive(Debug, Clone)]
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
pub const SMALL_BITS: usize = 2672;
#[allow(dead_code)]
pub const SMALL: &'static [u8; SMALL_BITS / 8] = &[
    0b11111110, 0b11011111, 0b11101111, 0b01111110, 0b10110110, 0b11011011, 0b00111101, 0b11111001,
    0b11010101, 0b01001111, 0b00011110, 0b11110011, 0b11101010, 0b10100110, 0b11101101, 0b10011110,
    0b11100110, 0b00001100, 0b11010011, 0b11010011, 0b00111011, 0b11011101, 0b01011001, 0b10100101,
    0b01101010, 0b01100111, 0b10010010, 0b10111101, 0b01111000, 0b00011110, 0b10100110, 0b01010110,
    0b01010110, 0b11100011, 0b10101101, 0b00101101, 0b11011110, 0b00101010, 0b01001100, 0b01010101,
    0b11011001, 0b10100011, 0b11110000, 0b10011111, 0b00000011, 0b01010100, 0b10100001, 0b11111000,
    0b00101110, 0b11111101, 0b01000100, 0b11101001, 0b01100110, 0b11110110, 0b00010011, 0b00111010,
    0b10111000, 0b01001100, 0b00101011, 0b00111010, 0b01000101, 0b00010001, 0b10111111, 0b01010100,
    0b10001100, 0b11000001, 0b01111010, 0b10110011, 0b11001000, 0b10111100, 0b10001100, 0b01001111,
    0b00100001, 0b01011000, 0b01110001, 0b01110001, 0b10011011, 0b11000001, 0b00010111, 0b11101111,
    0b01010100, 0b10010110, 0b00011010, 0b00001000, 0b11100101, 0b10000011, 0b10001100, 0b01000110,
    0b01110010, 0b11111011, 0b10101110, 0b01100101, 0b10010010, 0b10001111, 0b01011000, 0b10000111,
    0b11010010, 0b10010010, 0b11011000, 0b10000001, 0b01100101, 0b00100110, 0b11100011, 0b10100000,
    0b00010001, 0b00111000, 0b11000111, 0b00100110, 0b00111100, 0b10000001, 0b11101011, 0b10011001,
    0b10001101, 0b01010001, 0b10001000, 0b00111110, 0b00100100, 0b11110011, 0b00110011, 0b01001101,
    0b01011010, 0b10001011, 0b00011100, 0b10100111, 0b00101010, 0b10110100, 0b01011000, 0b01001100,
    0b01001110, 0b00100110, 0b11110110, 0b00011001, 0b10000010, 0b11011100, 0b10000011, 0b11000011,
    0b00101100, 0b11110001, 0b00111000, 0b00000010, 0b10110101, 0b11001101, 0b11001101, 0b00000010,
    0b10110010, 0b01001010, 0b10010100, 0b00001100, 0b01010111, 0b01001100, 0b01111010, 0b00110000,
    0b01000011, 0b00001011, 0b11110001, 0b11001011, 0b01000100, 0b01101100, 0b00100100, 0b11111000,
    0b00011001, 0b00000001, 0b10010101, 0b10101000, 0b01011100, 0b01110011, 0b11101010, 0b10001101,
    0b00100100, 0b10010110, 0b00101011, 0b01010000, 0b10100110, 0b00100010, 0b00011110, 0b11000100,
    0b11010001, 0b01001000, 0b00000110, 0b11010100, 0b00111010, 0b00101111, 0b01110100, 0b10011100,
    0b00000111, 0b01101010, 0b00000101, 0b10001000, 0b10111111, 0b01101000, 0b00010101, 0b00101110,
    0b01100000, 0b01010101, 0b11100011, 0b10110111, 0b01010001, 0b10011000, 0b00001000, 0b00010100,
    0b10000110, 0b01011010, 0b10101010, 0b01000101, 0b01001101, 0b01001001, 0b01110000, 0b00100111,
    0b11010010, 0b10010011, 0b11010101, 0b11001010, 0b10101011, 0b00000010, 0b10000011, 0b01100001,
    0b00000101, 0b00100100, 0b11001110, 0b10000111, 0b00100010, 0b11000010, 0b10101001, 0b10101101,
    0b00011000, 0b10001100, 0b01001101, 0b01111000, 0b11010001, 0b10001001, 0b00010110, 0b10110000,
    0b01010111, 0b11000111, 0b01100010, 0b10100010, 0b11000000, 0b00110100, 0b00100100, 0b01010010,
    0b10101110, 0b01011010, 0b01000000, 0b00110010, 0b10001101, 0b00100001, 0b00001000, 0b01000011,
    0b00110100, 0b10110110, 0b11010010, 0b10110110, 0b11011001, 0b00011001, 0b11100001, 0b01100000,
    0b01100111, 0b00011010, 0b00111001, 0b01100000, 0b11010000, 0b01000100, 0b01111010, 0b10010100,
    0b10011010, 0b00001001, 0b10001000, 0b10000011, 0b10101000, 0b01110100, 0b01010101, 0b00010000,
    0b00100111, 0b10100001, 0b01011101, 0b01101000, 0b00011110, 0b00100011, 0b11001000, 0b00110010,
    0b11100000, 0b00011001, 0b00000011, 0b01000100, 0b01110011, 0b01001000, 0b10110001, 0b00111000,
    0b11000011, 0b11100110, 0b00101010, 0b01010111, 0b01100001, 0b10011000, 0b10110101, 0b00011100,
    0b00001010, 0b01101000, 0b11000101, 0b10000001, 0b10001111, 0b10101100, 0b00000010, 0b00101001,
    0b00011010, 0b01000111, 0b11100011, 0b10010100, 0b00010001, 0b01001110, 0b01100100, 0b00101110,
    0b00010100, 0b11001011, 0b00111101, 0b11011100, 0b00010100, 0b11000101, 0b00000110, 0b00010000,
    0b11101001, 0b00101001, 0b10110001, 0b10000010, 0b11101001, 0b00110000, 0b01000111, 0b11100011,
    0b00110100, 0b00011001, 0b11000011, 0b00100101, 0b00001010, 0b00110000,
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
    WheelElem { unset_bit: 253, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 1, next: -7 },
    // remainder 11
    WheelElem { unset_bit: 251, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 1, next: -7 },
    // remainder 13
    WheelElem { unset_bit: 247, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 2, correction: 1, next: -7 },
    // remainder 17
    WheelElem { unset_bit: 239, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 1, next: -7 },
    // remainder 19
    WheelElem { unset_bit: 223, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 2, correction: 1, next: -7 },
    // remainder 23
    WheelElem { unset_bit: 191, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 127, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 2, correction: 1, next: -7 },
    // remainder 29
    WheelElem { unset_bit: 127, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 191, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 223, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 239, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 247, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 251, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 253, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 254, next_mult_factor: 2, correction: 1, next: -7 },
];
pub unsafe fn hardcoded_sieve(bytes: &mut [u8], si_: &mut usize, wi_: &mut usize, prime: usize) {
    let bytes = bytes;
    let start = bytes.as_mut_ptr();
    let len = bytes.len() as isize;
    let largest_step = ::std::cmp::min(len, 30 * (prime as isize + 1) - 1);
    let loop_len = len - largest_step;
    let loop_end = start.offset(loop_len);
    let end = start.offset(len);
    let si = *si_ as isize;
    let mut p = start.offset(si);
    let mut wi = *wi_;
    let prime_ = prime as isize;

    'outer: loop {
    match wi {
        0..=7 => { // 30 * x + 1
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
        8..=15 => { // 30 * x + 7
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
                        *p.offset(prime_ * 0 + 0) &= 253;
                        safe_assert!(start <= p.offset(prime_ * 6 + 1) &&
                                     p.offset(prime_ * 6 + 1) < end);
                        *p.offset(prime_ * 6 + 1) &= 223;
                        safe_assert!(start <= p.offset(prime_ * 10 + 2) &&
                                     p.offset(prime_ * 10 + 2) < end);
                        *p.offset(prime_ * 10 + 2) &= 239;
                        safe_assert!(start <= p.offset(prime_ * 12 + 3) &&
                                     p.offset(prime_ * 12 + 3) < end);
                        *p.offset(prime_ * 12 + 3) &= 254;
                        safe_assert!(start <= p.offset(prime_ * 16 + 3) &&
                                     p.offset(prime_ * 16 + 3) < end);
                        *p.offset(prime_ * 16 + 3) &= 127;
                        safe_assert!(start <= p.offset(prime_ * 18 + 4) &&
                                     p.offset(prime_ * 18 + 4) < end);
                        *p.offset(prime_ * 18 + 4) &= 247;
                        safe_assert!(start <= p.offset(prime_ * 22 + 5) &&
                                     p.offset(prime_ * 22 + 5) < end);
                        *p.offset(prime_ * 22 + 5) &= 251;
                        safe_assert!(start <= p.offset(prime_ * 28 + 6) &&
                                     p.offset(prime_ * 28 + 6) < end);
                        *p.offset(prime_ * 28 + 6) &= 191;

                        p = p.offset(prime_ * 30 + 7)
                    }
                    if p >= end { wi = 8; break 'outer; }
                    safe_assert!(start <= p && p < end);
                    *p &= 253; p = p.offset(prime_ * 6 + 1);
                    break 'label9
                   }
                   if p >= end { wi = 9; break 'outer; }
                   safe_assert!(start <= p && p < end);
                   *p &= 223; p = p.offset(prime_ * 4 + 1);
                   break 'label10
                  }
                  if p >= end { wi = 10; break 'outer; }
                  safe_assert!(start <= p && p < end);
                  *p &= 239; p = p.offset(prime_ * 2 + 1);
                  break 'label11
                 }
                 if p >= end { wi = 11; break 'outer; }
                 safe_assert!(start <= p && p < end);
                 *p &= 254; p = p.offset(prime_ * 4 + 0);
                 break 'label12
                }
                if p >= end { wi = 12; break 'outer; }
                safe_assert!(start <= p && p < end);
                *p &= 127; p = p.offset(prime_ * 2 + 1);
                break 'label13
               }
               if p >= end { wi = 13; break 'outer; }
               safe_assert!(start <= p && p < end);
               *p &= 247; p = p.offset(prime_ * 4 + 1);
               break 'label14
              }
              if p >= end { wi = 14; break 'outer; }
              safe_assert!(start <= p && p < end);
              *p &= 251; p = p.offset(prime_ * 6 + 1);
              break 'label15
             }
             if p >= end { wi = 15; break 'outer; }
             safe_assert!(start <= p && p < end);
             *p &= 191; p = p.offset(prime_ * 2 + 1);
             wi = 8
            }
        }
        16..=23 => { // 30 * x + 11
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
                        *p.offset(prime_ * 0 + 0) &= 251;
                        safe_assert!(start <= p.offset(prime_ * 6 + 2) &&
                                     p.offset(prime_ * 6 + 2) < end);
                        *p.offset(prime_ * 6 + 2) &= 239;
                        safe_assert!(start <= p.offset(prime_ * 10 + 4) &&
                                     p.offset(prime_ * 10 + 4) < end);
                        *p.offset(prime_ * 10 + 4) &= 254;
                        safe_assert!(start <= p.offset(prime_ * 12 + 4) &&
                                     p.offset(prime_ * 12 + 4) < end);
                        *p.offset(prime_ * 12 + 4) &= 191;
                        safe_assert!(start <= p.offset(prime_ * 16 + 6) &&
                                     p.offset(prime_ * 16 + 6) < end);
                        *p.offset(prime_ * 16 + 6) &= 253;
                        safe_assert!(start <= p.offset(prime_ * 18 + 6) &&
                                     p.offset(prime_ * 18 + 6) < end);
                        *p.offset(prime_ * 18 + 6) &= 127;
                        safe_assert!(start <= p.offset(prime_ * 22 + 8) &&
                                     p.offset(prime_ * 22 + 8) < end);
                        *p.offset(prime_ * 22 + 8) &= 247;
                        safe_assert!(start <= p.offset(prime_ * 28 + 10) &&
                                     p.offset(prime_ * 28 + 10) < end);
                        *p.offset(prime_ * 28 + 10) &= 223;

                        p = p.offset(prime_ * 30 + 11)
                    }
                    if p >= end { wi = 16; break 'outer; }
                    safe_assert!(start <= p && p < end);
                    *p &= 251; p = p.offset(prime_ * 6 + 2);
                    break 'label17
                   }
                   if p >= end { wi = 17; break 'outer; }
                   safe_assert!(start <= p && p < end);
                   *p &= 239; p = p.offset(prime_ * 4 + 2);
                   break 'label18
                  }
                  if p >= end { wi = 18; break 'outer; }
                  safe_assert!(start <= p && p < end);
                  *p &= 254; p = p.offset(prime_ * 2 + 0);
                  break 'label19
                 }
                 if p >= end { wi = 19; break 'outer; }
                 safe_assert!(start <= p && p < end);
                 *p &= 191; p = p.offset(prime_ * 4 + 2);
                 break 'label20
                }
                if p >= end { wi = 20; break 'outer; }
                safe_assert!(start <= p && p < end);
                *p &= 253; p = p.offset(prime_ * 2 + 0);
                break 'label21
               }
               if p >= end { wi = 21; break 'outer; }
               safe_assert!(start <= p && p < end);
               *p &= 127; p = p.offset(prime_ * 4 + 2);
               break 'label22
              }
              if p >= end { wi = 22; break 'outer; }
              safe_assert!(start <= p && p < end);
              *p &= 247; p = p.offset(prime_ * 6 + 2);
              break 'label23
             }
             if p >= end { wi = 23; break 'outer; }
             safe_assert!(start <= p && p < end);
             *p &= 223; p = p.offset(prime_ * 2 + 1);
             wi = 16
            }
        }
        24..=31 => { // 30 * x + 13
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
                        *p.offset(prime_ * 0 + 0) &= 247;
                        safe_assert!(start <= p.offset(prime_ * 6 + 3) &&
                                     p.offset(prime_ * 6 + 3) < end);
                        *p.offset(prime_ * 6 + 3) &= 254;
                        safe_assert!(start <= p.offset(prime_ * 10 + 4) &&
                                     p.offset(prime_ * 10 + 4) < end);
                        *p.offset(prime_ * 10 + 4) &= 191;
                        safe_assert!(start <= p.offset(prime_ * 12 + 5) &&
                                     p.offset(prime_ * 12 + 5) < end);
                        *p.offset(prime_ * 12 + 5) &= 223;
                        safe_assert!(start <= p.offset(prime_ * 16 + 7) &&
                                     p.offset(prime_ * 16 + 7) < end);
                        *p.offset(prime_ * 16 + 7) &= 251;
                        safe_assert!(start <= p.offset(prime_ * 18 + 8) &&
                                     p.offset(prime_ * 18 + 8) < end);
                        *p.offset(prime_ * 18 + 8) &= 253;
                        safe_assert!(start <= p.offset(prime_ * 22 + 9) &&
                                     p.offset(prime_ * 22 + 9) < end);
                        *p.offset(prime_ * 22 + 9) &= 127;
                        safe_assert!(start <= p.offset(prime_ * 28 + 12) &&
                                     p.offset(prime_ * 28 + 12) < end);
                        *p.offset(prime_ * 28 + 12) &= 239;

                        p = p.offset(prime_ * 30 + 13)
                    }
                    if p >= end { wi = 24; break 'outer; }
                    safe_assert!(start <= p && p < end);
                    *p &= 247; p = p.offset(prime_ * 6 + 3);
                    break 'label25
                   }
                   if p >= end { wi = 25; break 'outer; }
                   safe_assert!(start <= p && p < end);
                   *p &= 254; p = p.offset(prime_ * 4 + 1);
                   break 'label26
                  }
                  if p >= end { wi = 26; break 'outer; }
                  safe_assert!(start <= p && p < end);
                  *p &= 191; p = p.offset(prime_ * 2 + 1);
                  break 'label27
                 }
                 if p >= end { wi = 27; break 'outer; }
                 safe_assert!(start <= p && p < end);
                 *p &= 223; p = p.offset(prime_ * 4 + 2);
                 break 'label28
                }
                if p >= end { wi = 28; break 'outer; }
                safe_assert!(start <= p && p < end);
                *p &= 251; p = p.offset(prime_ * 2 + 1);
                break 'label29
               }
               if p >= end { wi = 29; break 'outer; }
               safe_assert!(start <= p && p < end);
               *p &= 253; p = p.offset(prime_ * 4 + 1);
               break 'label30
              }
              if p >= end { wi = 30; break 'outer; }
              safe_assert!(start <= p && p < end);
              *p &= 127; p = p.offset(prime_ * 6 + 3);
              break 'label31
             }
             if p >= end { wi = 31; break 'outer; }
             safe_assert!(start <= p && p < end);
             *p &= 239; p = p.offset(prime_ * 2 + 1);
             wi = 24
            }
        }
        32..=39 => { // 30 * x + 17
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
                        *p.offset(prime_ * 0 + 0) &= 239;
                        safe_assert!(start <= p.offset(prime_ * 6 + 3) &&
                                     p.offset(prime_ * 6 + 3) < end);
                        *p.offset(prime_ * 6 + 3) &= 127;
                        safe_assert!(start <= p.offset(prime_ * 10 + 6) &&
                                     p.offset(prime_ * 10 + 6) < end);
                        *p.offset(prime_ * 10 + 6) &= 253;
                        safe_assert!(start <= p.offset(prime_ * 12 + 7) &&
                                     p.offset(prime_ * 12 + 7) < end);
                        *p.offset(prime_ * 12 + 7) &= 251;
                        safe_assert!(start <= p.offset(prime_ * 16 + 9) &&
                                     p.offset(prime_ * 16 + 9) < end);
                        *p.offset(prime_ * 16 + 9) &= 223;
                        safe_assert!(start <= p.offset(prime_ * 18 + 10) &&
                                     p.offset(prime_ * 18 + 10) < end);
                        *p.offset(prime_ * 18 + 10) &= 191;
                        safe_assert!(start <= p.offset(prime_ * 22 + 13) &&
                                     p.offset(prime_ * 22 + 13) < end);
                        *p.offset(prime_ * 22 + 13) &= 254;
                        safe_assert!(start <= p.offset(prime_ * 28 + 16) &&
                                     p.offset(prime_ * 28 + 16) < end);
                        *p.offset(prime_ * 28 + 16) &= 247;

                        p = p.offset(prime_ * 30 + 17)
                    }
                    if p >= end { wi = 32; break 'outer; }
                    safe_assert!(start <= p && p < end);
                    *p &= 239; p = p.offset(prime_ * 6 + 3);
                    break 'label33
                   }
                   if p >= end { wi = 33; break 'outer; }
                   safe_assert!(start <= p && p < end);
                   *p &= 127; p = p.offset(prime_ * 4 + 3);
                   break 'label34
                  }
                  if p >= end { wi = 34; break 'outer; }
                  safe_assert!(start <= p && p < end);
                  *p &= 253; p = p.offset(prime_ * 2 + 1);
                  break 'label35
                 }
                 if p >= end { wi = 35; break 'outer; }
                 safe_assert!(start <= p && p < end);
                 *p &= 251; p = p.offset(prime_ * 4 + 2);
                 break 'label36
                }
                if p >= end { wi = 36; break 'outer; }
                safe_assert!(start <= p && p < end);
                *p &= 223; p = p.offset(prime_ * 2 + 1);
                break 'label37
               }
               if p >= end { wi = 37; break 'outer; }
               safe_assert!(start <= p && p < end);
               *p &= 191; p = p.offset(prime_ * 4 + 3);
               break 'label38
              }
              if p >= end { wi = 38; break 'outer; }
              safe_assert!(start <= p && p < end);
              *p &= 254; p = p.offset(prime_ * 6 + 3);
              break 'label39
             }
             if p >= end { wi = 39; break 'outer; }
             safe_assert!(start <= p && p < end);
             *p &= 247; p = p.offset(prime_ * 2 + 1);
             wi = 32
            }
        }
        40..=47 => { // 30 * x + 19
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
                        *p.offset(prime_ * 0 + 0) &= 223;
                        safe_assert!(start <= p.offset(prime_ * 6 + 4) &&
                                     p.offset(prime_ * 6 + 4) < end);
                        *p.offset(prime_ * 6 + 4) &= 247;
                        safe_assert!(start <= p.offset(prime_ * 10 + 6) &&
                                     p.offset(prime_ * 10 + 6) < end);
                        *p.offset(prime_ * 10 + 6) &= 127;
                        safe_assert!(start <= p.offset(prime_ * 12 + 8) &&
                                     p.offset(prime_ * 12 + 8) < end);
                        *p.offset(prime_ * 12 + 8) &= 253;
                        safe_assert!(start <= p.offset(prime_ * 16 + 10) &&
                                     p.offset(prime_ * 16 + 10) < end);
                        *p.offset(prime_ * 16 + 10) &= 191;
                        safe_assert!(start <= p.offset(prime_ * 18 + 12) &&
                                     p.offset(prime_ * 18 + 12) < end);
                        *p.offset(prime_ * 18 + 12) &= 254;
                        safe_assert!(start <= p.offset(prime_ * 22 + 14) &&
                                     p.offset(prime_ * 22 + 14) < end);
                        *p.offset(prime_ * 22 + 14) &= 239;
                        safe_assert!(start <= p.offset(prime_ * 28 + 18) &&
                                     p.offset(prime_ * 28 + 18) < end);
                        *p.offset(prime_ * 28 + 18) &= 251;

                        p = p.offset(prime_ * 30 + 19)
                    }
                    if p >= end { wi = 40; break 'outer; }
                    safe_assert!(start <= p && p < end);
                    *p &= 223; p = p.offset(prime_ * 6 + 4);
                    break 'label41
                   }
                   if p >= end { wi = 41; break 'outer; }
                   safe_assert!(start <= p && p < end);
                   *p &= 247; p = p.offset(prime_ * 4 + 2);
                   break 'label42
                  }
                  if p >= end { wi = 42; break 'outer; }
                  safe_assert!(start <= p && p < end);
                  *p &= 127; p = p.offset(prime_ * 2 + 2);
                  break 'label43
                 }
                 if p >= end { wi = 43; break 'outer; }
                 safe_assert!(start <= p && p < end);
                 *p &= 253; p = p.offset(prime_ * 4 + 2);
                 break 'label44
                }
                if p >= end { wi = 44; break 'outer; }
                safe_assert!(start <= p && p < end);
                *p &= 191; p = p.offset(prime_ * 2 + 2);
                break 'label45
               }
               if p >= end { wi = 45; break 'outer; }
               safe_assert!(start <= p && p < end);
               *p &= 254; p = p.offset(prime_ * 4 + 2);
               break 'label46
              }
              if p >= end { wi = 46; break 'outer; }
              safe_assert!(start <= p && p < end);
              *p &= 239; p = p.offset(prime_ * 6 + 4);
              break 'label47
             }
             if p >= end { wi = 47; break 'outer; }
             safe_assert!(start <= p && p < end);
             *p &= 251; p = p.offset(prime_ * 2 + 1);
             wi = 40
            }
        }
        48..=55 => { // 30 * x + 23
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
                        *p.offset(prime_ * 0 + 0) &= 191;
                        safe_assert!(start <= p.offset(prime_ * 6 + 5) &&
                                     p.offset(prime_ * 6 + 5) < end);
                        *p.offset(prime_ * 6 + 5) &= 251;
                        safe_assert!(start <= p.offset(prime_ * 10 + 8) &&
                                     p.offset(prime_ * 10 + 8) < end);
                        *p.offset(prime_ * 10 + 8) &= 247;
                        safe_assert!(start <= p.offset(prime_ * 12 + 9) &&
                                     p.offset(prime_ * 12 + 9) < end);
                        *p.offset(prime_ * 12 + 9) &= 127;
                        safe_assert!(start <= p.offset(prime_ * 16 + 13) &&
                                     p.offset(prime_ * 16 + 13) < end);
                        *p.offset(prime_ * 16 + 13) &= 254;
                        safe_assert!(start <= p.offset(prime_ * 18 + 14) &&
                                     p.offset(prime_ * 18 + 14) < end);
                        *p.offset(prime_ * 18 + 14) &= 239;
                        safe_assert!(start <= p.offset(prime_ * 22 + 17) &&
                                     p.offset(prime_ * 22 + 17) < end);
                        *p.offset(prime_ * 22 + 17) &= 223;
                        safe_assert!(start <= p.offset(prime_ * 28 + 22) &&
                                     p.offset(prime_ * 28 + 22) < end);
                        *p.offset(prime_ * 28 + 22) &= 253;

                        p = p.offset(prime_ * 30 + 23)
                    }
                    if p >= end { wi = 48; break 'outer; }
                    safe_assert!(start <= p && p < end);
                    *p &= 191; p = p.offset(prime_ * 6 + 5);
                    break 'label49
                   }
                   if p >= end { wi = 49; break 'outer; }
                   safe_assert!(start <= p && p < end);
                   *p &= 251; p = p.offset(prime_ * 4 + 3);
                   break 'label50
                  }
                  if p >= end { wi = 50; break 'outer; }
                  safe_assert!(start <= p && p < end);
                  *p &= 247; p = p.offset(prime_ * 2 + 1);
                  break 'label51
                 }
                 if p >= end { wi = 51; break 'outer; }
                 safe_assert!(start <= p && p < end);
                 *p &= 127; p = p.offset(prime_ * 4 + 4);
                 break 'label52
                }
                if p >= end { wi = 52; break 'outer; }
                safe_assert!(start <= p && p < end);
                *p &= 254; p = p.offset(prime_ * 2 + 1);
                break 'label53
               }
               if p >= end { wi = 53; break 'outer; }
               safe_assert!(start <= p && p < end);
               *p &= 239; p = p.offset(prime_ * 4 + 3);
               break 'label54
              }
              if p >= end { wi = 54; break 'outer; }
              safe_assert!(start <= p && p < end);
              *p &= 223; p = p.offset(prime_ * 6 + 5);
              break 'label55
             }
             if p >= end { wi = 55; break 'outer; }
             safe_assert!(start <= p && p < end);
             *p &= 253; p = p.offset(prime_ * 2 + 1);
             wi = 48
            }
        }
        56..=63 => { // 30 * x + 29
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
                        *p.offset(prime_ * 0 + 0) &= 127;
                        safe_assert!(start <= p.offset(prime_ * 6 + 6) &&
                                     p.offset(prime_ * 6 + 6) < end);
                        *p.offset(prime_ * 6 + 6) &= 191;
                        safe_assert!(start <= p.offset(prime_ * 10 + 10) &&
                                     p.offset(prime_ * 10 + 10) < end);
                        *p.offset(prime_ * 10 + 10) &= 223;
                        safe_assert!(start <= p.offset(prime_ * 12 + 12) &&
                                     p.offset(prime_ * 12 + 12) < end);
                        *p.offset(prime_ * 12 + 12) &= 239;
                        safe_assert!(start <= p.offset(prime_ * 16 + 16) &&
                                     p.offset(prime_ * 16 + 16) < end);
                        *p.offset(prime_ * 16 + 16) &= 247;
                        safe_assert!(start <= p.offset(prime_ * 18 + 18) &&
                                     p.offset(prime_ * 18 + 18) < end);
                        *p.offset(prime_ * 18 + 18) &= 251;
                        safe_assert!(start <= p.offset(prime_ * 22 + 22) &&
                                     p.offset(prime_ * 22 + 22) < end);
                        *p.offset(prime_ * 22 + 22) &= 253;
                        safe_assert!(start <= p.offset(prime_ * 28 + 28) &&
                                     p.offset(prime_ * 28 + 28) < end);
                        *p.offset(prime_ * 28 + 28) &= 254;

                        p = p.offset(prime_ * 30 + 29)
                    }
                    if p >= end { wi = 56; break 'outer; }
                    safe_assert!(start <= p && p < end);
                    *p &= 127; p = p.offset(prime_ * 6 + 6);
                    break 'label57
                   }
                   if p >= end { wi = 57; break 'outer; }
                   safe_assert!(start <= p && p < end);
                   *p &= 191; p = p.offset(prime_ * 4 + 4);
                   break 'label58
                  }
                  if p >= end { wi = 58; break 'outer; }
                  safe_assert!(start <= p && p < end);
                  *p &= 223; p = p.offset(prime_ * 2 + 2);
                  break 'label59
                 }
                 if p >= end { wi = 59; break 'outer; }
                 safe_assert!(start <= p && p < end);
                 *p &= 239; p = p.offset(prime_ * 4 + 4);
                 break 'label60
                }
                if p >= end { wi = 60; break 'outer; }
                safe_assert!(start <= p && p < end);
                *p &= 247; p = p.offset(prime_ * 2 + 2);
                break 'label61
               }
               if p >= end { wi = 61; break 'outer; }
               safe_assert!(start <= p && p < end);
               *p &= 251; p = p.offset(prime_ * 4 + 4);
               break 'label62
              }
              if p >= end { wi = 62; break 'outer; }
              safe_assert!(start <= p && p < end);
              *p &= 253; p = p.offset(prime_ * 6 + 6);
              break 'label63
             }
             if p >= end { wi = 63; break 'outer; }
             safe_assert!(start <= p && p < end);
             *p &= 254; p = p.offset(prime_ * 2 + 1);
             wi = 56
            }
        }
        _ => unreachable!("{}", wi),
    }
    }
    *si_ = (p as usize).wrapping_sub(end as usize);
    *wi_ = wi;
}

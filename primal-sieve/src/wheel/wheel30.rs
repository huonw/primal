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
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 1, next: -7 },
    // remainder 7
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 6, correction: 1, next: -7 },
    // remainder 11
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 2, next: -7 },
    // remainder 13
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 1, next: -7 },
    // remainder 17
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 2, next: -7 },
    // remainder 19
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 2, next: -7 },
    // remainder 23
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 3, next: -7 },
    // remainder 29
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 6, correction: 6, next: -7 },
];
pub unsafe fn hardcoded_sieve(bytes: &mut [u8], si_: &mut usize, wi_: &mut usize, prime: usize) {
    let bytes = bytes;
    let start = bytes.as_mut_ptr();
    let len = bytes.len() as isize;
    let largest_step = (28 * prime + 23) as isize;
    let loop_len = len - largest_step;
    let mut si = *si_ as isize;
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
                    while si < loop_len {
                        *start.offset(si + prime_ * 0 + 0) |= 1;
                        *start.offset(si + prime_ * 6 + 0) |= 2;
                        *start.offset(si + prime_ * 10 + 0) |= 4;
                        *start.offset(si + prime_ * 12 + 0) |= 8;
                        *start.offset(si + prime_ * 16 + 0) |= 16;
                        *start.offset(si + prime_ * 18 + 0) |= 32;
                        *start.offset(si + prime_ * 22 + 0) |= 64;
                        *start.offset(si + prime_ * 28 + 0) |= 128;

                        si += prime_ * 30 + 1
                    }
                    if si >= len { wi = 0; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 6 + 0;
                    break 'label1
                   }
                   if si >= len { wi = 1; break 'outer; }
                   *start.offset(si) |= 2; si += prime_ * 4 + 0;
                   break 'label2
                  }
                  if si >= len { wi = 2; break 'outer; }
                  *start.offset(si) |= 4; si += prime_ * 2 + 0;
                  break 'label3
                 }
                 if si >= len { wi = 3; break 'outer; }
                 *start.offset(si) |= 8; si += prime_ * 4 + 0;
                 break 'label4
                }
                if si >= len { wi = 4; break 'outer; }
                *start.offset(si) |= 16; si += prime_ * 2 + 0;
                break 'label5
               }
               if si >= len { wi = 5; break 'outer; }
               *start.offset(si) |= 32; si += prime_ * 4 + 0;
               break 'label6
              }
              if si >= len { wi = 6; break 'outer; }
              *start.offset(si) |= 64; si += prime_ * 6 + 0;
              break 'label7
             }
             if si >= len { wi = 7; break 'outer; }
             *start.offset(si) |= 128; si += prime_ * 2 + 1;
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
                    while si < loop_len {
                        *start.offset(si + prime_ * 0 + 0) |= 32;
                        *start.offset(si + prime_ * 4 + 1) |= 16;
                        *start.offset(si + prime_ * 6 + 2) |= 1;
                        *start.offset(si + prime_ * 10 + 2) |= 128;
                        *start.offset(si + prime_ * 12 + 3) |= 8;
                        *start.offset(si + prime_ * 16 + 4) |= 4;
                        *start.offset(si + prime_ * 22 + 5) |= 64;
                        *start.offset(si + prime_ * 24 + 6) |= 2;

                        si += prime_ * 30 + 7
                    }
                    if si >= len { wi = 8; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 1;
                    break 'label9
                   }
                   if si >= len { wi = 9; break 'outer; }
                   *start.offset(si) |= 16; si += prime_ * 2 + 1;
                   break 'label10
                  }
                  if si >= len { wi = 10; break 'outer; }
                  *start.offset(si) |= 1; si += prime_ * 4 + 0;
                  break 'label11
                 }
                 if si >= len { wi = 11; break 'outer; }
                 *start.offset(si) |= 128; si += prime_ * 2 + 1;
                 break 'label12
                }
                if si >= len { wi = 12; break 'outer; }
                *start.offset(si) |= 8; si += prime_ * 4 + 1;
                break 'label13
               }
               if si >= len { wi = 13; break 'outer; }
               *start.offset(si) |= 4; si += prime_ * 6 + 1;
               break 'label14
              }
              if si >= len { wi = 14; break 'outer; }
              *start.offset(si) |= 64; si += prime_ * 2 + 1;
              break 'label15
             }
             if si >= len { wi = 15; break 'outer; }
             *start.offset(si) |= 2; si += prime_ * 6 + 1;
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
                    while si < loop_len {
                        *start.offset(si + prime_ * 0 + 0) |= 1;
                        *start.offset(si + prime_ * 2 + 0) |= 64;
                        *start.offset(si + prime_ * 6 + 2) |= 2;
                        *start.offset(si + prime_ * 8 + 2) |= 128;
                        *start.offset(si + prime_ * 12 + 4) |= 8;
                        *start.offset(si + prime_ * 18 + 6) |= 32;
                        *start.offset(si + prime_ * 20 + 7) |= 4;
                        *start.offset(si + prime_ * 26 + 9) |= 16;

                        si += prime_ * 30 + 11
                    }
                    if si >= len { wi = 16; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 0;
                    break 'label17
                   }
                   if si >= len { wi = 17; break 'outer; }
                   *start.offset(si) |= 64; si += prime_ * 4 + 2;
                   break 'label18
                  }
                  if si >= len { wi = 18; break 'outer; }
                  *start.offset(si) |= 2; si += prime_ * 2 + 0;
                  break 'label19
                 }
                 if si >= len { wi = 19; break 'outer; }
                 *start.offset(si) |= 128; si += prime_ * 4 + 2;
                 break 'label20
                }
                if si >= len { wi = 20; break 'outer; }
                *start.offset(si) |= 8; si += prime_ * 6 + 2;
                break 'label21
               }
               if si >= len { wi = 21; break 'outer; }
               *start.offset(si) |= 32; si += prime_ * 2 + 1;
               break 'label22
              }
              if si >= len { wi = 22; break 'outer; }
              *start.offset(si) |= 4; si += prime_ * 6 + 2;
              break 'label23
             }
             if si >= len { wi = 23; break 'outer; }
             *start.offset(si) |= 16; si += prime_ * 4 + 2;
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
                    while si < loop_len {
                        *start.offset(si + prime_ * 0 + 0) |= 32;
                        *start.offset(si + prime_ * 4 + 2) |= 4;
                        *start.offset(si + prime_ * 6 + 3) |= 2;
                        *start.offset(si + prime_ * 10 + 4) |= 128;
                        *start.offset(si + prime_ * 16 + 7) |= 16;
                        *start.offset(si + prime_ * 18 + 8) |= 8;
                        *start.offset(si + prime_ * 24 + 11) |= 1;
                        *start.offset(si + prime_ * 28 + 12) |= 64;

                        si += prime_ * 30 + 13
                    }
                    if si >= len { wi = 24; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 2;
                    break 'label25
                   }
                   if si >= len { wi = 25; break 'outer; }
                   *start.offset(si) |= 4; si += prime_ * 2 + 1;
                   break 'label26
                  }
                  if si >= len { wi = 26; break 'outer; }
                  *start.offset(si) |= 2; si += prime_ * 4 + 1;
                  break 'label27
                 }
                 if si >= len { wi = 27; break 'outer; }
                 *start.offset(si) |= 128; si += prime_ * 6 + 3;
                 break 'label28
                }
                if si >= len { wi = 28; break 'outer; }
                *start.offset(si) |= 16; si += prime_ * 2 + 1;
                break 'label29
               }
               if si >= len { wi = 29; break 'outer; }
               *start.offset(si) |= 8; si += prime_ * 6 + 3;
               break 'label30
              }
              if si >= len { wi = 30; break 'outer; }
              *start.offset(si) |= 1; si += prime_ * 4 + 1;
              break 'label31
             }
             if si >= len { wi = 31; break 'outer; }
             *start.offset(si) |= 64; si += prime_ * 2 + 1;
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
                    while si < loop_len {
                        *start.offset(si + prime_ * 0 + 0) |= 32;
                        *start.offset(si + prime_ * 2 + 1) |= 64;
                        *start.offset(si + prime_ * 6 + 4) |= 1;
                        *start.offset(si + prime_ * 12 + 7) |= 8;
                        *start.offset(si + prime_ * 14 + 8) |= 16;
                        *start.offset(si + prime_ * 20 + 11) |= 128;
                        *start.offset(si + prime_ * 24 + 14) |= 2;
                        *start.offset(si + prime_ * 26 + 15) |= 4;

                        si += prime_ * 30 + 17
                    }
                    if si >= len { wi = 32; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 1;
                    break 'label33
                   }
                   if si >= len { wi = 33; break 'outer; }
                   *start.offset(si) |= 64; si += prime_ * 4 + 3;
                   break 'label34
                  }
                  if si >= len { wi = 34; break 'outer; }
                  *start.offset(si) |= 1; si += prime_ * 6 + 3;
                  break 'label35
                 }
                 if si >= len { wi = 35; break 'outer; }
                 *start.offset(si) |= 8; si += prime_ * 2 + 1;
                 break 'label36
                }
                if si >= len { wi = 36; break 'outer; }
                *start.offset(si) |= 16; si += prime_ * 6 + 3;
                break 'label37
               }
               if si >= len { wi = 37; break 'outer; }
               *start.offset(si) |= 128; si += prime_ * 4 + 3;
               break 'label38
              }
              if si >= len { wi = 38; break 'outer; }
              *start.offset(si) |= 2; si += prime_ * 2 + 1;
              break 'label39
             }
             if si >= len { wi = 39; break 'outer; }
             *start.offset(si) |= 4; si += prime_ * 4 + 2;
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
                    while si < loop_len {
                        *start.offset(si + prime_ * 0 + 0) |= 1;
                        *start.offset(si + prime_ * 4 + 2) |= 16;
                        *start.offset(si + prime_ * 10 + 6) |= 4;
                        *start.offset(si + prime_ * 12 + 7) |= 32;
                        *start.offset(si + prime_ * 18 + 11) |= 8;
                        *start.offset(si + prime_ * 22 + 13) |= 128;
                        *start.offset(si + prime_ * 24 + 15) |= 2;
                        *start.offset(si + prime_ * 28 + 17) |= 64;

                        si += prime_ * 30 + 19
                    }
                    if si >= len { wi = 40; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 2;
                    break 'label41
                   }
                   if si >= len { wi = 41; break 'outer; }
                   *start.offset(si) |= 16; si += prime_ * 6 + 4;
                   break 'label42
                  }
                  if si >= len { wi = 42; break 'outer; }
                  *start.offset(si) |= 4; si += prime_ * 2 + 1;
                  break 'label43
                 }
                 if si >= len { wi = 43; break 'outer; }
                 *start.offset(si) |= 32; si += prime_ * 6 + 4;
                 break 'label44
                }
                if si >= len { wi = 44; break 'outer; }
                *start.offset(si) |= 8; si += prime_ * 4 + 2;
                break 'label45
               }
               if si >= len { wi = 45; break 'outer; }
               *start.offset(si) |= 128; si += prime_ * 2 + 2;
               break 'label46
              }
              if si >= len { wi = 46; break 'outer; }
              *start.offset(si) |= 2; si += prime_ * 4 + 2;
              break 'label47
             }
             if si >= len { wi = 47; break 'outer; }
             *start.offset(si) |= 64; si += prime_ * 2 + 2;
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
                    while si < loop_len {
                        *start.offset(si + prime_ * 0 + 0) |= 32;
                        *start.offset(si + prime_ * 6 + 5) |= 2;
                        *start.offset(si + prime_ * 8 + 6) |= 64;
                        *start.offset(si + prime_ * 14 + 11) |= 4;
                        *start.offset(si + prime_ * 18 + 14) |= 8;
                        *start.offset(si + prime_ * 20 + 15) |= 128;
                        *start.offset(si + prime_ * 24 + 19) |= 1;
                        *start.offset(si + prime_ * 26 + 20) |= 16;

                        si += prime_ * 30 + 23
                    }
                    if si >= len { wi = 48; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 6 + 5;
                    break 'label49
                   }
                   if si >= len { wi = 49; break 'outer; }
                   *start.offset(si) |= 2; si += prime_ * 2 + 1;
                   break 'label50
                  }
                  if si >= len { wi = 50; break 'outer; }
                  *start.offset(si) |= 64; si += prime_ * 6 + 5;
                  break 'label51
                 }
                 if si >= len { wi = 51; break 'outer; }
                 *start.offset(si) |= 4; si += prime_ * 4 + 3;
                 break 'label52
                }
                if si >= len { wi = 52; break 'outer; }
                *start.offset(si) |= 8; si += prime_ * 2 + 1;
                break 'label53
               }
               if si >= len { wi = 53; break 'outer; }
               *start.offset(si) |= 128; si += prime_ * 4 + 4;
               break 'label54
              }
              if si >= len { wi = 54; break 'outer; }
              *start.offset(si) |= 1; si += prime_ * 2 + 1;
              break 'label55
             }
             if si >= len { wi = 55; break 'outer; }
             *start.offset(si) |= 16; si += prime_ * 4 + 3;
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
                    while si < loop_len {
                        *start.offset(si + prime_ * 0 + 0) |= 1;
                        *start.offset(si + prime_ * 2 + 1) |= 128;
                        *start.offset(si + prime_ * 8 + 7) |= 64;
                        *start.offset(si + prime_ * 12 + 11) |= 32;
                        *start.offset(si + prime_ * 14 + 13) |= 16;
                        *start.offset(si + prime_ * 18 + 17) |= 8;
                        *start.offset(si + prime_ * 20 + 19) |= 4;
                        *start.offset(si + prime_ * 24 + 23) |= 2;

                        si += prime_ * 30 + 29
                    }
                    if si >= len { wi = 56; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 1;
                    break 'label57
                   }
                   if si >= len { wi = 57; break 'outer; }
                   *start.offset(si) |= 128; si += prime_ * 6 + 6;
                   break 'label58
                  }
                  if si >= len { wi = 58; break 'outer; }
                  *start.offset(si) |= 64; si += prime_ * 4 + 4;
                  break 'label59
                 }
                 if si >= len { wi = 59; break 'outer; }
                 *start.offset(si) |= 32; si += prime_ * 2 + 2;
                 break 'label60
                }
                if si >= len { wi = 60; break 'outer; }
                *start.offset(si) |= 16; si += prime_ * 4 + 4;
                break 'label61
               }
               if si >= len { wi = 61; break 'outer; }
               *start.offset(si) |= 8; si += prime_ * 2 + 2;
               break 'label62
              }
              if si >= len { wi = 62; break 'outer; }
              *start.offset(si) |= 4; si += prime_ * 4 + 4;
              break 'label63
             }
             if si >= len { wi = 63; break 'outer; }
             *start.offset(si) |= 2; si += prime_ * 6 + 6;
             wi = 56
            }
        }
        _ => unreachable!("{}", wi),
    }
    }
    *si_ = (si as usize).wrapping_sub(bytes.len());
    *wi_ = wi;
}

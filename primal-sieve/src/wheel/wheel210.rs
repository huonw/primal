// automatically generated
use wheel::{WheelInit, Wheel, WheelElem};

#[derive(Debug)]
pub struct Wheel210;
impl Wheel for Wheel210 {
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

pub const SIZE: usize = 48;

pub const MODULO: usize = 210;

const INIT: &'static [WheelInit; 210] = &[
    WheelInit { next_mult_factor: 1, wheel_index: 0 }, // 0
    WheelInit { next_mult_factor: 0, wheel_index: 0 }, // 1
    WheelInit { next_mult_factor: 9, wheel_index: 1 }, // 2
    WheelInit { next_mult_factor: 8, wheel_index: 1 }, // 3
    WheelInit { next_mult_factor: 7, wheel_index: 1 }, // 4
    WheelInit { next_mult_factor: 6, wheel_index: 1 }, // 5
    WheelInit { next_mult_factor: 5, wheel_index: 1 }, // 6
    WheelInit { next_mult_factor: 4, wheel_index: 1 }, // 7
    WheelInit { next_mult_factor: 3, wheel_index: 1 }, // 8
    WheelInit { next_mult_factor: 2, wheel_index: 1 }, // 9
    WheelInit { next_mult_factor: 1, wheel_index: 1 }, // 10
    WheelInit { next_mult_factor: 0, wheel_index: 1 }, // 11
    WheelInit { next_mult_factor: 1, wheel_index: 2 }, // 12
    WheelInit { next_mult_factor: 0, wheel_index: 2 }, // 13
    WheelInit { next_mult_factor: 3, wheel_index: 3 }, // 14
    WheelInit { next_mult_factor: 2, wheel_index: 3 }, // 15
    WheelInit { next_mult_factor: 1, wheel_index: 3 }, // 16
    WheelInit { next_mult_factor: 0, wheel_index: 3 }, // 17
    WheelInit { next_mult_factor: 1, wheel_index: 4 }, // 18
    WheelInit { next_mult_factor: 0, wheel_index: 4 }, // 19
    WheelInit { next_mult_factor: 3, wheel_index: 5 }, // 20
    WheelInit { next_mult_factor: 2, wheel_index: 5 }, // 21
    WheelInit { next_mult_factor: 1, wheel_index: 5 }, // 22
    WheelInit { next_mult_factor: 0, wheel_index: 5 }, // 23
    WheelInit { next_mult_factor: 5, wheel_index: 6 }, // 24
    WheelInit { next_mult_factor: 4, wheel_index: 6 }, // 25
    WheelInit { next_mult_factor: 3, wheel_index: 6 }, // 26
    WheelInit { next_mult_factor: 2, wheel_index: 6 }, // 27
    WheelInit { next_mult_factor: 1, wheel_index: 6 }, // 28
    WheelInit { next_mult_factor: 0, wheel_index: 6 }, // 29
    WheelInit { next_mult_factor: 1, wheel_index: 7 }, // 30
    WheelInit { next_mult_factor: 0, wheel_index: 7 }, // 31
    WheelInit { next_mult_factor: 5, wheel_index: 8 }, // 32
    WheelInit { next_mult_factor: 4, wheel_index: 8 }, // 33
    WheelInit { next_mult_factor: 3, wheel_index: 8 }, // 34
    WheelInit { next_mult_factor: 2, wheel_index: 8 }, // 35
    WheelInit { next_mult_factor: 1, wheel_index: 8 }, // 36
    WheelInit { next_mult_factor: 0, wheel_index: 8 }, // 37
    WheelInit { next_mult_factor: 3, wheel_index: 9 }, // 38
    WheelInit { next_mult_factor: 2, wheel_index: 9 }, // 39
    WheelInit { next_mult_factor: 1, wheel_index: 9 }, // 40
    WheelInit { next_mult_factor: 0, wheel_index: 9 }, // 41
    WheelInit { next_mult_factor: 1, wheel_index: 10 }, // 42
    WheelInit { next_mult_factor: 0, wheel_index: 10 }, // 43
    WheelInit { next_mult_factor: 3, wheel_index: 11 }, // 44
    WheelInit { next_mult_factor: 2, wheel_index: 11 }, // 45
    WheelInit { next_mult_factor: 1, wheel_index: 11 }, // 46
    WheelInit { next_mult_factor: 0, wheel_index: 11 }, // 47
    WheelInit { next_mult_factor: 5, wheel_index: 12 }, // 48
    WheelInit { next_mult_factor: 4, wheel_index: 12 }, // 49
    WheelInit { next_mult_factor: 3, wheel_index: 12 }, // 50
    WheelInit { next_mult_factor: 2, wheel_index: 12 }, // 51
    WheelInit { next_mult_factor: 1, wheel_index: 12 }, // 52
    WheelInit { next_mult_factor: 0, wheel_index: 12 }, // 53
    WheelInit { next_mult_factor: 5, wheel_index: 13 }, // 54
    WheelInit { next_mult_factor: 4, wheel_index: 13 }, // 55
    WheelInit { next_mult_factor: 3, wheel_index: 13 }, // 56
    WheelInit { next_mult_factor: 2, wheel_index: 13 }, // 57
    WheelInit { next_mult_factor: 1, wheel_index: 13 }, // 58
    WheelInit { next_mult_factor: 0, wheel_index: 13 }, // 59
    WheelInit { next_mult_factor: 1, wheel_index: 14 }, // 60
    WheelInit { next_mult_factor: 0, wheel_index: 14 }, // 61
    WheelInit { next_mult_factor: 5, wheel_index: 15 }, // 62
    WheelInit { next_mult_factor: 4, wheel_index: 15 }, // 63
    WheelInit { next_mult_factor: 3, wheel_index: 15 }, // 64
    WheelInit { next_mult_factor: 2, wheel_index: 15 }, // 65
    WheelInit { next_mult_factor: 1, wheel_index: 15 }, // 66
    WheelInit { next_mult_factor: 0, wheel_index: 15 }, // 67
    WheelInit { next_mult_factor: 3, wheel_index: 16 }, // 68
    WheelInit { next_mult_factor: 2, wheel_index: 16 }, // 69
    WheelInit { next_mult_factor: 1, wheel_index: 16 }, // 70
    WheelInit { next_mult_factor: 0, wheel_index: 16 }, // 71
    WheelInit { next_mult_factor: 1, wheel_index: 17 }, // 72
    WheelInit { next_mult_factor: 0, wheel_index: 17 }, // 73
    WheelInit { next_mult_factor: 5, wheel_index: 18 }, // 74
    WheelInit { next_mult_factor: 4, wheel_index: 18 }, // 75
    WheelInit { next_mult_factor: 3, wheel_index: 18 }, // 76
    WheelInit { next_mult_factor: 2, wheel_index: 18 }, // 77
    WheelInit { next_mult_factor: 1, wheel_index: 18 }, // 78
    WheelInit { next_mult_factor: 0, wheel_index: 18 }, // 79
    WheelInit { next_mult_factor: 3, wheel_index: 19 }, // 80
    WheelInit { next_mult_factor: 2, wheel_index: 19 }, // 81
    WheelInit { next_mult_factor: 1, wheel_index: 19 }, // 82
    WheelInit { next_mult_factor: 0, wheel_index: 19 }, // 83
    WheelInit { next_mult_factor: 5, wheel_index: 20 }, // 84
    WheelInit { next_mult_factor: 4, wheel_index: 20 }, // 85
    WheelInit { next_mult_factor: 3, wheel_index: 20 }, // 86
    WheelInit { next_mult_factor: 2, wheel_index: 20 }, // 87
    WheelInit { next_mult_factor: 1, wheel_index: 20 }, // 88
    WheelInit { next_mult_factor: 0, wheel_index: 20 }, // 89
    WheelInit { next_mult_factor: 7, wheel_index: 21 }, // 90
    WheelInit { next_mult_factor: 6, wheel_index: 21 }, // 91
    WheelInit { next_mult_factor: 5, wheel_index: 21 }, // 92
    WheelInit { next_mult_factor: 4, wheel_index: 21 }, // 93
    WheelInit { next_mult_factor: 3, wheel_index: 21 }, // 94
    WheelInit { next_mult_factor: 2, wheel_index: 21 }, // 95
    WheelInit { next_mult_factor: 1, wheel_index: 21 }, // 96
    WheelInit { next_mult_factor: 0, wheel_index: 21 }, // 97
    WheelInit { next_mult_factor: 3, wheel_index: 22 }, // 98
    WheelInit { next_mult_factor: 2, wheel_index: 22 }, // 99
    WheelInit { next_mult_factor: 1, wheel_index: 22 }, // 100
    WheelInit { next_mult_factor: 0, wheel_index: 22 }, // 101
    WheelInit { next_mult_factor: 1, wheel_index: 23 }, // 102
    WheelInit { next_mult_factor: 0, wheel_index: 23 }, // 103
    WheelInit { next_mult_factor: 3, wheel_index: 24 }, // 104
    WheelInit { next_mult_factor: 2, wheel_index: 24 }, // 105
    WheelInit { next_mult_factor: 1, wheel_index: 24 }, // 106
    WheelInit { next_mult_factor: 0, wheel_index: 24 }, // 107
    WheelInit { next_mult_factor: 1, wheel_index: 25 }, // 108
    WheelInit { next_mult_factor: 0, wheel_index: 25 }, // 109
    WheelInit { next_mult_factor: 3, wheel_index: 26 }, // 110
    WheelInit { next_mult_factor: 2, wheel_index: 26 }, // 111
    WheelInit { next_mult_factor: 1, wheel_index: 26 }, // 112
    WheelInit { next_mult_factor: 0, wheel_index: 26 }, // 113
    WheelInit { next_mult_factor: 7, wheel_index: 27 }, // 114
    WheelInit { next_mult_factor: 6, wheel_index: 27 }, // 115
    WheelInit { next_mult_factor: 5, wheel_index: 27 }, // 116
    WheelInit { next_mult_factor: 4, wheel_index: 27 }, // 117
    WheelInit { next_mult_factor: 3, wheel_index: 27 }, // 118
    WheelInit { next_mult_factor: 2, wheel_index: 27 }, // 119
    WheelInit { next_mult_factor: 1, wheel_index: 27 }, // 120
    WheelInit { next_mult_factor: 0, wheel_index: 27 }, // 121
    WheelInit { next_mult_factor: 5, wheel_index: 28 }, // 122
    WheelInit { next_mult_factor: 4, wheel_index: 28 }, // 123
    WheelInit { next_mult_factor: 3, wheel_index: 28 }, // 124
    WheelInit { next_mult_factor: 2, wheel_index: 28 }, // 125
    WheelInit { next_mult_factor: 1, wheel_index: 28 }, // 126
    WheelInit { next_mult_factor: 0, wheel_index: 28 }, // 127
    WheelInit { next_mult_factor: 3, wheel_index: 29 }, // 128
    WheelInit { next_mult_factor: 2, wheel_index: 29 }, // 129
    WheelInit { next_mult_factor: 1, wheel_index: 29 }, // 130
    WheelInit { next_mult_factor: 0, wheel_index: 29 }, // 131
    WheelInit { next_mult_factor: 5, wheel_index: 30 }, // 132
    WheelInit { next_mult_factor: 4, wheel_index: 30 }, // 133
    WheelInit { next_mult_factor: 3, wheel_index: 30 }, // 134
    WheelInit { next_mult_factor: 2, wheel_index: 30 }, // 135
    WheelInit { next_mult_factor: 1, wheel_index: 30 }, // 136
    WheelInit { next_mult_factor: 0, wheel_index: 30 }, // 137
    WheelInit { next_mult_factor: 1, wheel_index: 31 }, // 138
    WheelInit { next_mult_factor: 0, wheel_index: 31 }, // 139
    WheelInit { next_mult_factor: 3, wheel_index: 32 }, // 140
    WheelInit { next_mult_factor: 2, wheel_index: 32 }, // 141
    WheelInit { next_mult_factor: 1, wheel_index: 32 }, // 142
    WheelInit { next_mult_factor: 0, wheel_index: 32 }, // 143
    WheelInit { next_mult_factor: 5, wheel_index: 33 }, // 144
    WheelInit { next_mult_factor: 4, wheel_index: 33 }, // 145
    WheelInit { next_mult_factor: 3, wheel_index: 33 }, // 146
    WheelInit { next_mult_factor: 2, wheel_index: 33 }, // 147
    WheelInit { next_mult_factor: 1, wheel_index: 33 }, // 148
    WheelInit { next_mult_factor: 0, wheel_index: 33 }, // 149
    WheelInit { next_mult_factor: 1, wheel_index: 34 }, // 150
    WheelInit { next_mult_factor: 0, wheel_index: 34 }, // 151
    WheelInit { next_mult_factor: 5, wheel_index: 35 }, // 152
    WheelInit { next_mult_factor: 4, wheel_index: 35 }, // 153
    WheelInit { next_mult_factor: 3, wheel_index: 35 }, // 154
    WheelInit { next_mult_factor: 2, wheel_index: 35 }, // 155
    WheelInit { next_mult_factor: 1, wheel_index: 35 }, // 156
    WheelInit { next_mult_factor: 0, wheel_index: 35 }, // 157
    WheelInit { next_mult_factor: 5, wheel_index: 36 }, // 158
    WheelInit { next_mult_factor: 4, wheel_index: 36 }, // 159
    WheelInit { next_mult_factor: 3, wheel_index: 36 }, // 160
    WheelInit { next_mult_factor: 2, wheel_index: 36 }, // 161
    WheelInit { next_mult_factor: 1, wheel_index: 36 }, // 162
    WheelInit { next_mult_factor: 0, wheel_index: 36 }, // 163
    WheelInit { next_mult_factor: 3, wheel_index: 37 }, // 164
    WheelInit { next_mult_factor: 2, wheel_index: 37 }, // 165
    WheelInit { next_mult_factor: 1, wheel_index: 37 }, // 166
    WheelInit { next_mult_factor: 0, wheel_index: 37 }, // 167
    WheelInit { next_mult_factor: 1, wheel_index: 38 }, // 168
    WheelInit { next_mult_factor: 0, wheel_index: 38 }, // 169
    WheelInit { next_mult_factor: 3, wheel_index: 39 }, // 170
    WheelInit { next_mult_factor: 2, wheel_index: 39 }, // 171
    WheelInit { next_mult_factor: 1, wheel_index: 39 }, // 172
    WheelInit { next_mult_factor: 0, wheel_index: 39 }, // 173
    WheelInit { next_mult_factor: 5, wheel_index: 40 }, // 174
    WheelInit { next_mult_factor: 4, wheel_index: 40 }, // 175
    WheelInit { next_mult_factor: 3, wheel_index: 40 }, // 176
    WheelInit { next_mult_factor: 2, wheel_index: 40 }, // 177
    WheelInit { next_mult_factor: 1, wheel_index: 40 }, // 178
    WheelInit { next_mult_factor: 0, wheel_index: 40 }, // 179
    WheelInit { next_mult_factor: 1, wheel_index: 41 }, // 180
    WheelInit { next_mult_factor: 0, wheel_index: 41 }, // 181
    WheelInit { next_mult_factor: 5, wheel_index: 42 }, // 182
    WheelInit { next_mult_factor: 4, wheel_index: 42 }, // 183
    WheelInit { next_mult_factor: 3, wheel_index: 42 }, // 184
    WheelInit { next_mult_factor: 2, wheel_index: 42 }, // 185
    WheelInit { next_mult_factor: 1, wheel_index: 42 }, // 186
    WheelInit { next_mult_factor: 0, wheel_index: 42 }, // 187
    WheelInit { next_mult_factor: 3, wheel_index: 43 }, // 188
    WheelInit { next_mult_factor: 2, wheel_index: 43 }, // 189
    WheelInit { next_mult_factor: 1, wheel_index: 43 }, // 190
    WheelInit { next_mult_factor: 0, wheel_index: 43 }, // 191
    WheelInit { next_mult_factor: 1, wheel_index: 44 }, // 192
    WheelInit { next_mult_factor: 0, wheel_index: 44 }, // 193
    WheelInit { next_mult_factor: 3, wheel_index: 45 }, // 194
    WheelInit { next_mult_factor: 2, wheel_index: 45 }, // 195
    WheelInit { next_mult_factor: 1, wheel_index: 45 }, // 196
    WheelInit { next_mult_factor: 0, wheel_index: 45 }, // 197
    WheelInit { next_mult_factor: 1, wheel_index: 46 }, // 198
    WheelInit { next_mult_factor: 0, wheel_index: 46 }, // 199
    WheelInit { next_mult_factor: 9, wheel_index: 47 }, // 200
    WheelInit { next_mult_factor: 8, wheel_index: 47 }, // 201
    WheelInit { next_mult_factor: 7, wheel_index: 47 }, // 202
    WheelInit { next_mult_factor: 6, wheel_index: 47 }, // 203
    WheelInit { next_mult_factor: 5, wheel_index: 47 }, // 204
    WheelInit { next_mult_factor: 4, wheel_index: 47 }, // 205
    WheelInit { next_mult_factor: 3, wheel_index: 47 }, // 206
    WheelInit { next_mult_factor: 2, wheel_index: 47 }, // 207
    WheelInit { next_mult_factor: 1, wheel_index: 47 }, // 208
    WheelInit { next_mult_factor: 0, wheel_index: 47 }, // 209
];
const WHEEL: &'static [WheelElem; 384] = &[
    // remainder 1
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 6, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 1, next: -47 },
    // remainder 7
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 6, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 6, correction: 1, next: -47 },
    // remainder 11
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 0, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 6, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 2, next: -47 },
    // remainder 13
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 1, next: -47 },
    // remainder 17
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 6, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 2, next: -47 },
    // remainder 19
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 6, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 4, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 2, correction: 2, next: -47 },
    // remainder 23
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 6, correction: 5, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 3, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 3, next: -47 },
    // remainder 29
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 1u8 << 0u8, next_mult_factor: 2, correction: 1, next: 1 },
    WheelElem { unset_bit: 1u8 << 7u8, next_mult_factor: 6, correction: 6, next: 1 },
    WheelElem { unset_bit: 1u8 << 6u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 5u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 4u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 3u8, next_mult_factor: 2, correction: 2, next: 1 },
    WheelElem { unset_bit: 1u8 << 2u8, next_mult_factor: 4, correction: 4, next: 1 },
    WheelElem { unset_bit: 1u8 << 1u8, next_mult_factor: 6, correction: 6, next: -47 },
];
pub unsafe fn hardcoded_sieve(bytes: &mut [u8], si_: &mut usize, wi_: &mut usize, prime: usize) {
    let bytes = bytes;
    let start = bytes.as_mut_ptr();
    let end = start.offset(bytes.len() as isize);
    let loop_end = end.offset(-((180 * prime + 6) as isize));
    let mut wi = *wi_;
    let mut p = start.offset(*si_ as isize);
    let prime_ = prime as isize;

    'outer: loop {
    match wi {
        0...47 => { // 30 * x + 1
            if wi != 0 {
                if wi <= 1 {
                    if p >= end { wi = 1; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 2 {
                    if p >= end { wi = 2; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 3 {
                    if p >= end { wi = 3; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 4 {
                    if p >= end { wi = 4; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 5 {
                    if p >= end { wi = 5; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 6 {
                    if p >= end { wi = 6; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 6 + 0)
                }
                if wi <= 7 {
                    if p >= end { wi = 7; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 8 {
                    if p >= end { wi = 8; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 6 + 0)
                }
                if wi <= 9 {
                    if p >= end { wi = 9; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 10 {
                    if p >= end { wi = 10; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 11 {
                    if p >= end { wi = 11; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 12 {
                    if p >= end { wi = 12; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 13 {
                    if p >= end { wi = 13; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 14 {
                    if p >= end { wi = 14; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 6 + 0)
                }
                if wi <= 15 {
                    if p >= end { wi = 15; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 16 {
                    if p >= end { wi = 16; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 6 + 0)
                }
                if wi <= 17 {
                    if p >= end { wi = 17; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 18 {
                    if p >= end { wi = 18; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 19 {
                    if p >= end { wi = 19; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 20 {
                    if p >= end { wi = 20; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 21 {
                    if p >= end { wi = 21; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 22 {
                    if p >= end { wi = 22; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 6 + 0)
                }
                if wi <= 23 {
                    if p >= end { wi = 23; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 24 {
                    if p >= end { wi = 24; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 6 + 0)
                }
                if wi <= 25 {
                    if p >= end { wi = 25; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 26 {
                    if p >= end { wi = 26; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 27 {
                    if p >= end { wi = 27; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 28 {
                    if p >= end { wi = 28; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 29 {
                    if p >= end { wi = 29; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 30 {
                    if p >= end { wi = 30; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 6 + 0)
                }
                if wi <= 31 {
                    if p >= end { wi = 31; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 32 {
                    if p >= end { wi = 32; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 6 + 0)
                }
                if wi <= 33 {
                    if p >= end { wi = 33; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 34 {
                    if p >= end { wi = 34; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 35 {
                    if p >= end { wi = 35; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 36 {
                    if p >= end { wi = 36; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 37 {
                    if p >= end { wi = 37; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 38 {
                    if p >= end { wi = 38; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 6 + 0)
                }
                if wi <= 39 {
                    if p >= end { wi = 39; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 40 {
                    if p >= end { wi = 40; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 6 + 0)
                }
                if wi <= 41 {
                    if p >= end { wi = 41; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 42 {
                    if p >= end { wi = 42; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 43 {
                    if p >= end { wi = 43; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 44 {
                    if p >= end { wi = 44; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 45 {
                    if p >= end { wi = 45; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 46 {
                    if p >= end { wi = 46; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 6 + 0)
                }
                if wi <= 47 {
                    if p >= end { wi = 47; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 1)
                }
            }

            while p < loop_end {
                *p.offset(prime_ * 0 + 0) |= 1;
                *p.offset(prime_ * 6 + 0) |= 2;
                *p.offset(prime_ * 10 + 0) |= 4;
                *p.offset(prime_ * 12 + 0) |= 8;
                *p.offset(prime_ * 16 + 0) |= 16;
                *p.offset(prime_ * 18 + 0) |= 32;
                *p.offset(prime_ * 22 + 0) |= 64;
                *p.offset(prime_ * 28 + 0) |= 128;
                *p.offset(prime_ * 30 + 1) |= 1;
                *p.offset(prime_ * 36 + 1) |= 2;
                *p.offset(prime_ * 40 + 1) |= 4;
                *p.offset(prime_ * 42 + 1) |= 8;
                *p.offset(prime_ * 46 + 1) |= 16;
                *p.offset(prime_ * 48 + 1) |= 32;
                *p.offset(prime_ * 52 + 1) |= 64;
                *p.offset(prime_ * 58 + 1) |= 128;
                *p.offset(prime_ * 60 + 2) |= 1;
                *p.offset(prime_ * 66 + 2) |= 2;
                *p.offset(prime_ * 70 + 2) |= 4;
                *p.offset(prime_ * 72 + 2) |= 8;
                *p.offset(prime_ * 76 + 2) |= 16;
                *p.offset(prime_ * 78 + 2) |= 32;
                *p.offset(prime_ * 82 + 2) |= 64;
                *p.offset(prime_ * 88 + 2) |= 128;
                *p.offset(prime_ * 90 + 3) |= 1;
                *p.offset(prime_ * 96 + 3) |= 2;
                *p.offset(prime_ * 100 + 3) |= 4;
                *p.offset(prime_ * 102 + 3) |= 8;
                *p.offset(prime_ * 106 + 3) |= 16;
                *p.offset(prime_ * 108 + 3) |= 32;
                *p.offset(prime_ * 112 + 3) |= 64;
                *p.offset(prime_ * 118 + 3) |= 128;
                *p.offset(prime_ * 120 + 4) |= 1;
                *p.offset(prime_ * 126 + 4) |= 2;
                *p.offset(prime_ * 130 + 4) |= 4;
                *p.offset(prime_ * 132 + 4) |= 8;
                *p.offset(prime_ * 136 + 4) |= 16;
                *p.offset(prime_ * 138 + 4) |= 32;
                *p.offset(prime_ * 142 + 4) |= 64;
                *p.offset(prime_ * 148 + 4) |= 128;
                *p.offset(prime_ * 150 + 5) |= 1;
                *p.offset(prime_ * 156 + 5) |= 2;
                *p.offset(prime_ * 160 + 5) |= 4;
                *p.offset(prime_ * 162 + 5) |= 8;
                *p.offset(prime_ * 166 + 5) |= 16;
                *p.offset(prime_ * 168 + 5) |= 32;
                *p.offset(prime_ * 172 + 5) |= 64;
                *p.offset(prime_ * 178 + 5) |= 128;

                p = p.offset(prime_ * 180 + 6)
            }
            if wi <= 0 {
                if p >= end { wi = 0; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 6 + 0)
            }
            if wi <= 1 {
                if p >= end { wi = 1; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 2 {
                if p >= end { wi = 2; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 3 {
                if p >= end { wi = 3; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 4 {
                if p >= end { wi = 4; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 5 {
                if p >= end { wi = 5; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 6 {
                if p >= end { wi = 6; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 6 + 0)
            }
            if wi <= 7 {
                if p >= end { wi = 7; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 8 {
                if p >= end { wi = 8; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 6 + 0)
            }
            if wi <= 9 {
                if p >= end { wi = 9; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 10 {
                if p >= end { wi = 10; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 11 {
                if p >= end { wi = 11; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 12 {
                if p >= end { wi = 12; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 13 {
                if p >= end { wi = 13; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 14 {
                if p >= end { wi = 14; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 6 + 0)
            }
            if wi <= 15 {
                if p >= end { wi = 15; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 16 {
                if p >= end { wi = 16; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 6 + 0)
            }
            if wi <= 17 {
                if p >= end { wi = 17; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 18 {
                if p >= end { wi = 18; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 19 {
                if p >= end { wi = 19; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 20 {
                if p >= end { wi = 20; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 21 {
                if p >= end { wi = 21; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 22 {
                if p >= end { wi = 22; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 6 + 0)
            }
            if wi <= 23 {
                if p >= end { wi = 23; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 24 {
                if p >= end { wi = 24; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 6 + 0)
            }
            if wi <= 25 {
                if p >= end { wi = 25; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 26 {
                if p >= end { wi = 26; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 27 {
                if p >= end { wi = 27; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 28 {
                if p >= end { wi = 28; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 29 {
                if p >= end { wi = 29; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 30 {
                if p >= end { wi = 30; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 6 + 0)
            }
            if wi <= 31 {
                if p >= end { wi = 31; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 32 {
                if p >= end { wi = 32; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 6 + 0)
            }
            if wi <= 33 {
                if p >= end { wi = 33; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 34 {
                if p >= end { wi = 34; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 35 {
                if p >= end { wi = 35; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 36 {
                if p >= end { wi = 36; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 37 {
                if p >= end { wi = 37; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 38 {
                if p >= end { wi = 38; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 6 + 0)
            }
            if wi <= 39 {
                if p >= end { wi = 39; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 40 {
                if p >= end { wi = 40; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 6 + 0)
            }
            if wi <= 41 {
                if p >= end { wi = 41; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 42 {
                if p >= end { wi = 42; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 43 {
                if p >= end { wi = 43; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 44 {
                if p >= end { wi = 44; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 45 {
                if p >= end { wi = 45; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 46 {
                if p >= end { wi = 46; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 6 + 0)
            }
            if wi <= 47 {
                if p >= end { wi = 47; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 1)
            }
        }
        48...95 => { // 30 * x + 7
            if wi != 48 {
                if wi <= 49 {
                    if p >= end { wi = 49; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 50 {
                    if p >= end { wi = 50; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 51 {
                    if p >= end { wi = 51; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 52 {
                    if p >= end { wi = 52; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 53 {
                    if p >= end { wi = 53; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 6 + 1)
                }
                if wi <= 54 {
                    if p >= end { wi = 54; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 55 {
                    if p >= end { wi = 55; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 6 + 1)
                }
                if wi <= 56 {
                    if p >= end { wi = 56; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 57 {
                    if p >= end { wi = 57; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 58 {
                    if p >= end { wi = 58; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 59 {
                    if p >= end { wi = 59; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 60 {
                    if p >= end { wi = 60; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 61 {
                    if p >= end { wi = 61; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 6 + 1)
                }
                if wi <= 62 {
                    if p >= end { wi = 62; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 63 {
                    if p >= end { wi = 63; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 6 + 1)
                }
                if wi <= 64 {
                    if p >= end { wi = 64; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 65 {
                    if p >= end { wi = 65; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 66 {
                    if p >= end { wi = 66; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 67 {
                    if p >= end { wi = 67; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 68 {
                    if p >= end { wi = 68; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 69 {
                    if p >= end { wi = 69; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 6 + 1)
                }
                if wi <= 70 {
                    if p >= end { wi = 70; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 71 {
                    if p >= end { wi = 71; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 6 + 1)
                }
                if wi <= 72 {
                    if p >= end { wi = 72; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 73 {
                    if p >= end { wi = 73; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 74 {
                    if p >= end { wi = 74; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 75 {
                    if p >= end { wi = 75; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 76 {
                    if p >= end { wi = 76; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 77 {
                    if p >= end { wi = 77; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 6 + 1)
                }
                if wi <= 78 {
                    if p >= end { wi = 78; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 79 {
                    if p >= end { wi = 79; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 6 + 1)
                }
                if wi <= 80 {
                    if p >= end { wi = 80; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 81 {
                    if p >= end { wi = 81; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 82 {
                    if p >= end { wi = 82; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 83 {
                    if p >= end { wi = 83; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 84 {
                    if p >= end { wi = 84; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 85 {
                    if p >= end { wi = 85; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 6 + 1)
                }
                if wi <= 86 {
                    if p >= end { wi = 86; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 87 {
                    if p >= end { wi = 87; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 6 + 1)
                }
                if wi <= 88 {
                    if p >= end { wi = 88; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 89 {
                    if p >= end { wi = 89; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 90 {
                    if p >= end { wi = 90; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 4 + 0)
                }
                if wi <= 91 {
                    if p >= end { wi = 91; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 92 {
                    if p >= end { wi = 92; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 93 {
                    if p >= end { wi = 93; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 6 + 1)
                }
                if wi <= 94 {
                    if p >= end { wi = 94; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 95 {
                    if p >= end { wi = 95; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 6 + 1)
                }
            }

            while p < loop_end {
                *p.offset(prime_ * 0 + 0) |= 32;
                *p.offset(prime_ * 4 + 1) |= 16;
                *p.offset(prime_ * 6 + 2) |= 1;
                *p.offset(prime_ * 10 + 2) |= 128;
                *p.offset(prime_ * 12 + 3) |= 8;
                *p.offset(prime_ * 16 + 4) |= 4;
                *p.offset(prime_ * 22 + 5) |= 64;
                *p.offset(prime_ * 24 + 6) |= 2;
                *p.offset(prime_ * 30 + 7) |= 32;
                *p.offset(prime_ * 34 + 8) |= 16;
                *p.offset(prime_ * 36 + 9) |= 1;
                *p.offset(prime_ * 40 + 9) |= 128;
                *p.offset(prime_ * 42 + 10) |= 8;
                *p.offset(prime_ * 46 + 11) |= 4;
                *p.offset(prime_ * 52 + 12) |= 64;
                *p.offset(prime_ * 54 + 13) |= 2;
                *p.offset(prime_ * 60 + 14) |= 32;
                *p.offset(prime_ * 64 + 15) |= 16;
                *p.offset(prime_ * 66 + 16) |= 1;
                *p.offset(prime_ * 70 + 16) |= 128;
                *p.offset(prime_ * 72 + 17) |= 8;
                *p.offset(prime_ * 76 + 18) |= 4;
                *p.offset(prime_ * 82 + 19) |= 64;
                *p.offset(prime_ * 84 + 20) |= 2;
                *p.offset(prime_ * 90 + 21) |= 32;
                *p.offset(prime_ * 94 + 22) |= 16;
                *p.offset(prime_ * 96 + 23) |= 1;
                *p.offset(prime_ * 100 + 23) |= 128;
                *p.offset(prime_ * 102 + 24) |= 8;
                *p.offset(prime_ * 106 + 25) |= 4;
                *p.offset(prime_ * 112 + 26) |= 64;
                *p.offset(prime_ * 114 + 27) |= 2;
                *p.offset(prime_ * 120 + 28) |= 32;
                *p.offset(prime_ * 124 + 29) |= 16;
                *p.offset(prime_ * 126 + 30) |= 1;
                *p.offset(prime_ * 130 + 30) |= 128;
                *p.offset(prime_ * 132 + 31) |= 8;
                *p.offset(prime_ * 136 + 32) |= 4;
                *p.offset(prime_ * 142 + 33) |= 64;
                *p.offset(prime_ * 144 + 34) |= 2;
                *p.offset(prime_ * 150 + 35) |= 32;
                *p.offset(prime_ * 154 + 36) |= 16;
                *p.offset(prime_ * 156 + 37) |= 1;
                *p.offset(prime_ * 160 + 37) |= 128;
                *p.offset(prime_ * 162 + 38) |= 8;
                *p.offset(prime_ * 166 + 39) |= 4;
                *p.offset(prime_ * 172 + 40) |= 64;
                *p.offset(prime_ * 174 + 41) |= 2;

                p = p.offset(prime_ * 180 + 42)
            }
            if wi <= 48 {
                if p >= end { wi = 48; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 49 {
                if p >= end { wi = 49; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 50 {
                if p >= end { wi = 50; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 51 {
                if p >= end { wi = 51; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 52 {
                if p >= end { wi = 52; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 53 {
                if p >= end { wi = 53; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 6 + 1)
            }
            if wi <= 54 {
                if p >= end { wi = 54; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 55 {
                if p >= end { wi = 55; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 6 + 1)
            }
            if wi <= 56 {
                if p >= end { wi = 56; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 57 {
                if p >= end { wi = 57; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 58 {
                if p >= end { wi = 58; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 59 {
                if p >= end { wi = 59; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 60 {
                if p >= end { wi = 60; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 61 {
                if p >= end { wi = 61; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 6 + 1)
            }
            if wi <= 62 {
                if p >= end { wi = 62; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 63 {
                if p >= end { wi = 63; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 6 + 1)
            }
            if wi <= 64 {
                if p >= end { wi = 64; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 65 {
                if p >= end { wi = 65; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 66 {
                if p >= end { wi = 66; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 67 {
                if p >= end { wi = 67; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 68 {
                if p >= end { wi = 68; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 69 {
                if p >= end { wi = 69; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 6 + 1)
            }
            if wi <= 70 {
                if p >= end { wi = 70; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 71 {
                if p >= end { wi = 71; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 6 + 1)
            }
            if wi <= 72 {
                if p >= end { wi = 72; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 73 {
                if p >= end { wi = 73; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 74 {
                if p >= end { wi = 74; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 75 {
                if p >= end { wi = 75; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 76 {
                if p >= end { wi = 76; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 77 {
                if p >= end { wi = 77; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 6 + 1)
            }
            if wi <= 78 {
                if p >= end { wi = 78; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 79 {
                if p >= end { wi = 79; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 6 + 1)
            }
            if wi <= 80 {
                if p >= end { wi = 80; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 81 {
                if p >= end { wi = 81; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 82 {
                if p >= end { wi = 82; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 83 {
                if p >= end { wi = 83; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 84 {
                if p >= end { wi = 84; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 85 {
                if p >= end { wi = 85; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 6 + 1)
            }
            if wi <= 86 {
                if p >= end { wi = 86; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 87 {
                if p >= end { wi = 87; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 6 + 1)
            }
            if wi <= 88 {
                if p >= end { wi = 88; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 89 {
                if p >= end { wi = 89; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 90 {
                if p >= end { wi = 90; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 0)
            }
            if wi <= 91 {
                if p >= end { wi = 91; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 92 {
                if p >= end { wi = 92; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 93 {
                if p >= end { wi = 93; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 6 + 1)
            }
            if wi <= 94 {
                if p >= end { wi = 94; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 95 {
                if p >= end { wi = 95; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 6 + 1)
            }
        }
        96...143 => { // 30 * x + 11
            if wi != 96 {
                if wi <= 97 {
                    if p >= end { wi = 97; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 98 {
                    if p >= end { wi = 98; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 99 {
                    if p >= end { wi = 99; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 100 {
                    if p >= end { wi = 100; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 6 + 2)
                }
                if wi <= 101 {
                    if p >= end { wi = 101; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 102 {
                    if p >= end { wi = 102; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 6 + 2)
                }
                if wi <= 103 {
                    if p >= end { wi = 103; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 104 {
                    if p >= end { wi = 104; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 105 {
                    if p >= end { wi = 105; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 106 {
                    if p >= end { wi = 106; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 107 {
                    if p >= end { wi = 107; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 108 {
                    if p >= end { wi = 108; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 6 + 2)
                }
                if wi <= 109 {
                    if p >= end { wi = 109; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 110 {
                    if p >= end { wi = 110; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 6 + 2)
                }
                if wi <= 111 {
                    if p >= end { wi = 111; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 112 {
                    if p >= end { wi = 112; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 113 {
                    if p >= end { wi = 113; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 114 {
                    if p >= end { wi = 114; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 115 {
                    if p >= end { wi = 115; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 116 {
                    if p >= end { wi = 116; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 6 + 2)
                }
                if wi <= 117 {
                    if p >= end { wi = 117; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 118 {
                    if p >= end { wi = 118; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 6 + 2)
                }
                if wi <= 119 {
                    if p >= end { wi = 119; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 120 {
                    if p >= end { wi = 120; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 121 {
                    if p >= end { wi = 121; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 122 {
                    if p >= end { wi = 122; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 123 {
                    if p >= end { wi = 123; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 124 {
                    if p >= end { wi = 124; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 6 + 2)
                }
                if wi <= 125 {
                    if p >= end { wi = 125; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 126 {
                    if p >= end { wi = 126; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 6 + 2)
                }
                if wi <= 127 {
                    if p >= end { wi = 127; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 128 {
                    if p >= end { wi = 128; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 129 {
                    if p >= end { wi = 129; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 130 {
                    if p >= end { wi = 130; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 131 {
                    if p >= end { wi = 131; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 132 {
                    if p >= end { wi = 132; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 6 + 2)
                }
                if wi <= 133 {
                    if p >= end { wi = 133; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 134 {
                    if p >= end { wi = 134; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 6 + 2)
                }
                if wi <= 135 {
                    if p >= end { wi = 135; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 136 {
                    if p >= end { wi = 136; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 137 {
                    if p >= end { wi = 137; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 138 {
                    if p >= end { wi = 138; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 0)
                }
                if wi <= 139 {
                    if p >= end { wi = 139; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 140 {
                    if p >= end { wi = 140; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 6 + 2)
                }
                if wi <= 141 {
                    if p >= end { wi = 141; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 142 {
                    if p >= end { wi = 142; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 6 + 2)
                }
                if wi <= 143 {
                    if p >= end { wi = 143; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 2)
                }
            }

            while p < loop_end {
                *p.offset(prime_ * 0 + 0) |= 1;
                *p.offset(prime_ * 2 + 0) |= 64;
                *p.offset(prime_ * 6 + 2) |= 2;
                *p.offset(prime_ * 8 + 2) |= 128;
                *p.offset(prime_ * 12 + 4) |= 8;
                *p.offset(prime_ * 18 + 6) |= 32;
                *p.offset(prime_ * 20 + 7) |= 4;
                *p.offset(prime_ * 26 + 9) |= 16;
                *p.offset(prime_ * 30 + 11) |= 1;
                *p.offset(prime_ * 32 + 11) |= 64;
                *p.offset(prime_ * 36 + 13) |= 2;
                *p.offset(prime_ * 38 + 13) |= 128;
                *p.offset(prime_ * 42 + 15) |= 8;
                *p.offset(prime_ * 48 + 17) |= 32;
                *p.offset(prime_ * 50 + 18) |= 4;
                *p.offset(prime_ * 56 + 20) |= 16;
                *p.offset(prime_ * 60 + 22) |= 1;
                *p.offset(prime_ * 62 + 22) |= 64;
                *p.offset(prime_ * 66 + 24) |= 2;
                *p.offset(prime_ * 68 + 24) |= 128;
                *p.offset(prime_ * 72 + 26) |= 8;
                *p.offset(prime_ * 78 + 28) |= 32;
                *p.offset(prime_ * 80 + 29) |= 4;
                *p.offset(prime_ * 86 + 31) |= 16;
                *p.offset(prime_ * 90 + 33) |= 1;
                *p.offset(prime_ * 92 + 33) |= 64;
                *p.offset(prime_ * 96 + 35) |= 2;
                *p.offset(prime_ * 98 + 35) |= 128;
                *p.offset(prime_ * 102 + 37) |= 8;
                *p.offset(prime_ * 108 + 39) |= 32;
                *p.offset(prime_ * 110 + 40) |= 4;
                *p.offset(prime_ * 116 + 42) |= 16;
                *p.offset(prime_ * 120 + 44) |= 1;
                *p.offset(prime_ * 122 + 44) |= 64;
                *p.offset(prime_ * 126 + 46) |= 2;
                *p.offset(prime_ * 128 + 46) |= 128;
                *p.offset(prime_ * 132 + 48) |= 8;
                *p.offset(prime_ * 138 + 50) |= 32;
                *p.offset(prime_ * 140 + 51) |= 4;
                *p.offset(prime_ * 146 + 53) |= 16;
                *p.offset(prime_ * 150 + 55) |= 1;
                *p.offset(prime_ * 152 + 55) |= 64;
                *p.offset(prime_ * 156 + 57) |= 2;
                *p.offset(prime_ * 158 + 57) |= 128;
                *p.offset(prime_ * 162 + 59) |= 8;
                *p.offset(prime_ * 168 + 61) |= 32;
                *p.offset(prime_ * 170 + 62) |= 4;
                *p.offset(prime_ * 176 + 64) |= 16;

                p = p.offset(prime_ * 180 + 66)
            }
            if wi <= 96 {
                if p >= end { wi = 96; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 97 {
                if p >= end { wi = 97; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 98 {
                if p >= end { wi = 98; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 99 {
                if p >= end { wi = 99; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 100 {
                if p >= end { wi = 100; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 6 + 2)
            }
            if wi <= 101 {
                if p >= end { wi = 101; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 102 {
                if p >= end { wi = 102; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 6 + 2)
            }
            if wi <= 103 {
                if p >= end { wi = 103; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 104 {
                if p >= end { wi = 104; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 105 {
                if p >= end { wi = 105; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 106 {
                if p >= end { wi = 106; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 107 {
                if p >= end { wi = 107; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 108 {
                if p >= end { wi = 108; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 6 + 2)
            }
            if wi <= 109 {
                if p >= end { wi = 109; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 110 {
                if p >= end { wi = 110; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 6 + 2)
            }
            if wi <= 111 {
                if p >= end { wi = 111; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 112 {
                if p >= end { wi = 112; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 113 {
                if p >= end { wi = 113; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 114 {
                if p >= end { wi = 114; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 115 {
                if p >= end { wi = 115; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 116 {
                if p >= end { wi = 116; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 6 + 2)
            }
            if wi <= 117 {
                if p >= end { wi = 117; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 118 {
                if p >= end { wi = 118; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 6 + 2)
            }
            if wi <= 119 {
                if p >= end { wi = 119; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 120 {
                if p >= end { wi = 120; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 121 {
                if p >= end { wi = 121; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 122 {
                if p >= end { wi = 122; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 123 {
                if p >= end { wi = 123; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 124 {
                if p >= end { wi = 124; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 6 + 2)
            }
            if wi <= 125 {
                if p >= end { wi = 125; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 126 {
                if p >= end { wi = 126; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 6 + 2)
            }
            if wi <= 127 {
                if p >= end { wi = 127; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 128 {
                if p >= end { wi = 128; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 129 {
                if p >= end { wi = 129; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 130 {
                if p >= end { wi = 130; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 131 {
                if p >= end { wi = 131; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 132 {
                if p >= end { wi = 132; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 6 + 2)
            }
            if wi <= 133 {
                if p >= end { wi = 133; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 134 {
                if p >= end { wi = 134; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 6 + 2)
            }
            if wi <= 135 {
                if p >= end { wi = 135; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 136 {
                if p >= end { wi = 136; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 137 {
                if p >= end { wi = 137; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 138 {
                if p >= end { wi = 138; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 0)
            }
            if wi <= 139 {
                if p >= end { wi = 139; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 140 {
                if p >= end { wi = 140; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 6 + 2)
            }
            if wi <= 141 {
                if p >= end { wi = 141; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 142 {
                if p >= end { wi = 142; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 6 + 2)
            }
            if wi <= 143 {
                if p >= end { wi = 143; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 2)
            }
        }
        144...191 => { // 30 * x + 13
            if wi != 144 {
                if wi <= 145 {
                    if p >= end { wi = 145; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 146 {
                    if p >= end { wi = 146; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 147 {
                    if p >= end { wi = 147; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 148 {
                    if p >= end { wi = 148; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 149 {
                    if p >= end { wi = 149; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 150 {
                    if p >= end { wi = 150; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 151 {
                    if p >= end { wi = 151; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 152 {
                    if p >= end { wi = 152; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 153 {
                    if p >= end { wi = 153; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 154 {
                    if p >= end { wi = 154; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 155 {
                    if p >= end { wi = 155; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 156 {
                    if p >= end { wi = 156; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 157 {
                    if p >= end { wi = 157; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 158 {
                    if p >= end { wi = 158; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 159 {
                    if p >= end { wi = 159; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 160 {
                    if p >= end { wi = 160; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 161 {
                    if p >= end { wi = 161; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 162 {
                    if p >= end { wi = 162; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 163 {
                    if p >= end { wi = 163; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 164 {
                    if p >= end { wi = 164; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 165 {
                    if p >= end { wi = 165; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 166 {
                    if p >= end { wi = 166; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 167 {
                    if p >= end { wi = 167; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 168 {
                    if p >= end { wi = 168; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 169 {
                    if p >= end { wi = 169; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 170 {
                    if p >= end { wi = 170; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 171 {
                    if p >= end { wi = 171; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 172 {
                    if p >= end { wi = 172; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 173 {
                    if p >= end { wi = 173; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 174 {
                    if p >= end { wi = 174; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 175 {
                    if p >= end { wi = 175; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 176 {
                    if p >= end { wi = 176; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 177 {
                    if p >= end { wi = 177; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 178 {
                    if p >= end { wi = 178; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 179 {
                    if p >= end { wi = 179; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 180 {
                    if p >= end { wi = 180; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 181 {
                    if p >= end { wi = 181; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 182 {
                    if p >= end { wi = 182; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 183 {
                    if p >= end { wi = 183; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 184 {
                    if p >= end { wi = 184; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 185 {
                    if p >= end { wi = 185; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 186 {
                    if p >= end { wi = 186; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 187 {
                    if p >= end { wi = 187; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 188 {
                    if p >= end { wi = 188; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 189 {
                    if p >= end { wi = 189; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 190 {
                    if p >= end { wi = 190; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 4 + 1)
                }
                if wi <= 191 {
                    if p >= end { wi = 191; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 1)
                }
            }

            while p < loop_end {
                *p.offset(prime_ * 0 + 0) |= 32;
                *p.offset(prime_ * 4 + 2) |= 4;
                *p.offset(prime_ * 6 + 3) |= 2;
                *p.offset(prime_ * 10 + 4) |= 128;
                *p.offset(prime_ * 16 + 7) |= 16;
                *p.offset(prime_ * 18 + 8) |= 8;
                *p.offset(prime_ * 24 + 11) |= 1;
                *p.offset(prime_ * 28 + 12) |= 64;
                *p.offset(prime_ * 30 + 13) |= 32;
                *p.offset(prime_ * 34 + 15) |= 4;
                *p.offset(prime_ * 36 + 16) |= 2;
                *p.offset(prime_ * 40 + 17) |= 128;
                *p.offset(prime_ * 46 + 20) |= 16;
                *p.offset(prime_ * 48 + 21) |= 8;
                *p.offset(prime_ * 54 + 24) |= 1;
                *p.offset(prime_ * 58 + 25) |= 64;
                *p.offset(prime_ * 60 + 26) |= 32;
                *p.offset(prime_ * 64 + 28) |= 4;
                *p.offset(prime_ * 66 + 29) |= 2;
                *p.offset(prime_ * 70 + 30) |= 128;
                *p.offset(prime_ * 76 + 33) |= 16;
                *p.offset(prime_ * 78 + 34) |= 8;
                *p.offset(prime_ * 84 + 37) |= 1;
                *p.offset(prime_ * 88 + 38) |= 64;
                *p.offset(prime_ * 90 + 39) |= 32;
                *p.offset(prime_ * 94 + 41) |= 4;
                *p.offset(prime_ * 96 + 42) |= 2;
                *p.offset(prime_ * 100 + 43) |= 128;
                *p.offset(prime_ * 106 + 46) |= 16;
                *p.offset(prime_ * 108 + 47) |= 8;
                *p.offset(prime_ * 114 + 50) |= 1;
                *p.offset(prime_ * 118 + 51) |= 64;
                *p.offset(prime_ * 120 + 52) |= 32;
                *p.offset(prime_ * 124 + 54) |= 4;
                *p.offset(prime_ * 126 + 55) |= 2;
                *p.offset(prime_ * 130 + 56) |= 128;
                *p.offset(prime_ * 136 + 59) |= 16;
                *p.offset(prime_ * 138 + 60) |= 8;
                *p.offset(prime_ * 144 + 63) |= 1;
                *p.offset(prime_ * 148 + 64) |= 64;
                *p.offset(prime_ * 150 + 65) |= 32;
                *p.offset(prime_ * 154 + 67) |= 4;
                *p.offset(prime_ * 156 + 68) |= 2;
                *p.offset(prime_ * 160 + 69) |= 128;
                *p.offset(prime_ * 166 + 72) |= 16;
                *p.offset(prime_ * 168 + 73) |= 8;
                *p.offset(prime_ * 174 + 76) |= 1;
                *p.offset(prime_ * 178 + 77) |= 64;

                p = p.offset(prime_ * 180 + 78)
            }
            if wi <= 144 {
                if p >= end { wi = 144; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 145 {
                if p >= end { wi = 145; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 146 {
                if p >= end { wi = 146; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 147 {
                if p >= end { wi = 147; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 148 {
                if p >= end { wi = 148; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 149 {
                if p >= end { wi = 149; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 150 {
                if p >= end { wi = 150; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 151 {
                if p >= end { wi = 151; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 152 {
                if p >= end { wi = 152; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 153 {
                if p >= end { wi = 153; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 154 {
                if p >= end { wi = 154; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 155 {
                if p >= end { wi = 155; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 156 {
                if p >= end { wi = 156; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 157 {
                if p >= end { wi = 157; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 158 {
                if p >= end { wi = 158; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 159 {
                if p >= end { wi = 159; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 160 {
                if p >= end { wi = 160; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 161 {
                if p >= end { wi = 161; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 162 {
                if p >= end { wi = 162; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 163 {
                if p >= end { wi = 163; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 164 {
                if p >= end { wi = 164; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 165 {
                if p >= end { wi = 165; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 166 {
                if p >= end { wi = 166; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 167 {
                if p >= end { wi = 167; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 168 {
                if p >= end { wi = 168; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 169 {
                if p >= end { wi = 169; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 170 {
                if p >= end { wi = 170; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 171 {
                if p >= end { wi = 171; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 172 {
                if p >= end { wi = 172; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 173 {
                if p >= end { wi = 173; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 174 {
                if p >= end { wi = 174; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 175 {
                if p >= end { wi = 175; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 176 {
                if p >= end { wi = 176; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 177 {
                if p >= end { wi = 177; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 178 {
                if p >= end { wi = 178; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 179 {
                if p >= end { wi = 179; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 180 {
                if p >= end { wi = 180; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 181 {
                if p >= end { wi = 181; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 182 {
                if p >= end { wi = 182; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 183 {
                if p >= end { wi = 183; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 184 {
                if p >= end { wi = 184; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 185 {
                if p >= end { wi = 185; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 186 {
                if p >= end { wi = 186; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 187 {
                if p >= end { wi = 187; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 188 {
                if p >= end { wi = 188; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 189 {
                if p >= end { wi = 189; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 190 {
                if p >= end { wi = 190; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 1)
            }
            if wi <= 191 {
                if p >= end { wi = 191; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 1)
            }
        }
        192...239 => { // 30 * x + 17
            if wi != 192 {
                if wi <= 193 {
                    if p >= end { wi = 193; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 194 {
                    if p >= end { wi = 194; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 195 {
                    if p >= end { wi = 195; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 196 {
                    if p >= end { wi = 196; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 197 {
                    if p >= end { wi = 197; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 198 {
                    if p >= end { wi = 198; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 199 {
                    if p >= end { wi = 199; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 200 {
                    if p >= end { wi = 200; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 201 {
                    if p >= end { wi = 201; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 202 {
                    if p >= end { wi = 202; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 203 {
                    if p >= end { wi = 203; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 204 {
                    if p >= end { wi = 204; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 205 {
                    if p >= end { wi = 205; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 206 {
                    if p >= end { wi = 206; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 207 {
                    if p >= end { wi = 207; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 208 {
                    if p >= end { wi = 208; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 209 {
                    if p >= end { wi = 209; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 210 {
                    if p >= end { wi = 210; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 211 {
                    if p >= end { wi = 211; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 212 {
                    if p >= end { wi = 212; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 213 {
                    if p >= end { wi = 213; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 214 {
                    if p >= end { wi = 214; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 215 {
                    if p >= end { wi = 215; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 216 {
                    if p >= end { wi = 216; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 217 {
                    if p >= end { wi = 217; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 218 {
                    if p >= end { wi = 218; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 219 {
                    if p >= end { wi = 219; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 220 {
                    if p >= end { wi = 220; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 221 {
                    if p >= end { wi = 221; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 222 {
                    if p >= end { wi = 222; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 223 {
                    if p >= end { wi = 223; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 224 {
                    if p >= end { wi = 224; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 225 {
                    if p >= end { wi = 225; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 226 {
                    if p >= end { wi = 226; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 227 {
                    if p >= end { wi = 227; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 228 {
                    if p >= end { wi = 228; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 229 {
                    if p >= end { wi = 229; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 230 {
                    if p >= end { wi = 230; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 231 {
                    if p >= end { wi = 231; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 232 {
                    if p >= end { wi = 232; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 233 {
                    if p >= end { wi = 233; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 234 {
                    if p >= end { wi = 234; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 235 {
                    if p >= end { wi = 235; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 236 {
                    if p >= end { wi = 236; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 6 + 3)
                }
                if wi <= 237 {
                    if p >= end { wi = 237; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 238 {
                    if p >= end { wi = 238; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 239 {
                    if p >= end { wi = 239; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 2)
                }
            }

            while p < loop_end {
                *p.offset(prime_ * 0 + 0) |= 32;
                *p.offset(prime_ * 2 + 1) |= 64;
                *p.offset(prime_ * 6 + 4) |= 1;
                *p.offset(prime_ * 12 + 7) |= 8;
                *p.offset(prime_ * 14 + 8) |= 16;
                *p.offset(prime_ * 20 + 11) |= 128;
                *p.offset(prime_ * 24 + 14) |= 2;
                *p.offset(prime_ * 26 + 15) |= 4;
                *p.offset(prime_ * 30 + 17) |= 32;
                *p.offset(prime_ * 32 + 18) |= 64;
                *p.offset(prime_ * 36 + 21) |= 1;
                *p.offset(prime_ * 42 + 24) |= 8;
                *p.offset(prime_ * 44 + 25) |= 16;
                *p.offset(prime_ * 50 + 28) |= 128;
                *p.offset(prime_ * 54 + 31) |= 2;
                *p.offset(prime_ * 56 + 32) |= 4;
                *p.offset(prime_ * 60 + 34) |= 32;
                *p.offset(prime_ * 62 + 35) |= 64;
                *p.offset(prime_ * 66 + 38) |= 1;
                *p.offset(prime_ * 72 + 41) |= 8;
                *p.offset(prime_ * 74 + 42) |= 16;
                *p.offset(prime_ * 80 + 45) |= 128;
                *p.offset(prime_ * 84 + 48) |= 2;
                *p.offset(prime_ * 86 + 49) |= 4;
                *p.offset(prime_ * 90 + 51) |= 32;
                *p.offset(prime_ * 92 + 52) |= 64;
                *p.offset(prime_ * 96 + 55) |= 1;
                *p.offset(prime_ * 102 + 58) |= 8;
                *p.offset(prime_ * 104 + 59) |= 16;
                *p.offset(prime_ * 110 + 62) |= 128;
                *p.offset(prime_ * 114 + 65) |= 2;
                *p.offset(prime_ * 116 + 66) |= 4;
                *p.offset(prime_ * 120 + 68) |= 32;
                *p.offset(prime_ * 122 + 69) |= 64;
                *p.offset(prime_ * 126 + 72) |= 1;
                *p.offset(prime_ * 132 + 75) |= 8;
                *p.offset(prime_ * 134 + 76) |= 16;
                *p.offset(prime_ * 140 + 79) |= 128;
                *p.offset(prime_ * 144 + 82) |= 2;
                *p.offset(prime_ * 146 + 83) |= 4;
                *p.offset(prime_ * 150 + 85) |= 32;
                *p.offset(prime_ * 152 + 86) |= 64;
                *p.offset(prime_ * 156 + 89) |= 1;
                *p.offset(prime_ * 162 + 92) |= 8;
                *p.offset(prime_ * 164 + 93) |= 16;
                *p.offset(prime_ * 170 + 96) |= 128;
                *p.offset(prime_ * 174 + 99) |= 2;
                *p.offset(prime_ * 176 + 100) |= 4;

                p = p.offset(prime_ * 180 + 102)
            }
            if wi <= 192 {
                if p >= end { wi = 192; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 193 {
                if p >= end { wi = 193; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 194 {
                if p >= end { wi = 194; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 195 {
                if p >= end { wi = 195; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 196 {
                if p >= end { wi = 196; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 197 {
                if p >= end { wi = 197; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 198 {
                if p >= end { wi = 198; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 199 {
                if p >= end { wi = 199; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 200 {
                if p >= end { wi = 200; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 201 {
                if p >= end { wi = 201; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 202 {
                if p >= end { wi = 202; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 203 {
                if p >= end { wi = 203; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 204 {
                if p >= end { wi = 204; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 205 {
                if p >= end { wi = 205; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 206 {
                if p >= end { wi = 206; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 207 {
                if p >= end { wi = 207; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 208 {
                if p >= end { wi = 208; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 209 {
                if p >= end { wi = 209; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 210 {
                if p >= end { wi = 210; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 211 {
                if p >= end { wi = 211; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 212 {
                if p >= end { wi = 212; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 213 {
                if p >= end { wi = 213; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 214 {
                if p >= end { wi = 214; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 215 {
                if p >= end { wi = 215; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 216 {
                if p >= end { wi = 216; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 217 {
                if p >= end { wi = 217; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 218 {
                if p >= end { wi = 218; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 219 {
                if p >= end { wi = 219; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 220 {
                if p >= end { wi = 220; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 221 {
                if p >= end { wi = 221; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 222 {
                if p >= end { wi = 222; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 223 {
                if p >= end { wi = 223; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 224 {
                if p >= end { wi = 224; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 225 {
                if p >= end { wi = 225; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 226 {
                if p >= end { wi = 226; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 227 {
                if p >= end { wi = 227; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 228 {
                if p >= end { wi = 228; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 229 {
                if p >= end { wi = 229; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 230 {
                if p >= end { wi = 230; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 231 {
                if p >= end { wi = 231; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 232 {
                if p >= end { wi = 232; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 233 {
                if p >= end { wi = 233; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 234 {
                if p >= end { wi = 234; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 235 {
                if p >= end { wi = 235; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 236 {
                if p >= end { wi = 236; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 6 + 3)
            }
            if wi <= 237 {
                if p >= end { wi = 237; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 238 {
                if p >= end { wi = 238; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 239 {
                if p >= end { wi = 239; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 2)
            }
        }
        240...287 => { // 30 * x + 19
            if wi != 240 {
                if wi <= 241 {
                    if p >= end { wi = 241; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 6 + 4)
                }
                if wi <= 242 {
                    if p >= end { wi = 242; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 243 {
                    if p >= end { wi = 243; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 6 + 4)
                }
                if wi <= 244 {
                    if p >= end { wi = 244; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 245 {
                    if p >= end { wi = 245; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 246 {
                    if p >= end { wi = 246; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 247 {
                    if p >= end { wi = 247; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 248 {
                    if p >= end { wi = 248; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 249 {
                    if p >= end { wi = 249; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 6 + 4)
                }
                if wi <= 250 {
                    if p >= end { wi = 250; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 251 {
                    if p >= end { wi = 251; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 6 + 4)
                }
                if wi <= 252 {
                    if p >= end { wi = 252; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 253 {
                    if p >= end { wi = 253; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 254 {
                    if p >= end { wi = 254; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 255 {
                    if p >= end { wi = 255; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 256 {
                    if p >= end { wi = 256; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 257 {
                    if p >= end { wi = 257; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 6 + 4)
                }
                if wi <= 258 {
                    if p >= end { wi = 258; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 259 {
                    if p >= end { wi = 259; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 6 + 4)
                }
                if wi <= 260 {
                    if p >= end { wi = 260; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 261 {
                    if p >= end { wi = 261; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 262 {
                    if p >= end { wi = 262; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 263 {
                    if p >= end { wi = 263; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 264 {
                    if p >= end { wi = 264; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 265 {
                    if p >= end { wi = 265; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 6 + 4)
                }
                if wi <= 266 {
                    if p >= end { wi = 266; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 267 {
                    if p >= end { wi = 267; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 6 + 4)
                }
                if wi <= 268 {
                    if p >= end { wi = 268; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 269 {
                    if p >= end { wi = 269; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 270 {
                    if p >= end { wi = 270; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 271 {
                    if p >= end { wi = 271; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 272 {
                    if p >= end { wi = 272; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 273 {
                    if p >= end { wi = 273; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 6 + 4)
                }
                if wi <= 274 {
                    if p >= end { wi = 274; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 275 {
                    if p >= end { wi = 275; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 6 + 4)
                }
                if wi <= 276 {
                    if p >= end { wi = 276; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 277 {
                    if p >= end { wi = 277; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 278 {
                    if p >= end { wi = 278; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 279 {
                    if p >= end { wi = 279; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 280 {
                    if p >= end { wi = 280; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 281 {
                    if p >= end { wi = 281; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 6 + 4)
                }
                if wi <= 282 {
                    if p >= end { wi = 282; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 283 {
                    if p >= end { wi = 283; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 6 + 4)
                }
                if wi <= 284 {
                    if p >= end { wi = 284; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 285 {
                    if p >= end { wi = 285; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 286 {
                    if p >= end { wi = 286; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 4 + 2)
                }
                if wi <= 287 {
                    if p >= end { wi = 287; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 2 + 2)
                }
            }

            while p < loop_end {
                *p.offset(prime_ * 0 + 0) |= 1;
                *p.offset(prime_ * 4 + 2) |= 16;
                *p.offset(prime_ * 10 + 6) |= 4;
                *p.offset(prime_ * 12 + 7) |= 32;
                *p.offset(prime_ * 18 + 11) |= 8;
                *p.offset(prime_ * 22 + 13) |= 128;
                *p.offset(prime_ * 24 + 15) |= 2;
                *p.offset(prime_ * 28 + 17) |= 64;
                *p.offset(prime_ * 30 + 19) |= 1;
                *p.offset(prime_ * 34 + 21) |= 16;
                *p.offset(prime_ * 40 + 25) |= 4;
                *p.offset(prime_ * 42 + 26) |= 32;
                *p.offset(prime_ * 48 + 30) |= 8;
                *p.offset(prime_ * 52 + 32) |= 128;
                *p.offset(prime_ * 54 + 34) |= 2;
                *p.offset(prime_ * 58 + 36) |= 64;
                *p.offset(prime_ * 60 + 38) |= 1;
                *p.offset(prime_ * 64 + 40) |= 16;
                *p.offset(prime_ * 70 + 44) |= 4;
                *p.offset(prime_ * 72 + 45) |= 32;
                *p.offset(prime_ * 78 + 49) |= 8;
                *p.offset(prime_ * 82 + 51) |= 128;
                *p.offset(prime_ * 84 + 53) |= 2;
                *p.offset(prime_ * 88 + 55) |= 64;
                *p.offset(prime_ * 90 + 57) |= 1;
                *p.offset(prime_ * 94 + 59) |= 16;
                *p.offset(prime_ * 100 + 63) |= 4;
                *p.offset(prime_ * 102 + 64) |= 32;
                *p.offset(prime_ * 108 + 68) |= 8;
                *p.offset(prime_ * 112 + 70) |= 128;
                *p.offset(prime_ * 114 + 72) |= 2;
                *p.offset(prime_ * 118 + 74) |= 64;
                *p.offset(prime_ * 120 + 76) |= 1;
                *p.offset(prime_ * 124 + 78) |= 16;
                *p.offset(prime_ * 130 + 82) |= 4;
                *p.offset(prime_ * 132 + 83) |= 32;
                *p.offset(prime_ * 138 + 87) |= 8;
                *p.offset(prime_ * 142 + 89) |= 128;
                *p.offset(prime_ * 144 + 91) |= 2;
                *p.offset(prime_ * 148 + 93) |= 64;
                *p.offset(prime_ * 150 + 95) |= 1;
                *p.offset(prime_ * 154 + 97) |= 16;
                *p.offset(prime_ * 160 + 101) |= 4;
                *p.offset(prime_ * 162 + 102) |= 32;
                *p.offset(prime_ * 168 + 106) |= 8;
                *p.offset(prime_ * 172 + 108) |= 128;
                *p.offset(prime_ * 174 + 110) |= 2;
                *p.offset(prime_ * 178 + 112) |= 64;

                p = p.offset(prime_ * 180 + 114)
            }
            if wi <= 240 {
                if p >= end { wi = 240; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 241 {
                if p >= end { wi = 241; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 6 + 4)
            }
            if wi <= 242 {
                if p >= end { wi = 242; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 243 {
                if p >= end { wi = 243; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 6 + 4)
            }
            if wi <= 244 {
                if p >= end { wi = 244; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 245 {
                if p >= end { wi = 245; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 246 {
                if p >= end { wi = 246; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 247 {
                if p >= end { wi = 247; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 248 {
                if p >= end { wi = 248; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 249 {
                if p >= end { wi = 249; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 6 + 4)
            }
            if wi <= 250 {
                if p >= end { wi = 250; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 251 {
                if p >= end { wi = 251; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 6 + 4)
            }
            if wi <= 252 {
                if p >= end { wi = 252; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 253 {
                if p >= end { wi = 253; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 254 {
                if p >= end { wi = 254; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 255 {
                if p >= end { wi = 255; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 256 {
                if p >= end { wi = 256; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 257 {
                if p >= end { wi = 257; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 6 + 4)
            }
            if wi <= 258 {
                if p >= end { wi = 258; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 259 {
                if p >= end { wi = 259; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 6 + 4)
            }
            if wi <= 260 {
                if p >= end { wi = 260; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 261 {
                if p >= end { wi = 261; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 262 {
                if p >= end { wi = 262; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 263 {
                if p >= end { wi = 263; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 264 {
                if p >= end { wi = 264; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 265 {
                if p >= end { wi = 265; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 6 + 4)
            }
            if wi <= 266 {
                if p >= end { wi = 266; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 267 {
                if p >= end { wi = 267; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 6 + 4)
            }
            if wi <= 268 {
                if p >= end { wi = 268; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 269 {
                if p >= end { wi = 269; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 270 {
                if p >= end { wi = 270; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 271 {
                if p >= end { wi = 271; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 272 {
                if p >= end { wi = 272; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 273 {
                if p >= end { wi = 273; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 6 + 4)
            }
            if wi <= 274 {
                if p >= end { wi = 274; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 275 {
                if p >= end { wi = 275; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 6 + 4)
            }
            if wi <= 276 {
                if p >= end { wi = 276; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 277 {
                if p >= end { wi = 277; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 278 {
                if p >= end { wi = 278; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 279 {
                if p >= end { wi = 279; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 280 {
                if p >= end { wi = 280; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 281 {
                if p >= end { wi = 281; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 6 + 4)
            }
            if wi <= 282 {
                if p >= end { wi = 282; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 283 {
                if p >= end { wi = 283; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 6 + 4)
            }
            if wi <= 284 {
                if p >= end { wi = 284; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 285 {
                if p >= end { wi = 285; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 286 {
                if p >= end { wi = 286; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 4 + 2)
            }
            if wi <= 287 {
                if p >= end { wi = 287; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 2 + 2)
            }
        }
        288...335 => { // 30 * x + 23
            if wi != 288 {
                if wi <= 289 {
                    if p >= end { wi = 289; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 290 {
                    if p >= end { wi = 290; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 6 + 5)
                }
                if wi <= 291 {
                    if p >= end { wi = 291; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 292 {
                    if p >= end { wi = 292; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 293 {
                    if p >= end { wi = 293; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 294 {
                    if p >= end { wi = 294; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 295 {
                    if p >= end { wi = 295; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 296 {
                    if p >= end { wi = 296; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 6 + 5)
                }
                if wi <= 297 {
                    if p >= end { wi = 297; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 298 {
                    if p >= end { wi = 298; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 6 + 5)
                }
                if wi <= 299 {
                    if p >= end { wi = 299; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 300 {
                    if p >= end { wi = 300; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 301 {
                    if p >= end { wi = 301; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 302 {
                    if p >= end { wi = 302; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 303 {
                    if p >= end { wi = 303; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 304 {
                    if p >= end { wi = 304; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 6 + 5)
                }
                if wi <= 305 {
                    if p >= end { wi = 305; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 306 {
                    if p >= end { wi = 306; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 6 + 5)
                }
                if wi <= 307 {
                    if p >= end { wi = 307; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 308 {
                    if p >= end { wi = 308; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 309 {
                    if p >= end { wi = 309; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 310 {
                    if p >= end { wi = 310; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 311 {
                    if p >= end { wi = 311; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 312 {
                    if p >= end { wi = 312; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 6 + 5)
                }
                if wi <= 313 {
                    if p >= end { wi = 313; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 314 {
                    if p >= end { wi = 314; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 6 + 5)
                }
                if wi <= 315 {
                    if p >= end { wi = 315; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 316 {
                    if p >= end { wi = 316; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 317 {
                    if p >= end { wi = 317; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 318 {
                    if p >= end { wi = 318; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 319 {
                    if p >= end { wi = 319; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 320 {
                    if p >= end { wi = 320; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 6 + 5)
                }
                if wi <= 321 {
                    if p >= end { wi = 321; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 322 {
                    if p >= end { wi = 322; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 6 + 5)
                }
                if wi <= 323 {
                    if p >= end { wi = 323; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 324 {
                    if p >= end { wi = 324; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 325 {
                    if p >= end { wi = 325; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 326 {
                    if p >= end { wi = 326; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 327 {
                    if p >= end { wi = 327; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 328 {
                    if p >= end { wi = 328; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 6 + 5)
                }
                if wi <= 329 {
                    if p >= end { wi = 329; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 330 {
                    if p >= end { wi = 330; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 6 + 5)
                }
                if wi <= 331 {
                    if p >= end { wi = 331; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 3)
                }
                if wi <= 332 {
                    if p >= end { wi = 332; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 333 {
                    if p >= end { wi = 333; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 334 {
                    if p >= end { wi = 334; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 335 {
                    if p >= end { wi = 335; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 3)
                }
            }

            while p < loop_end {
                *p.offset(prime_ * 0 + 0) |= 32;
                *p.offset(prime_ * 6 + 5) |= 2;
                *p.offset(prime_ * 8 + 6) |= 64;
                *p.offset(prime_ * 14 + 11) |= 4;
                *p.offset(prime_ * 18 + 14) |= 8;
                *p.offset(prime_ * 20 + 15) |= 128;
                *p.offset(prime_ * 24 + 19) |= 1;
                *p.offset(prime_ * 26 + 20) |= 16;
                *p.offset(prime_ * 30 + 23) |= 32;
                *p.offset(prime_ * 36 + 28) |= 2;
                *p.offset(prime_ * 38 + 29) |= 64;
                *p.offset(prime_ * 44 + 34) |= 4;
                *p.offset(prime_ * 48 + 37) |= 8;
                *p.offset(prime_ * 50 + 38) |= 128;
                *p.offset(prime_ * 54 + 42) |= 1;
                *p.offset(prime_ * 56 + 43) |= 16;
                *p.offset(prime_ * 60 + 46) |= 32;
                *p.offset(prime_ * 66 + 51) |= 2;
                *p.offset(prime_ * 68 + 52) |= 64;
                *p.offset(prime_ * 74 + 57) |= 4;
                *p.offset(prime_ * 78 + 60) |= 8;
                *p.offset(prime_ * 80 + 61) |= 128;
                *p.offset(prime_ * 84 + 65) |= 1;
                *p.offset(prime_ * 86 + 66) |= 16;
                *p.offset(prime_ * 90 + 69) |= 32;
                *p.offset(prime_ * 96 + 74) |= 2;
                *p.offset(prime_ * 98 + 75) |= 64;
                *p.offset(prime_ * 104 + 80) |= 4;
                *p.offset(prime_ * 108 + 83) |= 8;
                *p.offset(prime_ * 110 + 84) |= 128;
                *p.offset(prime_ * 114 + 88) |= 1;
                *p.offset(prime_ * 116 + 89) |= 16;
                *p.offset(prime_ * 120 + 92) |= 32;
                *p.offset(prime_ * 126 + 97) |= 2;
                *p.offset(prime_ * 128 + 98) |= 64;
                *p.offset(prime_ * 134 + 103) |= 4;
                *p.offset(prime_ * 138 + 106) |= 8;
                *p.offset(prime_ * 140 + 107) |= 128;
                *p.offset(prime_ * 144 + 111) |= 1;
                *p.offset(prime_ * 146 + 112) |= 16;
                *p.offset(prime_ * 150 + 115) |= 32;
                *p.offset(prime_ * 156 + 120) |= 2;
                *p.offset(prime_ * 158 + 121) |= 64;
                *p.offset(prime_ * 164 + 126) |= 4;
                *p.offset(prime_ * 168 + 129) |= 8;
                *p.offset(prime_ * 170 + 130) |= 128;
                *p.offset(prime_ * 174 + 134) |= 1;
                *p.offset(prime_ * 176 + 135) |= 16;

                p = p.offset(prime_ * 180 + 138)
            }
            if wi <= 288 {
                if p >= end { wi = 288; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 6 + 5)
            }
            if wi <= 289 {
                if p >= end { wi = 289; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 290 {
                if p >= end { wi = 290; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 6 + 5)
            }
            if wi <= 291 {
                if p >= end { wi = 291; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 292 {
                if p >= end { wi = 292; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 293 {
                if p >= end { wi = 293; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 294 {
                if p >= end { wi = 294; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 295 {
                if p >= end { wi = 295; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 296 {
                if p >= end { wi = 296; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 6 + 5)
            }
            if wi <= 297 {
                if p >= end { wi = 297; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 298 {
                if p >= end { wi = 298; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 6 + 5)
            }
            if wi <= 299 {
                if p >= end { wi = 299; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 300 {
                if p >= end { wi = 300; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 301 {
                if p >= end { wi = 301; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 302 {
                if p >= end { wi = 302; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 303 {
                if p >= end { wi = 303; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 304 {
                if p >= end { wi = 304; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 6 + 5)
            }
            if wi <= 305 {
                if p >= end { wi = 305; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 306 {
                if p >= end { wi = 306; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 6 + 5)
            }
            if wi <= 307 {
                if p >= end { wi = 307; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 308 {
                if p >= end { wi = 308; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 309 {
                if p >= end { wi = 309; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 310 {
                if p >= end { wi = 310; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 311 {
                if p >= end { wi = 311; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 312 {
                if p >= end { wi = 312; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 6 + 5)
            }
            if wi <= 313 {
                if p >= end { wi = 313; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 314 {
                if p >= end { wi = 314; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 6 + 5)
            }
            if wi <= 315 {
                if p >= end { wi = 315; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 316 {
                if p >= end { wi = 316; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 317 {
                if p >= end { wi = 317; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 318 {
                if p >= end { wi = 318; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 319 {
                if p >= end { wi = 319; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 320 {
                if p >= end { wi = 320; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 6 + 5)
            }
            if wi <= 321 {
                if p >= end { wi = 321; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 322 {
                if p >= end { wi = 322; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 6 + 5)
            }
            if wi <= 323 {
                if p >= end { wi = 323; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 324 {
                if p >= end { wi = 324; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 325 {
                if p >= end { wi = 325; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 326 {
                if p >= end { wi = 326; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 327 {
                if p >= end { wi = 327; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 328 {
                if p >= end { wi = 328; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 6 + 5)
            }
            if wi <= 329 {
                if p >= end { wi = 329; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 330 {
                if p >= end { wi = 330; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 6 + 5)
            }
            if wi <= 331 {
                if p >= end { wi = 331; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 3)
            }
            if wi <= 332 {
                if p >= end { wi = 332; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 333 {
                if p >= end { wi = 333; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 334 {
                if p >= end { wi = 334; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 335 {
                if p >= end { wi = 335; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 3)
            }
        }
        336...383 => { // 30 * x + 29
            if wi != 336 {
                if wi <= 337 {
                    if p >= end { wi = 337; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 6 + 6)
                }
                if wi <= 338 {
                    if p >= end { wi = 338; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 339 {
                    if p >= end { wi = 339; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 340 {
                    if p >= end { wi = 340; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 341 {
                    if p >= end { wi = 341; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 342 {
                    if p >= end { wi = 342; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 343 {
                    if p >= end { wi = 343; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 6 + 6)
                }
                if wi <= 344 {
                    if p >= end { wi = 344; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 345 {
                    if p >= end { wi = 345; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 6 + 6)
                }
                if wi <= 346 {
                    if p >= end { wi = 346; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 347 {
                    if p >= end { wi = 347; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 348 {
                    if p >= end { wi = 348; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 349 {
                    if p >= end { wi = 349; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 350 {
                    if p >= end { wi = 350; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 351 {
                    if p >= end { wi = 351; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 6 + 6)
                }
                if wi <= 352 {
                    if p >= end { wi = 352; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 353 {
                    if p >= end { wi = 353; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 6 + 6)
                }
                if wi <= 354 {
                    if p >= end { wi = 354; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 355 {
                    if p >= end { wi = 355; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 356 {
                    if p >= end { wi = 356; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 357 {
                    if p >= end { wi = 357; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 358 {
                    if p >= end { wi = 358; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 359 {
                    if p >= end { wi = 359; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 6 + 6)
                }
                if wi <= 360 {
                    if p >= end { wi = 360; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 361 {
                    if p >= end { wi = 361; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 6 + 6)
                }
                if wi <= 362 {
                    if p >= end { wi = 362; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 363 {
                    if p >= end { wi = 363; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 364 {
                    if p >= end { wi = 364; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 365 {
                    if p >= end { wi = 365; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 366 {
                    if p >= end { wi = 366; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 367 {
                    if p >= end { wi = 367; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 6 + 6)
                }
                if wi <= 368 {
                    if p >= end { wi = 368; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 369 {
                    if p >= end { wi = 369; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 6 + 6)
                }
                if wi <= 370 {
                    if p >= end { wi = 370; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 371 {
                    if p >= end { wi = 371; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 372 {
                    if p >= end { wi = 372; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 373 {
                    if p >= end { wi = 373; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 374 {
                    if p >= end { wi = 374; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 375 {
                    if p >= end { wi = 375; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 6 + 6)
                }
                if wi <= 376 {
                    if p >= end { wi = 376; break 'outer; }
                    *p |= 1; p = p.offset(prime_ * 2 + 1)
                }
                if wi <= 377 {
                    if p >= end { wi = 377; break 'outer; }
                    *p |= 128; p = p.offset(prime_ * 6 + 6)
                }
                if wi <= 378 {
                    if p >= end { wi = 378; break 'outer; }
                    *p |= 64; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 379 {
                    if p >= end { wi = 379; break 'outer; }
                    *p |= 32; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 380 {
                    if p >= end { wi = 380; break 'outer; }
                    *p |= 16; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 381 {
                    if p >= end { wi = 381; break 'outer; }
                    *p |= 8; p = p.offset(prime_ * 2 + 2)
                }
                if wi <= 382 {
                    if p >= end { wi = 382; break 'outer; }
                    *p |= 4; p = p.offset(prime_ * 4 + 4)
                }
                if wi <= 383 {
                    if p >= end { wi = 383; break 'outer; }
                    *p |= 2; p = p.offset(prime_ * 6 + 6)
                }
            }

            while p < loop_end {
                *p.offset(prime_ * 0 + 0) |= 1;
                *p.offset(prime_ * 2 + 1) |= 128;
                *p.offset(prime_ * 8 + 7) |= 64;
                *p.offset(prime_ * 12 + 11) |= 32;
                *p.offset(prime_ * 14 + 13) |= 16;
                *p.offset(prime_ * 18 + 17) |= 8;
                *p.offset(prime_ * 20 + 19) |= 4;
                *p.offset(prime_ * 24 + 23) |= 2;
                *p.offset(prime_ * 30 + 29) |= 1;
                *p.offset(prime_ * 32 + 30) |= 128;
                *p.offset(prime_ * 38 + 36) |= 64;
                *p.offset(prime_ * 42 + 40) |= 32;
                *p.offset(prime_ * 44 + 42) |= 16;
                *p.offset(prime_ * 48 + 46) |= 8;
                *p.offset(prime_ * 50 + 48) |= 4;
                *p.offset(prime_ * 54 + 52) |= 2;
                *p.offset(prime_ * 60 + 58) |= 1;
                *p.offset(prime_ * 62 + 59) |= 128;
                *p.offset(prime_ * 68 + 65) |= 64;
                *p.offset(prime_ * 72 + 69) |= 32;
                *p.offset(prime_ * 74 + 71) |= 16;
                *p.offset(prime_ * 78 + 75) |= 8;
                *p.offset(prime_ * 80 + 77) |= 4;
                *p.offset(prime_ * 84 + 81) |= 2;
                *p.offset(prime_ * 90 + 87) |= 1;
                *p.offset(prime_ * 92 + 88) |= 128;
                *p.offset(prime_ * 98 + 94) |= 64;
                *p.offset(prime_ * 102 + 98) |= 32;
                *p.offset(prime_ * 104 + 100) |= 16;
                *p.offset(prime_ * 108 + 104) |= 8;
                *p.offset(prime_ * 110 + 106) |= 4;
                *p.offset(prime_ * 114 + 110) |= 2;
                *p.offset(prime_ * 120 + 116) |= 1;
                *p.offset(prime_ * 122 + 117) |= 128;
                *p.offset(prime_ * 128 + 123) |= 64;
                *p.offset(prime_ * 132 + 127) |= 32;
                *p.offset(prime_ * 134 + 129) |= 16;
                *p.offset(prime_ * 138 + 133) |= 8;
                *p.offset(prime_ * 140 + 135) |= 4;
                *p.offset(prime_ * 144 + 139) |= 2;
                *p.offset(prime_ * 150 + 145) |= 1;
                *p.offset(prime_ * 152 + 146) |= 128;
                *p.offset(prime_ * 158 + 152) |= 64;
                *p.offset(prime_ * 162 + 156) |= 32;
                *p.offset(prime_ * 164 + 158) |= 16;
                *p.offset(prime_ * 168 + 162) |= 8;
                *p.offset(prime_ * 170 + 164) |= 4;
                *p.offset(prime_ * 174 + 168) |= 2;

                p = p.offset(prime_ * 180 + 174)
            }
            if wi <= 336 {
                if p >= end { wi = 336; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 337 {
                if p >= end { wi = 337; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 6 + 6)
            }
            if wi <= 338 {
                if p >= end { wi = 338; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 339 {
                if p >= end { wi = 339; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 340 {
                if p >= end { wi = 340; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 341 {
                if p >= end { wi = 341; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 342 {
                if p >= end { wi = 342; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 343 {
                if p >= end { wi = 343; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 6 + 6)
            }
            if wi <= 344 {
                if p >= end { wi = 344; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 345 {
                if p >= end { wi = 345; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 6 + 6)
            }
            if wi <= 346 {
                if p >= end { wi = 346; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 347 {
                if p >= end { wi = 347; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 348 {
                if p >= end { wi = 348; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 349 {
                if p >= end { wi = 349; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 350 {
                if p >= end { wi = 350; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 351 {
                if p >= end { wi = 351; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 6 + 6)
            }
            if wi <= 352 {
                if p >= end { wi = 352; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 353 {
                if p >= end { wi = 353; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 6 + 6)
            }
            if wi <= 354 {
                if p >= end { wi = 354; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 355 {
                if p >= end { wi = 355; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 356 {
                if p >= end { wi = 356; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 357 {
                if p >= end { wi = 357; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 358 {
                if p >= end { wi = 358; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 359 {
                if p >= end { wi = 359; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 6 + 6)
            }
            if wi <= 360 {
                if p >= end { wi = 360; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 361 {
                if p >= end { wi = 361; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 6 + 6)
            }
            if wi <= 362 {
                if p >= end { wi = 362; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 363 {
                if p >= end { wi = 363; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 364 {
                if p >= end { wi = 364; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 365 {
                if p >= end { wi = 365; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 366 {
                if p >= end { wi = 366; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 367 {
                if p >= end { wi = 367; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 6 + 6)
            }
            if wi <= 368 {
                if p >= end { wi = 368; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 369 {
                if p >= end { wi = 369; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 6 + 6)
            }
            if wi <= 370 {
                if p >= end { wi = 370; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 371 {
                if p >= end { wi = 371; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 372 {
                if p >= end { wi = 372; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 373 {
                if p >= end { wi = 373; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 374 {
                if p >= end { wi = 374; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 375 {
                if p >= end { wi = 375; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 6 + 6)
            }
            if wi <= 376 {
                if p >= end { wi = 376; break 'outer; }
                *p |= 1; p = p.offset(prime_ * 2 + 1)
            }
            if wi <= 377 {
                if p >= end { wi = 377; break 'outer; }
                *p |= 128; p = p.offset(prime_ * 6 + 6)
            }
            if wi <= 378 {
                if p >= end { wi = 378; break 'outer; }
                *p |= 64; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 379 {
                if p >= end { wi = 379; break 'outer; }
                *p |= 32; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 380 {
                if p >= end { wi = 380; break 'outer; }
                *p |= 16; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 381 {
                if p >= end { wi = 381; break 'outer; }
                *p |= 8; p = p.offset(prime_ * 2 + 2)
            }
            if wi <= 382 {
                if p >= end { wi = 382; break 'outer; }
                *p |= 4; p = p.offset(prime_ * 4 + 4)
            }
            if wi <= 383 {
                if p >= end { wi = 383; break 'outer; }
                *p |= 2; p = p.offset(prime_ * 6 + 6)
            }
        }
        _ => unreachable!(),
    }
    break 'outer;
    }
    let si = p as usize - start as usize;
    *si_ = si.wrapping_sub(bytes.len());
    *wi_ = wi;
}

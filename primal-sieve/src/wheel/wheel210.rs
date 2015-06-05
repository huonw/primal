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
    let len = bytes.len() as isize;
    let largest_step = (178 * prime + 168) as isize;
    let loop_len = len - largest_step;
    let mut si = *si_ as isize;
    let mut wi = *wi_;
    let prime_ = prime as isize;

    'outer: loop {
    match wi {
        0...47 => { // 30 * x + 1
            loop {
             'label47: loop {
              'label46: loop {
               'label45: loop {
                'label44: loop {
                 'label43: loop {
                  'label42: loop {
                   'label41: loop {
                    'label40: loop {
                     'label39: loop {
                      'label38: loop {
                       'label37: loop {
                        'label36: loop {
                         'label35: loop {
                          'label34: loop {
                           'label33: loop {
                            'label32: loop {
                             'label31: loop {
                              'label30: loop {
                               'label29: loop {
                                'label28: loop {
                                 'label27: loop {
                                  'label26: loop {
                                   'label25: loop {
                                    'label24: loop {
                                     'label23: loop {
                                      'label22: loop {
                                       'label21: loop {
                                        'label20: loop {
                                         'label19: loop {
                                          'label18: loop {
                                           'label17: loop {
                                            'label16: loop {
                                             'label15: loop {
                                              'label14: loop {
                                               'label13: loop {
                                                'label12: loop {
                                                 'label11: loop {
                                                  'label10: loop {
                                                   'label9: loop {
                                                    'label8: loop {
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
                                                              7 => break 'label7,
                                                              8 => break 'label8,
                                                              9 => break 'label9,
                                                              10 => break 'label10,
                                                              11 => break 'label11,
                                                              12 => break 'label12,
                                                              13 => break 'label13,
                                                              14 => break 'label14,
                                                              15 => break 'label15,
                                                              16 => break 'label16,
                                                              17 => break 'label17,
                                                              18 => break 'label18,
                                                              19 => break 'label19,
                                                              20 => break 'label20,
                                                              21 => break 'label21,
                                                              22 => break 'label22,
                                                              23 => break 'label23,
                                                              24 => break 'label24,
                                                              25 => break 'label25,
                                                              26 => break 'label26,
                                                              27 => break 'label27,
                                                              28 => break 'label28,
                                                              29 => break 'label29,
                                                              30 => break 'label30,
                                                              31 => break 'label31,
                                                              32 => break 'label32,
                                                              33 => break 'label33,
                                                              34 => break 'label34,
                                                              35 => break 'label35,
                                                              36 => break 'label36,
                                                              37 => break 'label37,
                                                              38 => break 'label38,
                                                              39 => break 'label39,
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
                                                                *start.offset(si + prime_ * 6 + 0) |= 2;
                                                                *start.offset(si + prime_ * 10 + 0) |= 4;
                                                                *start.offset(si + prime_ * 12 + 0) |= 8;
                                                                *start.offset(si + prime_ * 16 + 0) |= 16;
                                                                *start.offset(si + prime_ * 18 + 0) |= 32;
                                                                *start.offset(si + prime_ * 22 + 0) |= 64;
                                                                *start.offset(si + prime_ * 28 + 0) |= 128;
                                                                *start.offset(si + prime_ * 30 + 1) |= 1;
                                                                *start.offset(si + prime_ * 36 + 1) |= 2;
                                                                *start.offset(si + prime_ * 40 + 1) |= 4;
                                                                *start.offset(si + prime_ * 42 + 1) |= 8;
                                                                *start.offset(si + prime_ * 46 + 1) |= 16;
                                                                *start.offset(si + prime_ * 48 + 1) |= 32;
                                                                *start.offset(si + prime_ * 52 + 1) |= 64;
                                                                *start.offset(si + prime_ * 58 + 1) |= 128;
                                                                *start.offset(si + prime_ * 60 + 2) |= 1;
                                                                *start.offset(si + prime_ * 66 + 2) |= 2;
                                                                *start.offset(si + prime_ * 70 + 2) |= 4;
                                                                *start.offset(si + prime_ * 72 + 2) |= 8;
                                                                *start.offset(si + prime_ * 76 + 2) |= 16;
                                                                *start.offset(si + prime_ * 78 + 2) |= 32;
                                                                *start.offset(si + prime_ * 82 + 2) |= 64;
                                                                *start.offset(si + prime_ * 88 + 2) |= 128;
                                                                *start.offset(si + prime_ * 90 + 3) |= 1;
                                                                *start.offset(si + prime_ * 96 + 3) |= 2;
                                                                *start.offset(si + prime_ * 100 + 3) |= 4;
                                                                *start.offset(si + prime_ * 102 + 3) |= 8;
                                                                *start.offset(si + prime_ * 106 + 3) |= 16;
                                                                *start.offset(si + prime_ * 108 + 3) |= 32;
                                                                *start.offset(si + prime_ * 112 + 3) |= 64;
                                                                *start.offset(si + prime_ * 118 + 3) |= 128;
                                                                *start.offset(si + prime_ * 120 + 4) |= 1;
                                                                *start.offset(si + prime_ * 126 + 4) |= 2;
                                                                *start.offset(si + prime_ * 130 + 4) |= 4;
                                                                *start.offset(si + prime_ * 132 + 4) |= 8;
                                                                *start.offset(si + prime_ * 136 + 4) |= 16;
                                                                *start.offset(si + prime_ * 138 + 4) |= 32;
                                                                *start.offset(si + prime_ * 142 + 4) |= 64;
                                                                *start.offset(si + prime_ * 148 + 4) |= 128;
                                                                *start.offset(si + prime_ * 150 + 5) |= 1;
                                                                *start.offset(si + prime_ * 156 + 5) |= 2;
                                                                *start.offset(si + prime_ * 160 + 5) |= 4;
                                                                *start.offset(si + prime_ * 162 + 5) |= 8;
                                                                *start.offset(si + prime_ * 166 + 5) |= 16;
                                                                *start.offset(si + prime_ * 168 + 5) |= 32;
                                                                *start.offset(si + prime_ * 172 + 5) |= 64;
                                                                *start.offset(si + prime_ * 178 + 5) |= 128;

                                                                si += prime_ * 180 + 6
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
                                                     break 'label8
                                                    }
                                                    if si >= len { wi = 8; break 'outer; }
                                                    *start.offset(si) |= 1; si += prime_ * 6 + 0;
                                                    break 'label9
                                                   }
                                                   if si >= len { wi = 9; break 'outer; }
                                                   *start.offset(si) |= 2; si += prime_ * 4 + 0;
                                                   break 'label10
                                                  }
                                                  if si >= len { wi = 10; break 'outer; }
                                                  *start.offset(si) |= 4; si += prime_ * 2 + 0;
                                                  break 'label11
                                                 }
                                                 if si >= len { wi = 11; break 'outer; }
                                                 *start.offset(si) |= 8; si += prime_ * 4 + 0;
                                                 break 'label12
                                                }
                                                if si >= len { wi = 12; break 'outer; }
                                                *start.offset(si) |= 16; si += prime_ * 2 + 0;
                                                break 'label13
                                               }
                                               if si >= len { wi = 13; break 'outer; }
                                               *start.offset(si) |= 32; si += prime_ * 4 + 0;
                                               break 'label14
                                              }
                                              if si >= len { wi = 14; break 'outer; }
                                              *start.offset(si) |= 64; si += prime_ * 6 + 0;
                                              break 'label15
                                             }
                                             if si >= len { wi = 15; break 'outer; }
                                             *start.offset(si) |= 128; si += prime_ * 2 + 1;
                                             break 'label16
                                            }
                                            if si >= len { wi = 16; break 'outer; }
                                            *start.offset(si) |= 1; si += prime_ * 6 + 0;
                                            break 'label17
                                           }
                                           if si >= len { wi = 17; break 'outer; }
                                           *start.offset(si) |= 2; si += prime_ * 4 + 0;
                                           break 'label18
                                          }
                                          if si >= len { wi = 18; break 'outer; }
                                          *start.offset(si) |= 4; si += prime_ * 2 + 0;
                                          break 'label19
                                         }
                                         if si >= len { wi = 19; break 'outer; }
                                         *start.offset(si) |= 8; si += prime_ * 4 + 0;
                                         break 'label20
                                        }
                                        if si >= len { wi = 20; break 'outer; }
                                        *start.offset(si) |= 16; si += prime_ * 2 + 0;
                                        break 'label21
                                       }
                                       if si >= len { wi = 21; break 'outer; }
                                       *start.offset(si) |= 32; si += prime_ * 4 + 0;
                                       break 'label22
                                      }
                                      if si >= len { wi = 22; break 'outer; }
                                      *start.offset(si) |= 64; si += prime_ * 6 + 0;
                                      break 'label23
                                     }
                                     if si >= len { wi = 23; break 'outer; }
                                     *start.offset(si) |= 128; si += prime_ * 2 + 1;
                                     break 'label24
                                    }
                                    if si >= len { wi = 24; break 'outer; }
                                    *start.offset(si) |= 1; si += prime_ * 6 + 0;
                                    break 'label25
                                   }
                                   if si >= len { wi = 25; break 'outer; }
                                   *start.offset(si) |= 2; si += prime_ * 4 + 0;
                                   break 'label26
                                  }
                                  if si >= len { wi = 26; break 'outer; }
                                  *start.offset(si) |= 4; si += prime_ * 2 + 0;
                                  break 'label27
                                 }
                                 if si >= len { wi = 27; break 'outer; }
                                 *start.offset(si) |= 8; si += prime_ * 4 + 0;
                                 break 'label28
                                }
                                if si >= len { wi = 28; break 'outer; }
                                *start.offset(si) |= 16; si += prime_ * 2 + 0;
                                break 'label29
                               }
                               if si >= len { wi = 29; break 'outer; }
                               *start.offset(si) |= 32; si += prime_ * 4 + 0;
                               break 'label30
                              }
                              if si >= len { wi = 30; break 'outer; }
                              *start.offset(si) |= 64; si += prime_ * 6 + 0;
                              break 'label31
                             }
                             if si >= len { wi = 31; break 'outer; }
                             *start.offset(si) |= 128; si += prime_ * 2 + 1;
                             break 'label32
                            }
                            if si >= len { wi = 32; break 'outer; }
                            *start.offset(si) |= 1; si += prime_ * 6 + 0;
                            break 'label33
                           }
                           if si >= len { wi = 33; break 'outer; }
                           *start.offset(si) |= 2; si += prime_ * 4 + 0;
                           break 'label34
                          }
                          if si >= len { wi = 34; break 'outer; }
                          *start.offset(si) |= 4; si += prime_ * 2 + 0;
                          break 'label35
                         }
                         if si >= len { wi = 35; break 'outer; }
                         *start.offset(si) |= 8; si += prime_ * 4 + 0;
                         break 'label36
                        }
                        if si >= len { wi = 36; break 'outer; }
                        *start.offset(si) |= 16; si += prime_ * 2 + 0;
                        break 'label37
                       }
                       if si >= len { wi = 37; break 'outer; }
                       *start.offset(si) |= 32; si += prime_ * 4 + 0;
                       break 'label38
                      }
                      if si >= len { wi = 38; break 'outer; }
                      *start.offset(si) |= 64; si += prime_ * 6 + 0;
                      break 'label39
                     }
                     if si >= len { wi = 39; break 'outer; }
                     *start.offset(si) |= 128; si += prime_ * 2 + 1;
                     break 'label40
                    }
                    if si >= len { wi = 40; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 6 + 0;
                    break 'label41
                   }
                   if si >= len { wi = 41; break 'outer; }
                   *start.offset(si) |= 2; si += prime_ * 4 + 0;
                   break 'label42
                  }
                  if si >= len { wi = 42; break 'outer; }
                  *start.offset(si) |= 4; si += prime_ * 2 + 0;
                  break 'label43
                 }
                 if si >= len { wi = 43; break 'outer; }
                 *start.offset(si) |= 8; si += prime_ * 4 + 0;
                 break 'label44
                }
                if si >= len { wi = 44; break 'outer; }
                *start.offset(si) |= 16; si += prime_ * 2 + 0;
                break 'label45
               }
               if si >= len { wi = 45; break 'outer; }
               *start.offset(si) |= 32; si += prime_ * 4 + 0;
               break 'label46
              }
              if si >= len { wi = 46; break 'outer; }
              *start.offset(si) |= 64; si += prime_ * 6 + 0;
              break 'label47
             }
             if si >= len { wi = 47; break 'outer; }
             *start.offset(si) |= 128; si += prime_ * 2 + 1;
             wi = 0
            }
        }
        48...95 => { // 30 * x + 7
            loop {
             'label95: loop {
              'label94: loop {
               'label93: loop {
                'label92: loop {
                 'label91: loop {
                  'label90: loop {
                   'label89: loop {
                    'label88: loop {
                     'label87: loop {
                      'label86: loop {
                       'label85: loop {
                        'label84: loop {
                         'label83: loop {
                          'label82: loop {
                           'label81: loop {
                            'label80: loop {
                             'label79: loop {
                              'label78: loop {
                               'label77: loop {
                                'label76: loop {
                                 'label75: loop {
                                  'label74: loop {
                                   'label73: loop {
                                    'label72: loop {
                                     'label71: loop {
                                      'label70: loop {
                                       'label69: loop {
                                        'label68: loop {
                                         'label67: loop {
                                          'label66: loop {
                                           'label65: loop {
                                            'label64: loop {
                                             'label63: loop {
                                              'label62: loop {
                                               'label61: loop {
                                                'label60: loop {
                                                 'label59: loop {
                                                  'label58: loop {
                                                   'label57: loop {
                                                    'label56: loop {
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
                                                              55 => break 'label55,
                                                              56 => break 'label56,
                                                              57 => break 'label57,
                                                              58 => break 'label58,
                                                              59 => break 'label59,
                                                              60 => break 'label60,
                                                              61 => break 'label61,
                                                              62 => break 'label62,
                                                              63 => break 'label63,
                                                              64 => break 'label64,
                                                              65 => break 'label65,
                                                              66 => break 'label66,
                                                              67 => break 'label67,
                                                              68 => break 'label68,
                                                              69 => break 'label69,
                                                              70 => break 'label70,
                                                              71 => break 'label71,
                                                              72 => break 'label72,
                                                              73 => break 'label73,
                                                              74 => break 'label74,
                                                              75 => break 'label75,
                                                              76 => break 'label76,
                                                              77 => break 'label77,
                                                              78 => break 'label78,
                                                              79 => break 'label79,
                                                              80 => break 'label80,
                                                              81 => break 'label81,
                                                              82 => break 'label82,
                                                              83 => break 'label83,
                                                              84 => break 'label84,
                                                              85 => break 'label85,
                                                              86 => break 'label86,
                                                              87 => break 'label87,
                                                              88 => break 'label88,
                                                              89 => break 'label89,
                                                              90 => break 'label90,
                                                              91 => break 'label91,
                                                              92 => break 'label92,
                                                              93 => break 'label93,
                                                              94 => break 'label94,
                                                              _ => break 'label95,
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
                                                                *start.offset(si + prime_ * 30 + 7) |= 32;
                                                                *start.offset(si + prime_ * 34 + 8) |= 16;
                                                                *start.offset(si + prime_ * 36 + 9) |= 1;
                                                                *start.offset(si + prime_ * 40 + 9) |= 128;
                                                                *start.offset(si + prime_ * 42 + 10) |= 8;
                                                                *start.offset(si + prime_ * 46 + 11) |= 4;
                                                                *start.offset(si + prime_ * 52 + 12) |= 64;
                                                                *start.offset(si + prime_ * 54 + 13) |= 2;
                                                                *start.offset(si + prime_ * 60 + 14) |= 32;
                                                                *start.offset(si + prime_ * 64 + 15) |= 16;
                                                                *start.offset(si + prime_ * 66 + 16) |= 1;
                                                                *start.offset(si + prime_ * 70 + 16) |= 128;
                                                                *start.offset(si + prime_ * 72 + 17) |= 8;
                                                                *start.offset(si + prime_ * 76 + 18) |= 4;
                                                                *start.offset(si + prime_ * 82 + 19) |= 64;
                                                                *start.offset(si + prime_ * 84 + 20) |= 2;
                                                                *start.offset(si + prime_ * 90 + 21) |= 32;
                                                                *start.offset(si + prime_ * 94 + 22) |= 16;
                                                                *start.offset(si + prime_ * 96 + 23) |= 1;
                                                                *start.offset(si + prime_ * 100 + 23) |= 128;
                                                                *start.offset(si + prime_ * 102 + 24) |= 8;
                                                                *start.offset(si + prime_ * 106 + 25) |= 4;
                                                                *start.offset(si + prime_ * 112 + 26) |= 64;
                                                                *start.offset(si + prime_ * 114 + 27) |= 2;
                                                                *start.offset(si + prime_ * 120 + 28) |= 32;
                                                                *start.offset(si + prime_ * 124 + 29) |= 16;
                                                                *start.offset(si + prime_ * 126 + 30) |= 1;
                                                                *start.offset(si + prime_ * 130 + 30) |= 128;
                                                                *start.offset(si + prime_ * 132 + 31) |= 8;
                                                                *start.offset(si + prime_ * 136 + 32) |= 4;
                                                                *start.offset(si + prime_ * 142 + 33) |= 64;
                                                                *start.offset(si + prime_ * 144 + 34) |= 2;
                                                                *start.offset(si + prime_ * 150 + 35) |= 32;
                                                                *start.offset(si + prime_ * 154 + 36) |= 16;
                                                                *start.offset(si + prime_ * 156 + 37) |= 1;
                                                                *start.offset(si + prime_ * 160 + 37) |= 128;
                                                                *start.offset(si + prime_ * 162 + 38) |= 8;
                                                                *start.offset(si + prime_ * 166 + 39) |= 4;
                                                                *start.offset(si + prime_ * 172 + 40) |= 64;
                                                                *start.offset(si + prime_ * 174 + 41) |= 2;

                                                                si += prime_ * 180 + 42
                                                            }
                                                            if si >= len { wi = 48; break 'outer; }
                                                            *start.offset(si) |= 32; si += prime_ * 4 + 1;
                                                            break 'label49
                                                           }
                                                           if si >= len { wi = 49; break 'outer; }
                                                           *start.offset(si) |= 16; si += prime_ * 2 + 1;
                                                           break 'label50
                                                          }
                                                          if si >= len { wi = 50; break 'outer; }
                                                          *start.offset(si) |= 1; si += prime_ * 4 + 0;
                                                          break 'label51
                                                         }
                                                         if si >= len { wi = 51; break 'outer; }
                                                         *start.offset(si) |= 128; si += prime_ * 2 + 1;
                                                         break 'label52
                                                        }
                                                        if si >= len { wi = 52; break 'outer; }
                                                        *start.offset(si) |= 8; si += prime_ * 4 + 1;
                                                        break 'label53
                                                       }
                                                       if si >= len { wi = 53; break 'outer; }
                                                       *start.offset(si) |= 4; si += prime_ * 6 + 1;
                                                       break 'label54
                                                      }
                                                      if si >= len { wi = 54; break 'outer; }
                                                      *start.offset(si) |= 64; si += prime_ * 2 + 1;
                                                      break 'label55
                                                     }
                                                     if si >= len { wi = 55; break 'outer; }
                                                     *start.offset(si) |= 2; si += prime_ * 6 + 1;
                                                     break 'label56
                                                    }
                                                    if si >= len { wi = 56; break 'outer; }
                                                    *start.offset(si) |= 32; si += prime_ * 4 + 1;
                                                    break 'label57
                                                   }
                                                   if si >= len { wi = 57; break 'outer; }
                                                   *start.offset(si) |= 16; si += prime_ * 2 + 1;
                                                   break 'label58
                                                  }
                                                  if si >= len { wi = 58; break 'outer; }
                                                  *start.offset(si) |= 1; si += prime_ * 4 + 0;
                                                  break 'label59
                                                 }
                                                 if si >= len { wi = 59; break 'outer; }
                                                 *start.offset(si) |= 128; si += prime_ * 2 + 1;
                                                 break 'label60
                                                }
                                                if si >= len { wi = 60; break 'outer; }
                                                *start.offset(si) |= 8; si += prime_ * 4 + 1;
                                                break 'label61
                                               }
                                               if si >= len { wi = 61; break 'outer; }
                                               *start.offset(si) |= 4; si += prime_ * 6 + 1;
                                               break 'label62
                                              }
                                              if si >= len { wi = 62; break 'outer; }
                                              *start.offset(si) |= 64; si += prime_ * 2 + 1;
                                              break 'label63
                                             }
                                             if si >= len { wi = 63; break 'outer; }
                                             *start.offset(si) |= 2; si += prime_ * 6 + 1;
                                             break 'label64
                                            }
                                            if si >= len { wi = 64; break 'outer; }
                                            *start.offset(si) |= 32; si += prime_ * 4 + 1;
                                            break 'label65
                                           }
                                           if si >= len { wi = 65; break 'outer; }
                                           *start.offset(si) |= 16; si += prime_ * 2 + 1;
                                           break 'label66
                                          }
                                          if si >= len { wi = 66; break 'outer; }
                                          *start.offset(si) |= 1; si += prime_ * 4 + 0;
                                          break 'label67
                                         }
                                         if si >= len { wi = 67; break 'outer; }
                                         *start.offset(si) |= 128; si += prime_ * 2 + 1;
                                         break 'label68
                                        }
                                        if si >= len { wi = 68; break 'outer; }
                                        *start.offset(si) |= 8; si += prime_ * 4 + 1;
                                        break 'label69
                                       }
                                       if si >= len { wi = 69; break 'outer; }
                                       *start.offset(si) |= 4; si += prime_ * 6 + 1;
                                       break 'label70
                                      }
                                      if si >= len { wi = 70; break 'outer; }
                                      *start.offset(si) |= 64; si += prime_ * 2 + 1;
                                      break 'label71
                                     }
                                     if si >= len { wi = 71; break 'outer; }
                                     *start.offset(si) |= 2; si += prime_ * 6 + 1;
                                     break 'label72
                                    }
                                    if si >= len { wi = 72; break 'outer; }
                                    *start.offset(si) |= 32; si += prime_ * 4 + 1;
                                    break 'label73
                                   }
                                   if si >= len { wi = 73; break 'outer; }
                                   *start.offset(si) |= 16; si += prime_ * 2 + 1;
                                   break 'label74
                                  }
                                  if si >= len { wi = 74; break 'outer; }
                                  *start.offset(si) |= 1; si += prime_ * 4 + 0;
                                  break 'label75
                                 }
                                 if si >= len { wi = 75; break 'outer; }
                                 *start.offset(si) |= 128; si += prime_ * 2 + 1;
                                 break 'label76
                                }
                                if si >= len { wi = 76; break 'outer; }
                                *start.offset(si) |= 8; si += prime_ * 4 + 1;
                                break 'label77
                               }
                               if si >= len { wi = 77; break 'outer; }
                               *start.offset(si) |= 4; si += prime_ * 6 + 1;
                               break 'label78
                              }
                              if si >= len { wi = 78; break 'outer; }
                              *start.offset(si) |= 64; si += prime_ * 2 + 1;
                              break 'label79
                             }
                             if si >= len { wi = 79; break 'outer; }
                             *start.offset(si) |= 2; si += prime_ * 6 + 1;
                             break 'label80
                            }
                            if si >= len { wi = 80; break 'outer; }
                            *start.offset(si) |= 32; si += prime_ * 4 + 1;
                            break 'label81
                           }
                           if si >= len { wi = 81; break 'outer; }
                           *start.offset(si) |= 16; si += prime_ * 2 + 1;
                           break 'label82
                          }
                          if si >= len { wi = 82; break 'outer; }
                          *start.offset(si) |= 1; si += prime_ * 4 + 0;
                          break 'label83
                         }
                         if si >= len { wi = 83; break 'outer; }
                         *start.offset(si) |= 128; si += prime_ * 2 + 1;
                         break 'label84
                        }
                        if si >= len { wi = 84; break 'outer; }
                        *start.offset(si) |= 8; si += prime_ * 4 + 1;
                        break 'label85
                       }
                       if si >= len { wi = 85; break 'outer; }
                       *start.offset(si) |= 4; si += prime_ * 6 + 1;
                       break 'label86
                      }
                      if si >= len { wi = 86; break 'outer; }
                      *start.offset(si) |= 64; si += prime_ * 2 + 1;
                      break 'label87
                     }
                     if si >= len { wi = 87; break 'outer; }
                     *start.offset(si) |= 2; si += prime_ * 6 + 1;
                     break 'label88
                    }
                    if si >= len { wi = 88; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 1;
                    break 'label89
                   }
                   if si >= len { wi = 89; break 'outer; }
                   *start.offset(si) |= 16; si += prime_ * 2 + 1;
                   break 'label90
                  }
                  if si >= len { wi = 90; break 'outer; }
                  *start.offset(si) |= 1; si += prime_ * 4 + 0;
                  break 'label91
                 }
                 if si >= len { wi = 91; break 'outer; }
                 *start.offset(si) |= 128; si += prime_ * 2 + 1;
                 break 'label92
                }
                if si >= len { wi = 92; break 'outer; }
                *start.offset(si) |= 8; si += prime_ * 4 + 1;
                break 'label93
               }
               if si >= len { wi = 93; break 'outer; }
               *start.offset(si) |= 4; si += prime_ * 6 + 1;
               break 'label94
              }
              if si >= len { wi = 94; break 'outer; }
              *start.offset(si) |= 64; si += prime_ * 2 + 1;
              break 'label95
             }
             if si >= len { wi = 95; break 'outer; }
             *start.offset(si) |= 2; si += prime_ * 6 + 1;
             wi = 48
            }
        }
        96...143 => { // 30 * x + 11
            loop {
             'label143: loop {
              'label142: loop {
               'label141: loop {
                'label140: loop {
                 'label139: loop {
                  'label138: loop {
                   'label137: loop {
                    'label136: loop {
                     'label135: loop {
                      'label134: loop {
                       'label133: loop {
                        'label132: loop {
                         'label131: loop {
                          'label130: loop {
                           'label129: loop {
                            'label128: loop {
                             'label127: loop {
                              'label126: loop {
                               'label125: loop {
                                'label124: loop {
                                 'label123: loop {
                                  'label122: loop {
                                   'label121: loop {
                                    'label120: loop {
                                     'label119: loop {
                                      'label118: loop {
                                       'label117: loop {
                                        'label116: loop {
                                         'label115: loop {
                                          'label114: loop {
                                           'label113: loop {
                                            'label112: loop {
                                             'label111: loop {
                                              'label110: loop {
                                               'label109: loop {
                                                'label108: loop {
                                                 'label107: loop {
                                                  'label106: loop {
                                                   'label105: loop {
                                                    'label104: loop {
                                                     'label103: loop {
                                                      'label102: loop {
                                                       'label101: loop {
                                                        'label100: loop {
                                                         'label99: loop {
                                                          'label98: loop {
                                                           'label97: loop {
                                                            'label96: loop {
                                                             match wi {
                                                              96 => break 'label96,
                                                              97 => break 'label97,
                                                              98 => break 'label98,
                                                              99 => break 'label99,
                                                              100 => break 'label100,
                                                              101 => break 'label101,
                                                              102 => break 'label102,
                                                              103 => break 'label103,
                                                              104 => break 'label104,
                                                              105 => break 'label105,
                                                              106 => break 'label106,
                                                              107 => break 'label107,
                                                              108 => break 'label108,
                                                              109 => break 'label109,
                                                              110 => break 'label110,
                                                              111 => break 'label111,
                                                              112 => break 'label112,
                                                              113 => break 'label113,
                                                              114 => break 'label114,
                                                              115 => break 'label115,
                                                              116 => break 'label116,
                                                              117 => break 'label117,
                                                              118 => break 'label118,
                                                              119 => break 'label119,
                                                              120 => break 'label120,
                                                              121 => break 'label121,
                                                              122 => break 'label122,
                                                              123 => break 'label123,
                                                              124 => break 'label124,
                                                              125 => break 'label125,
                                                              126 => break 'label126,
                                                              127 => break 'label127,
                                                              128 => break 'label128,
                                                              129 => break 'label129,
                                                              130 => break 'label130,
                                                              131 => break 'label131,
                                                              132 => break 'label132,
                                                              133 => break 'label133,
                                                              134 => break 'label134,
                                                              135 => break 'label135,
                                                              136 => break 'label136,
                                                              137 => break 'label137,
                                                              138 => break 'label138,
                                                              139 => break 'label139,
                                                              140 => break 'label140,
                                                              141 => break 'label141,
                                                              142 => break 'label142,
                                                              _ => break 'label143,
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
                                                                *start.offset(si + prime_ * 30 + 11) |= 1;
                                                                *start.offset(si + prime_ * 32 + 11) |= 64;
                                                                *start.offset(si + prime_ * 36 + 13) |= 2;
                                                                *start.offset(si + prime_ * 38 + 13) |= 128;
                                                                *start.offset(si + prime_ * 42 + 15) |= 8;
                                                                *start.offset(si + prime_ * 48 + 17) |= 32;
                                                                *start.offset(si + prime_ * 50 + 18) |= 4;
                                                                *start.offset(si + prime_ * 56 + 20) |= 16;
                                                                *start.offset(si + prime_ * 60 + 22) |= 1;
                                                                *start.offset(si + prime_ * 62 + 22) |= 64;
                                                                *start.offset(si + prime_ * 66 + 24) |= 2;
                                                                *start.offset(si + prime_ * 68 + 24) |= 128;
                                                                *start.offset(si + prime_ * 72 + 26) |= 8;
                                                                *start.offset(si + prime_ * 78 + 28) |= 32;
                                                                *start.offset(si + prime_ * 80 + 29) |= 4;
                                                                *start.offset(si + prime_ * 86 + 31) |= 16;
                                                                *start.offset(si + prime_ * 90 + 33) |= 1;
                                                                *start.offset(si + prime_ * 92 + 33) |= 64;
                                                                *start.offset(si + prime_ * 96 + 35) |= 2;
                                                                *start.offset(si + prime_ * 98 + 35) |= 128;
                                                                *start.offset(si + prime_ * 102 + 37) |= 8;
                                                                *start.offset(si + prime_ * 108 + 39) |= 32;
                                                                *start.offset(si + prime_ * 110 + 40) |= 4;
                                                                *start.offset(si + prime_ * 116 + 42) |= 16;
                                                                *start.offset(si + prime_ * 120 + 44) |= 1;
                                                                *start.offset(si + prime_ * 122 + 44) |= 64;
                                                                *start.offset(si + prime_ * 126 + 46) |= 2;
                                                                *start.offset(si + prime_ * 128 + 46) |= 128;
                                                                *start.offset(si + prime_ * 132 + 48) |= 8;
                                                                *start.offset(si + prime_ * 138 + 50) |= 32;
                                                                *start.offset(si + prime_ * 140 + 51) |= 4;
                                                                *start.offset(si + prime_ * 146 + 53) |= 16;
                                                                *start.offset(si + prime_ * 150 + 55) |= 1;
                                                                *start.offset(si + prime_ * 152 + 55) |= 64;
                                                                *start.offset(si + prime_ * 156 + 57) |= 2;
                                                                *start.offset(si + prime_ * 158 + 57) |= 128;
                                                                *start.offset(si + prime_ * 162 + 59) |= 8;
                                                                *start.offset(si + prime_ * 168 + 61) |= 32;
                                                                *start.offset(si + prime_ * 170 + 62) |= 4;
                                                                *start.offset(si + prime_ * 176 + 64) |= 16;

                                                                si += prime_ * 180 + 66
                                                            }
                                                            if si >= len { wi = 96; break 'outer; }
                                                            *start.offset(si) |= 1; si += prime_ * 2 + 0;
                                                            break 'label97
                                                           }
                                                           if si >= len { wi = 97; break 'outer; }
                                                           *start.offset(si) |= 64; si += prime_ * 4 + 2;
                                                           break 'label98
                                                          }
                                                          if si >= len { wi = 98; break 'outer; }
                                                          *start.offset(si) |= 2; si += prime_ * 2 + 0;
                                                          break 'label99
                                                         }
                                                         if si >= len { wi = 99; break 'outer; }
                                                         *start.offset(si) |= 128; si += prime_ * 4 + 2;
                                                         break 'label100
                                                        }
                                                        if si >= len { wi = 100; break 'outer; }
                                                        *start.offset(si) |= 8; si += prime_ * 6 + 2;
                                                        break 'label101
                                                       }
                                                       if si >= len { wi = 101; break 'outer; }
                                                       *start.offset(si) |= 32; si += prime_ * 2 + 1;
                                                       break 'label102
                                                      }
                                                      if si >= len { wi = 102; break 'outer; }
                                                      *start.offset(si) |= 4; si += prime_ * 6 + 2;
                                                      break 'label103
                                                     }
                                                     if si >= len { wi = 103; break 'outer; }
                                                     *start.offset(si) |= 16; si += prime_ * 4 + 2;
                                                     break 'label104
                                                    }
                                                    if si >= len { wi = 104; break 'outer; }
                                                    *start.offset(si) |= 1; si += prime_ * 2 + 0;
                                                    break 'label105
                                                   }
                                                   if si >= len { wi = 105; break 'outer; }
                                                   *start.offset(si) |= 64; si += prime_ * 4 + 2;
                                                   break 'label106
                                                  }
                                                  if si >= len { wi = 106; break 'outer; }
                                                  *start.offset(si) |= 2; si += prime_ * 2 + 0;
                                                  break 'label107
                                                 }
                                                 if si >= len { wi = 107; break 'outer; }
                                                 *start.offset(si) |= 128; si += prime_ * 4 + 2;
                                                 break 'label108
                                                }
                                                if si >= len { wi = 108; break 'outer; }
                                                *start.offset(si) |= 8; si += prime_ * 6 + 2;
                                                break 'label109
                                               }
                                               if si >= len { wi = 109; break 'outer; }
                                               *start.offset(si) |= 32; si += prime_ * 2 + 1;
                                               break 'label110
                                              }
                                              if si >= len { wi = 110; break 'outer; }
                                              *start.offset(si) |= 4; si += prime_ * 6 + 2;
                                              break 'label111
                                             }
                                             if si >= len { wi = 111; break 'outer; }
                                             *start.offset(si) |= 16; si += prime_ * 4 + 2;
                                             break 'label112
                                            }
                                            if si >= len { wi = 112; break 'outer; }
                                            *start.offset(si) |= 1; si += prime_ * 2 + 0;
                                            break 'label113
                                           }
                                           if si >= len { wi = 113; break 'outer; }
                                           *start.offset(si) |= 64; si += prime_ * 4 + 2;
                                           break 'label114
                                          }
                                          if si >= len { wi = 114; break 'outer; }
                                          *start.offset(si) |= 2; si += prime_ * 2 + 0;
                                          break 'label115
                                         }
                                         if si >= len { wi = 115; break 'outer; }
                                         *start.offset(si) |= 128; si += prime_ * 4 + 2;
                                         break 'label116
                                        }
                                        if si >= len { wi = 116; break 'outer; }
                                        *start.offset(si) |= 8; si += prime_ * 6 + 2;
                                        break 'label117
                                       }
                                       if si >= len { wi = 117; break 'outer; }
                                       *start.offset(si) |= 32; si += prime_ * 2 + 1;
                                       break 'label118
                                      }
                                      if si >= len { wi = 118; break 'outer; }
                                      *start.offset(si) |= 4; si += prime_ * 6 + 2;
                                      break 'label119
                                     }
                                     if si >= len { wi = 119; break 'outer; }
                                     *start.offset(si) |= 16; si += prime_ * 4 + 2;
                                     break 'label120
                                    }
                                    if si >= len { wi = 120; break 'outer; }
                                    *start.offset(si) |= 1; si += prime_ * 2 + 0;
                                    break 'label121
                                   }
                                   if si >= len { wi = 121; break 'outer; }
                                   *start.offset(si) |= 64; si += prime_ * 4 + 2;
                                   break 'label122
                                  }
                                  if si >= len { wi = 122; break 'outer; }
                                  *start.offset(si) |= 2; si += prime_ * 2 + 0;
                                  break 'label123
                                 }
                                 if si >= len { wi = 123; break 'outer; }
                                 *start.offset(si) |= 128; si += prime_ * 4 + 2;
                                 break 'label124
                                }
                                if si >= len { wi = 124; break 'outer; }
                                *start.offset(si) |= 8; si += prime_ * 6 + 2;
                                break 'label125
                               }
                               if si >= len { wi = 125; break 'outer; }
                               *start.offset(si) |= 32; si += prime_ * 2 + 1;
                               break 'label126
                              }
                              if si >= len { wi = 126; break 'outer; }
                              *start.offset(si) |= 4; si += prime_ * 6 + 2;
                              break 'label127
                             }
                             if si >= len { wi = 127; break 'outer; }
                             *start.offset(si) |= 16; si += prime_ * 4 + 2;
                             break 'label128
                            }
                            if si >= len { wi = 128; break 'outer; }
                            *start.offset(si) |= 1; si += prime_ * 2 + 0;
                            break 'label129
                           }
                           if si >= len { wi = 129; break 'outer; }
                           *start.offset(si) |= 64; si += prime_ * 4 + 2;
                           break 'label130
                          }
                          if si >= len { wi = 130; break 'outer; }
                          *start.offset(si) |= 2; si += prime_ * 2 + 0;
                          break 'label131
                         }
                         if si >= len { wi = 131; break 'outer; }
                         *start.offset(si) |= 128; si += prime_ * 4 + 2;
                         break 'label132
                        }
                        if si >= len { wi = 132; break 'outer; }
                        *start.offset(si) |= 8; si += prime_ * 6 + 2;
                        break 'label133
                       }
                       if si >= len { wi = 133; break 'outer; }
                       *start.offset(si) |= 32; si += prime_ * 2 + 1;
                       break 'label134
                      }
                      if si >= len { wi = 134; break 'outer; }
                      *start.offset(si) |= 4; si += prime_ * 6 + 2;
                      break 'label135
                     }
                     if si >= len { wi = 135; break 'outer; }
                     *start.offset(si) |= 16; si += prime_ * 4 + 2;
                     break 'label136
                    }
                    if si >= len { wi = 136; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 0;
                    break 'label137
                   }
                   if si >= len { wi = 137; break 'outer; }
                   *start.offset(si) |= 64; si += prime_ * 4 + 2;
                   break 'label138
                  }
                  if si >= len { wi = 138; break 'outer; }
                  *start.offset(si) |= 2; si += prime_ * 2 + 0;
                  break 'label139
                 }
                 if si >= len { wi = 139; break 'outer; }
                 *start.offset(si) |= 128; si += prime_ * 4 + 2;
                 break 'label140
                }
                if si >= len { wi = 140; break 'outer; }
                *start.offset(si) |= 8; si += prime_ * 6 + 2;
                break 'label141
               }
               if si >= len { wi = 141; break 'outer; }
               *start.offset(si) |= 32; si += prime_ * 2 + 1;
               break 'label142
              }
              if si >= len { wi = 142; break 'outer; }
              *start.offset(si) |= 4; si += prime_ * 6 + 2;
              break 'label143
             }
             if si >= len { wi = 143; break 'outer; }
             *start.offset(si) |= 16; si += prime_ * 4 + 2;
             wi = 96
            }
        }
        144...191 => { // 30 * x + 13
            loop {
             'label191: loop {
              'label190: loop {
               'label189: loop {
                'label188: loop {
                 'label187: loop {
                  'label186: loop {
                   'label185: loop {
                    'label184: loop {
                     'label183: loop {
                      'label182: loop {
                       'label181: loop {
                        'label180: loop {
                         'label179: loop {
                          'label178: loop {
                           'label177: loop {
                            'label176: loop {
                             'label175: loop {
                              'label174: loop {
                               'label173: loop {
                                'label172: loop {
                                 'label171: loop {
                                  'label170: loop {
                                   'label169: loop {
                                    'label168: loop {
                                     'label167: loop {
                                      'label166: loop {
                                       'label165: loop {
                                        'label164: loop {
                                         'label163: loop {
                                          'label162: loop {
                                           'label161: loop {
                                            'label160: loop {
                                             'label159: loop {
                                              'label158: loop {
                                               'label157: loop {
                                                'label156: loop {
                                                 'label155: loop {
                                                  'label154: loop {
                                                   'label153: loop {
                                                    'label152: loop {
                                                     'label151: loop {
                                                      'label150: loop {
                                                       'label149: loop {
                                                        'label148: loop {
                                                         'label147: loop {
                                                          'label146: loop {
                                                           'label145: loop {
                                                            'label144: loop {
                                                             match wi {
                                                              144 => break 'label144,
                                                              145 => break 'label145,
                                                              146 => break 'label146,
                                                              147 => break 'label147,
                                                              148 => break 'label148,
                                                              149 => break 'label149,
                                                              150 => break 'label150,
                                                              151 => break 'label151,
                                                              152 => break 'label152,
                                                              153 => break 'label153,
                                                              154 => break 'label154,
                                                              155 => break 'label155,
                                                              156 => break 'label156,
                                                              157 => break 'label157,
                                                              158 => break 'label158,
                                                              159 => break 'label159,
                                                              160 => break 'label160,
                                                              161 => break 'label161,
                                                              162 => break 'label162,
                                                              163 => break 'label163,
                                                              164 => break 'label164,
                                                              165 => break 'label165,
                                                              166 => break 'label166,
                                                              167 => break 'label167,
                                                              168 => break 'label168,
                                                              169 => break 'label169,
                                                              170 => break 'label170,
                                                              171 => break 'label171,
                                                              172 => break 'label172,
                                                              173 => break 'label173,
                                                              174 => break 'label174,
                                                              175 => break 'label175,
                                                              176 => break 'label176,
                                                              177 => break 'label177,
                                                              178 => break 'label178,
                                                              179 => break 'label179,
                                                              180 => break 'label180,
                                                              181 => break 'label181,
                                                              182 => break 'label182,
                                                              183 => break 'label183,
                                                              184 => break 'label184,
                                                              185 => break 'label185,
                                                              186 => break 'label186,
                                                              187 => break 'label187,
                                                              188 => break 'label188,
                                                              189 => break 'label189,
                                                              190 => break 'label190,
                                                              _ => break 'label191,
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
                                                                *start.offset(si + prime_ * 30 + 13) |= 32;
                                                                *start.offset(si + prime_ * 34 + 15) |= 4;
                                                                *start.offset(si + prime_ * 36 + 16) |= 2;
                                                                *start.offset(si + prime_ * 40 + 17) |= 128;
                                                                *start.offset(si + prime_ * 46 + 20) |= 16;
                                                                *start.offset(si + prime_ * 48 + 21) |= 8;
                                                                *start.offset(si + prime_ * 54 + 24) |= 1;
                                                                *start.offset(si + prime_ * 58 + 25) |= 64;
                                                                *start.offset(si + prime_ * 60 + 26) |= 32;
                                                                *start.offset(si + prime_ * 64 + 28) |= 4;
                                                                *start.offset(si + prime_ * 66 + 29) |= 2;
                                                                *start.offset(si + prime_ * 70 + 30) |= 128;
                                                                *start.offset(si + prime_ * 76 + 33) |= 16;
                                                                *start.offset(si + prime_ * 78 + 34) |= 8;
                                                                *start.offset(si + prime_ * 84 + 37) |= 1;
                                                                *start.offset(si + prime_ * 88 + 38) |= 64;
                                                                *start.offset(si + prime_ * 90 + 39) |= 32;
                                                                *start.offset(si + prime_ * 94 + 41) |= 4;
                                                                *start.offset(si + prime_ * 96 + 42) |= 2;
                                                                *start.offset(si + prime_ * 100 + 43) |= 128;
                                                                *start.offset(si + prime_ * 106 + 46) |= 16;
                                                                *start.offset(si + prime_ * 108 + 47) |= 8;
                                                                *start.offset(si + prime_ * 114 + 50) |= 1;
                                                                *start.offset(si + prime_ * 118 + 51) |= 64;
                                                                *start.offset(si + prime_ * 120 + 52) |= 32;
                                                                *start.offset(si + prime_ * 124 + 54) |= 4;
                                                                *start.offset(si + prime_ * 126 + 55) |= 2;
                                                                *start.offset(si + prime_ * 130 + 56) |= 128;
                                                                *start.offset(si + prime_ * 136 + 59) |= 16;
                                                                *start.offset(si + prime_ * 138 + 60) |= 8;
                                                                *start.offset(si + prime_ * 144 + 63) |= 1;
                                                                *start.offset(si + prime_ * 148 + 64) |= 64;
                                                                *start.offset(si + prime_ * 150 + 65) |= 32;
                                                                *start.offset(si + prime_ * 154 + 67) |= 4;
                                                                *start.offset(si + prime_ * 156 + 68) |= 2;
                                                                *start.offset(si + prime_ * 160 + 69) |= 128;
                                                                *start.offset(si + prime_ * 166 + 72) |= 16;
                                                                *start.offset(si + prime_ * 168 + 73) |= 8;
                                                                *start.offset(si + prime_ * 174 + 76) |= 1;
                                                                *start.offset(si + prime_ * 178 + 77) |= 64;

                                                                si += prime_ * 180 + 78
                                                            }
                                                            if si >= len { wi = 144; break 'outer; }
                                                            *start.offset(si) |= 32; si += prime_ * 4 + 2;
                                                            break 'label145
                                                           }
                                                           if si >= len { wi = 145; break 'outer; }
                                                           *start.offset(si) |= 4; si += prime_ * 2 + 1;
                                                           break 'label146
                                                          }
                                                          if si >= len { wi = 146; break 'outer; }
                                                          *start.offset(si) |= 2; si += prime_ * 4 + 1;
                                                          break 'label147
                                                         }
                                                         if si >= len { wi = 147; break 'outer; }
                                                         *start.offset(si) |= 128; si += prime_ * 6 + 3;
                                                         break 'label148
                                                        }
                                                        if si >= len { wi = 148; break 'outer; }
                                                        *start.offset(si) |= 16; si += prime_ * 2 + 1;
                                                        break 'label149
                                                       }
                                                       if si >= len { wi = 149; break 'outer; }
                                                       *start.offset(si) |= 8; si += prime_ * 6 + 3;
                                                       break 'label150
                                                      }
                                                      if si >= len { wi = 150; break 'outer; }
                                                      *start.offset(si) |= 1; si += prime_ * 4 + 1;
                                                      break 'label151
                                                     }
                                                     if si >= len { wi = 151; break 'outer; }
                                                     *start.offset(si) |= 64; si += prime_ * 2 + 1;
                                                     break 'label152
                                                    }
                                                    if si >= len { wi = 152; break 'outer; }
                                                    *start.offset(si) |= 32; si += prime_ * 4 + 2;
                                                    break 'label153
                                                   }
                                                   if si >= len { wi = 153; break 'outer; }
                                                   *start.offset(si) |= 4; si += prime_ * 2 + 1;
                                                   break 'label154
                                                  }
                                                  if si >= len { wi = 154; break 'outer; }
                                                  *start.offset(si) |= 2; si += prime_ * 4 + 1;
                                                  break 'label155
                                                 }
                                                 if si >= len { wi = 155; break 'outer; }
                                                 *start.offset(si) |= 128; si += prime_ * 6 + 3;
                                                 break 'label156
                                                }
                                                if si >= len { wi = 156; break 'outer; }
                                                *start.offset(si) |= 16; si += prime_ * 2 + 1;
                                                break 'label157
                                               }
                                               if si >= len { wi = 157; break 'outer; }
                                               *start.offset(si) |= 8; si += prime_ * 6 + 3;
                                               break 'label158
                                              }
                                              if si >= len { wi = 158; break 'outer; }
                                              *start.offset(si) |= 1; si += prime_ * 4 + 1;
                                              break 'label159
                                             }
                                             if si >= len { wi = 159; break 'outer; }
                                             *start.offset(si) |= 64; si += prime_ * 2 + 1;
                                             break 'label160
                                            }
                                            if si >= len { wi = 160; break 'outer; }
                                            *start.offset(si) |= 32; si += prime_ * 4 + 2;
                                            break 'label161
                                           }
                                           if si >= len { wi = 161; break 'outer; }
                                           *start.offset(si) |= 4; si += prime_ * 2 + 1;
                                           break 'label162
                                          }
                                          if si >= len { wi = 162; break 'outer; }
                                          *start.offset(si) |= 2; si += prime_ * 4 + 1;
                                          break 'label163
                                         }
                                         if si >= len { wi = 163; break 'outer; }
                                         *start.offset(si) |= 128; si += prime_ * 6 + 3;
                                         break 'label164
                                        }
                                        if si >= len { wi = 164; break 'outer; }
                                        *start.offset(si) |= 16; si += prime_ * 2 + 1;
                                        break 'label165
                                       }
                                       if si >= len { wi = 165; break 'outer; }
                                       *start.offset(si) |= 8; si += prime_ * 6 + 3;
                                       break 'label166
                                      }
                                      if si >= len { wi = 166; break 'outer; }
                                      *start.offset(si) |= 1; si += prime_ * 4 + 1;
                                      break 'label167
                                     }
                                     if si >= len { wi = 167; break 'outer; }
                                     *start.offset(si) |= 64; si += prime_ * 2 + 1;
                                     break 'label168
                                    }
                                    if si >= len { wi = 168; break 'outer; }
                                    *start.offset(si) |= 32; si += prime_ * 4 + 2;
                                    break 'label169
                                   }
                                   if si >= len { wi = 169; break 'outer; }
                                   *start.offset(si) |= 4; si += prime_ * 2 + 1;
                                   break 'label170
                                  }
                                  if si >= len { wi = 170; break 'outer; }
                                  *start.offset(si) |= 2; si += prime_ * 4 + 1;
                                  break 'label171
                                 }
                                 if si >= len { wi = 171; break 'outer; }
                                 *start.offset(si) |= 128; si += prime_ * 6 + 3;
                                 break 'label172
                                }
                                if si >= len { wi = 172; break 'outer; }
                                *start.offset(si) |= 16; si += prime_ * 2 + 1;
                                break 'label173
                               }
                               if si >= len { wi = 173; break 'outer; }
                               *start.offset(si) |= 8; si += prime_ * 6 + 3;
                               break 'label174
                              }
                              if si >= len { wi = 174; break 'outer; }
                              *start.offset(si) |= 1; si += prime_ * 4 + 1;
                              break 'label175
                             }
                             if si >= len { wi = 175; break 'outer; }
                             *start.offset(si) |= 64; si += prime_ * 2 + 1;
                             break 'label176
                            }
                            if si >= len { wi = 176; break 'outer; }
                            *start.offset(si) |= 32; si += prime_ * 4 + 2;
                            break 'label177
                           }
                           if si >= len { wi = 177; break 'outer; }
                           *start.offset(si) |= 4; si += prime_ * 2 + 1;
                           break 'label178
                          }
                          if si >= len { wi = 178; break 'outer; }
                          *start.offset(si) |= 2; si += prime_ * 4 + 1;
                          break 'label179
                         }
                         if si >= len { wi = 179; break 'outer; }
                         *start.offset(si) |= 128; si += prime_ * 6 + 3;
                         break 'label180
                        }
                        if si >= len { wi = 180; break 'outer; }
                        *start.offset(si) |= 16; si += prime_ * 2 + 1;
                        break 'label181
                       }
                       if si >= len { wi = 181; break 'outer; }
                       *start.offset(si) |= 8; si += prime_ * 6 + 3;
                       break 'label182
                      }
                      if si >= len { wi = 182; break 'outer; }
                      *start.offset(si) |= 1; si += prime_ * 4 + 1;
                      break 'label183
                     }
                     if si >= len { wi = 183; break 'outer; }
                     *start.offset(si) |= 64; si += prime_ * 2 + 1;
                     break 'label184
                    }
                    if si >= len { wi = 184; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 2;
                    break 'label185
                   }
                   if si >= len { wi = 185; break 'outer; }
                   *start.offset(si) |= 4; si += prime_ * 2 + 1;
                   break 'label186
                  }
                  if si >= len { wi = 186; break 'outer; }
                  *start.offset(si) |= 2; si += prime_ * 4 + 1;
                  break 'label187
                 }
                 if si >= len { wi = 187; break 'outer; }
                 *start.offset(si) |= 128; si += prime_ * 6 + 3;
                 break 'label188
                }
                if si >= len { wi = 188; break 'outer; }
                *start.offset(si) |= 16; si += prime_ * 2 + 1;
                break 'label189
               }
               if si >= len { wi = 189; break 'outer; }
               *start.offset(si) |= 8; si += prime_ * 6 + 3;
               break 'label190
              }
              if si >= len { wi = 190; break 'outer; }
              *start.offset(si) |= 1; si += prime_ * 4 + 1;
              break 'label191
             }
             if si >= len { wi = 191; break 'outer; }
             *start.offset(si) |= 64; si += prime_ * 2 + 1;
             wi = 144
            }
        }
        192...239 => { // 30 * x + 17
            loop {
             'label239: loop {
              'label238: loop {
               'label237: loop {
                'label236: loop {
                 'label235: loop {
                  'label234: loop {
                   'label233: loop {
                    'label232: loop {
                     'label231: loop {
                      'label230: loop {
                       'label229: loop {
                        'label228: loop {
                         'label227: loop {
                          'label226: loop {
                           'label225: loop {
                            'label224: loop {
                             'label223: loop {
                              'label222: loop {
                               'label221: loop {
                                'label220: loop {
                                 'label219: loop {
                                  'label218: loop {
                                   'label217: loop {
                                    'label216: loop {
                                     'label215: loop {
                                      'label214: loop {
                                       'label213: loop {
                                        'label212: loop {
                                         'label211: loop {
                                          'label210: loop {
                                           'label209: loop {
                                            'label208: loop {
                                             'label207: loop {
                                              'label206: loop {
                                               'label205: loop {
                                                'label204: loop {
                                                 'label203: loop {
                                                  'label202: loop {
                                                   'label201: loop {
                                                    'label200: loop {
                                                     'label199: loop {
                                                      'label198: loop {
                                                       'label197: loop {
                                                        'label196: loop {
                                                         'label195: loop {
                                                          'label194: loop {
                                                           'label193: loop {
                                                            'label192: loop {
                                                             match wi {
                                                              192 => break 'label192,
                                                              193 => break 'label193,
                                                              194 => break 'label194,
                                                              195 => break 'label195,
                                                              196 => break 'label196,
                                                              197 => break 'label197,
                                                              198 => break 'label198,
                                                              199 => break 'label199,
                                                              200 => break 'label200,
                                                              201 => break 'label201,
                                                              202 => break 'label202,
                                                              203 => break 'label203,
                                                              204 => break 'label204,
                                                              205 => break 'label205,
                                                              206 => break 'label206,
                                                              207 => break 'label207,
                                                              208 => break 'label208,
                                                              209 => break 'label209,
                                                              210 => break 'label210,
                                                              211 => break 'label211,
                                                              212 => break 'label212,
                                                              213 => break 'label213,
                                                              214 => break 'label214,
                                                              215 => break 'label215,
                                                              216 => break 'label216,
                                                              217 => break 'label217,
                                                              218 => break 'label218,
                                                              219 => break 'label219,
                                                              220 => break 'label220,
                                                              221 => break 'label221,
                                                              222 => break 'label222,
                                                              223 => break 'label223,
                                                              224 => break 'label224,
                                                              225 => break 'label225,
                                                              226 => break 'label226,
                                                              227 => break 'label227,
                                                              228 => break 'label228,
                                                              229 => break 'label229,
                                                              230 => break 'label230,
                                                              231 => break 'label231,
                                                              232 => break 'label232,
                                                              233 => break 'label233,
                                                              234 => break 'label234,
                                                              235 => break 'label235,
                                                              236 => break 'label236,
                                                              237 => break 'label237,
                                                              238 => break 'label238,
                                                              _ => break 'label239,
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
                                                                *start.offset(si + prime_ * 30 + 17) |= 32;
                                                                *start.offset(si + prime_ * 32 + 18) |= 64;
                                                                *start.offset(si + prime_ * 36 + 21) |= 1;
                                                                *start.offset(si + prime_ * 42 + 24) |= 8;
                                                                *start.offset(si + prime_ * 44 + 25) |= 16;
                                                                *start.offset(si + prime_ * 50 + 28) |= 128;
                                                                *start.offset(si + prime_ * 54 + 31) |= 2;
                                                                *start.offset(si + prime_ * 56 + 32) |= 4;
                                                                *start.offset(si + prime_ * 60 + 34) |= 32;
                                                                *start.offset(si + prime_ * 62 + 35) |= 64;
                                                                *start.offset(si + prime_ * 66 + 38) |= 1;
                                                                *start.offset(si + prime_ * 72 + 41) |= 8;
                                                                *start.offset(si + prime_ * 74 + 42) |= 16;
                                                                *start.offset(si + prime_ * 80 + 45) |= 128;
                                                                *start.offset(si + prime_ * 84 + 48) |= 2;
                                                                *start.offset(si + prime_ * 86 + 49) |= 4;
                                                                *start.offset(si + prime_ * 90 + 51) |= 32;
                                                                *start.offset(si + prime_ * 92 + 52) |= 64;
                                                                *start.offset(si + prime_ * 96 + 55) |= 1;
                                                                *start.offset(si + prime_ * 102 + 58) |= 8;
                                                                *start.offset(si + prime_ * 104 + 59) |= 16;
                                                                *start.offset(si + prime_ * 110 + 62) |= 128;
                                                                *start.offset(si + prime_ * 114 + 65) |= 2;
                                                                *start.offset(si + prime_ * 116 + 66) |= 4;
                                                                *start.offset(si + prime_ * 120 + 68) |= 32;
                                                                *start.offset(si + prime_ * 122 + 69) |= 64;
                                                                *start.offset(si + prime_ * 126 + 72) |= 1;
                                                                *start.offset(si + prime_ * 132 + 75) |= 8;
                                                                *start.offset(si + prime_ * 134 + 76) |= 16;
                                                                *start.offset(si + prime_ * 140 + 79) |= 128;
                                                                *start.offset(si + prime_ * 144 + 82) |= 2;
                                                                *start.offset(si + prime_ * 146 + 83) |= 4;
                                                                *start.offset(si + prime_ * 150 + 85) |= 32;
                                                                *start.offset(si + prime_ * 152 + 86) |= 64;
                                                                *start.offset(si + prime_ * 156 + 89) |= 1;
                                                                *start.offset(si + prime_ * 162 + 92) |= 8;
                                                                *start.offset(si + prime_ * 164 + 93) |= 16;
                                                                *start.offset(si + prime_ * 170 + 96) |= 128;
                                                                *start.offset(si + prime_ * 174 + 99) |= 2;
                                                                *start.offset(si + prime_ * 176 + 100) |= 4;

                                                                si += prime_ * 180 + 102
                                                            }
                                                            if si >= len { wi = 192; break 'outer; }
                                                            *start.offset(si) |= 32; si += prime_ * 2 + 1;
                                                            break 'label193
                                                           }
                                                           if si >= len { wi = 193; break 'outer; }
                                                           *start.offset(si) |= 64; si += prime_ * 4 + 3;
                                                           break 'label194
                                                          }
                                                          if si >= len { wi = 194; break 'outer; }
                                                          *start.offset(si) |= 1; si += prime_ * 6 + 3;
                                                          break 'label195
                                                         }
                                                         if si >= len { wi = 195; break 'outer; }
                                                         *start.offset(si) |= 8; si += prime_ * 2 + 1;
                                                         break 'label196
                                                        }
                                                        if si >= len { wi = 196; break 'outer; }
                                                        *start.offset(si) |= 16; si += prime_ * 6 + 3;
                                                        break 'label197
                                                       }
                                                       if si >= len { wi = 197; break 'outer; }
                                                       *start.offset(si) |= 128; si += prime_ * 4 + 3;
                                                       break 'label198
                                                      }
                                                      if si >= len { wi = 198; break 'outer; }
                                                      *start.offset(si) |= 2; si += prime_ * 2 + 1;
                                                      break 'label199
                                                     }
                                                     if si >= len { wi = 199; break 'outer; }
                                                     *start.offset(si) |= 4; si += prime_ * 4 + 2;
                                                     break 'label200
                                                    }
                                                    if si >= len { wi = 200; break 'outer; }
                                                    *start.offset(si) |= 32; si += prime_ * 2 + 1;
                                                    break 'label201
                                                   }
                                                   if si >= len { wi = 201; break 'outer; }
                                                   *start.offset(si) |= 64; si += prime_ * 4 + 3;
                                                   break 'label202
                                                  }
                                                  if si >= len { wi = 202; break 'outer; }
                                                  *start.offset(si) |= 1; si += prime_ * 6 + 3;
                                                  break 'label203
                                                 }
                                                 if si >= len { wi = 203; break 'outer; }
                                                 *start.offset(si) |= 8; si += prime_ * 2 + 1;
                                                 break 'label204
                                                }
                                                if si >= len { wi = 204; break 'outer; }
                                                *start.offset(si) |= 16; si += prime_ * 6 + 3;
                                                break 'label205
                                               }
                                               if si >= len { wi = 205; break 'outer; }
                                               *start.offset(si) |= 128; si += prime_ * 4 + 3;
                                               break 'label206
                                              }
                                              if si >= len { wi = 206; break 'outer; }
                                              *start.offset(si) |= 2; si += prime_ * 2 + 1;
                                              break 'label207
                                             }
                                             if si >= len { wi = 207; break 'outer; }
                                             *start.offset(si) |= 4; si += prime_ * 4 + 2;
                                             break 'label208
                                            }
                                            if si >= len { wi = 208; break 'outer; }
                                            *start.offset(si) |= 32; si += prime_ * 2 + 1;
                                            break 'label209
                                           }
                                           if si >= len { wi = 209; break 'outer; }
                                           *start.offset(si) |= 64; si += prime_ * 4 + 3;
                                           break 'label210
                                          }
                                          if si >= len { wi = 210; break 'outer; }
                                          *start.offset(si) |= 1; si += prime_ * 6 + 3;
                                          break 'label211
                                         }
                                         if si >= len { wi = 211; break 'outer; }
                                         *start.offset(si) |= 8; si += prime_ * 2 + 1;
                                         break 'label212
                                        }
                                        if si >= len { wi = 212; break 'outer; }
                                        *start.offset(si) |= 16; si += prime_ * 6 + 3;
                                        break 'label213
                                       }
                                       if si >= len { wi = 213; break 'outer; }
                                       *start.offset(si) |= 128; si += prime_ * 4 + 3;
                                       break 'label214
                                      }
                                      if si >= len { wi = 214; break 'outer; }
                                      *start.offset(si) |= 2; si += prime_ * 2 + 1;
                                      break 'label215
                                     }
                                     if si >= len { wi = 215; break 'outer; }
                                     *start.offset(si) |= 4; si += prime_ * 4 + 2;
                                     break 'label216
                                    }
                                    if si >= len { wi = 216; break 'outer; }
                                    *start.offset(si) |= 32; si += prime_ * 2 + 1;
                                    break 'label217
                                   }
                                   if si >= len { wi = 217; break 'outer; }
                                   *start.offset(si) |= 64; si += prime_ * 4 + 3;
                                   break 'label218
                                  }
                                  if si >= len { wi = 218; break 'outer; }
                                  *start.offset(si) |= 1; si += prime_ * 6 + 3;
                                  break 'label219
                                 }
                                 if si >= len { wi = 219; break 'outer; }
                                 *start.offset(si) |= 8; si += prime_ * 2 + 1;
                                 break 'label220
                                }
                                if si >= len { wi = 220; break 'outer; }
                                *start.offset(si) |= 16; si += prime_ * 6 + 3;
                                break 'label221
                               }
                               if si >= len { wi = 221; break 'outer; }
                               *start.offset(si) |= 128; si += prime_ * 4 + 3;
                               break 'label222
                              }
                              if si >= len { wi = 222; break 'outer; }
                              *start.offset(si) |= 2; si += prime_ * 2 + 1;
                              break 'label223
                             }
                             if si >= len { wi = 223; break 'outer; }
                             *start.offset(si) |= 4; si += prime_ * 4 + 2;
                             break 'label224
                            }
                            if si >= len { wi = 224; break 'outer; }
                            *start.offset(si) |= 32; si += prime_ * 2 + 1;
                            break 'label225
                           }
                           if si >= len { wi = 225; break 'outer; }
                           *start.offset(si) |= 64; si += prime_ * 4 + 3;
                           break 'label226
                          }
                          if si >= len { wi = 226; break 'outer; }
                          *start.offset(si) |= 1; si += prime_ * 6 + 3;
                          break 'label227
                         }
                         if si >= len { wi = 227; break 'outer; }
                         *start.offset(si) |= 8; si += prime_ * 2 + 1;
                         break 'label228
                        }
                        if si >= len { wi = 228; break 'outer; }
                        *start.offset(si) |= 16; si += prime_ * 6 + 3;
                        break 'label229
                       }
                       if si >= len { wi = 229; break 'outer; }
                       *start.offset(si) |= 128; si += prime_ * 4 + 3;
                       break 'label230
                      }
                      if si >= len { wi = 230; break 'outer; }
                      *start.offset(si) |= 2; si += prime_ * 2 + 1;
                      break 'label231
                     }
                     if si >= len { wi = 231; break 'outer; }
                     *start.offset(si) |= 4; si += prime_ * 4 + 2;
                     break 'label232
                    }
                    if si >= len { wi = 232; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 1;
                    break 'label233
                   }
                   if si >= len { wi = 233; break 'outer; }
                   *start.offset(si) |= 64; si += prime_ * 4 + 3;
                   break 'label234
                  }
                  if si >= len { wi = 234; break 'outer; }
                  *start.offset(si) |= 1; si += prime_ * 6 + 3;
                  break 'label235
                 }
                 if si >= len { wi = 235; break 'outer; }
                 *start.offset(si) |= 8; si += prime_ * 2 + 1;
                 break 'label236
                }
                if si >= len { wi = 236; break 'outer; }
                *start.offset(si) |= 16; si += prime_ * 6 + 3;
                break 'label237
               }
               if si >= len { wi = 237; break 'outer; }
               *start.offset(si) |= 128; si += prime_ * 4 + 3;
               break 'label238
              }
              if si >= len { wi = 238; break 'outer; }
              *start.offset(si) |= 2; si += prime_ * 2 + 1;
              break 'label239
             }
             if si >= len { wi = 239; break 'outer; }
             *start.offset(si) |= 4; si += prime_ * 4 + 2;
             wi = 192
            }
        }
        240...287 => { // 30 * x + 19
            loop {
             'label287: loop {
              'label286: loop {
               'label285: loop {
                'label284: loop {
                 'label283: loop {
                  'label282: loop {
                   'label281: loop {
                    'label280: loop {
                     'label279: loop {
                      'label278: loop {
                       'label277: loop {
                        'label276: loop {
                         'label275: loop {
                          'label274: loop {
                           'label273: loop {
                            'label272: loop {
                             'label271: loop {
                              'label270: loop {
                               'label269: loop {
                                'label268: loop {
                                 'label267: loop {
                                  'label266: loop {
                                   'label265: loop {
                                    'label264: loop {
                                     'label263: loop {
                                      'label262: loop {
                                       'label261: loop {
                                        'label260: loop {
                                         'label259: loop {
                                          'label258: loop {
                                           'label257: loop {
                                            'label256: loop {
                                             'label255: loop {
                                              'label254: loop {
                                               'label253: loop {
                                                'label252: loop {
                                                 'label251: loop {
                                                  'label250: loop {
                                                   'label249: loop {
                                                    'label248: loop {
                                                     'label247: loop {
                                                      'label246: loop {
                                                       'label245: loop {
                                                        'label244: loop {
                                                         'label243: loop {
                                                          'label242: loop {
                                                           'label241: loop {
                                                            'label240: loop {
                                                             match wi {
                                                              240 => break 'label240,
                                                              241 => break 'label241,
                                                              242 => break 'label242,
                                                              243 => break 'label243,
                                                              244 => break 'label244,
                                                              245 => break 'label245,
                                                              246 => break 'label246,
                                                              247 => break 'label247,
                                                              248 => break 'label248,
                                                              249 => break 'label249,
                                                              250 => break 'label250,
                                                              251 => break 'label251,
                                                              252 => break 'label252,
                                                              253 => break 'label253,
                                                              254 => break 'label254,
                                                              255 => break 'label255,
                                                              256 => break 'label256,
                                                              257 => break 'label257,
                                                              258 => break 'label258,
                                                              259 => break 'label259,
                                                              260 => break 'label260,
                                                              261 => break 'label261,
                                                              262 => break 'label262,
                                                              263 => break 'label263,
                                                              264 => break 'label264,
                                                              265 => break 'label265,
                                                              266 => break 'label266,
                                                              267 => break 'label267,
                                                              268 => break 'label268,
                                                              269 => break 'label269,
                                                              270 => break 'label270,
                                                              271 => break 'label271,
                                                              272 => break 'label272,
                                                              273 => break 'label273,
                                                              274 => break 'label274,
                                                              275 => break 'label275,
                                                              276 => break 'label276,
                                                              277 => break 'label277,
                                                              278 => break 'label278,
                                                              279 => break 'label279,
                                                              280 => break 'label280,
                                                              281 => break 'label281,
                                                              282 => break 'label282,
                                                              283 => break 'label283,
                                                              284 => break 'label284,
                                                              285 => break 'label285,
                                                              286 => break 'label286,
                                                              _ => break 'label287,
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
                                                                *start.offset(si + prime_ * 30 + 19) |= 1;
                                                                *start.offset(si + prime_ * 34 + 21) |= 16;
                                                                *start.offset(si + prime_ * 40 + 25) |= 4;
                                                                *start.offset(si + prime_ * 42 + 26) |= 32;
                                                                *start.offset(si + prime_ * 48 + 30) |= 8;
                                                                *start.offset(si + prime_ * 52 + 32) |= 128;
                                                                *start.offset(si + prime_ * 54 + 34) |= 2;
                                                                *start.offset(si + prime_ * 58 + 36) |= 64;
                                                                *start.offset(si + prime_ * 60 + 38) |= 1;
                                                                *start.offset(si + prime_ * 64 + 40) |= 16;
                                                                *start.offset(si + prime_ * 70 + 44) |= 4;
                                                                *start.offset(si + prime_ * 72 + 45) |= 32;
                                                                *start.offset(si + prime_ * 78 + 49) |= 8;
                                                                *start.offset(si + prime_ * 82 + 51) |= 128;
                                                                *start.offset(si + prime_ * 84 + 53) |= 2;
                                                                *start.offset(si + prime_ * 88 + 55) |= 64;
                                                                *start.offset(si + prime_ * 90 + 57) |= 1;
                                                                *start.offset(si + prime_ * 94 + 59) |= 16;
                                                                *start.offset(si + prime_ * 100 + 63) |= 4;
                                                                *start.offset(si + prime_ * 102 + 64) |= 32;
                                                                *start.offset(si + prime_ * 108 + 68) |= 8;
                                                                *start.offset(si + prime_ * 112 + 70) |= 128;
                                                                *start.offset(si + prime_ * 114 + 72) |= 2;
                                                                *start.offset(si + prime_ * 118 + 74) |= 64;
                                                                *start.offset(si + prime_ * 120 + 76) |= 1;
                                                                *start.offset(si + prime_ * 124 + 78) |= 16;
                                                                *start.offset(si + prime_ * 130 + 82) |= 4;
                                                                *start.offset(si + prime_ * 132 + 83) |= 32;
                                                                *start.offset(si + prime_ * 138 + 87) |= 8;
                                                                *start.offset(si + prime_ * 142 + 89) |= 128;
                                                                *start.offset(si + prime_ * 144 + 91) |= 2;
                                                                *start.offset(si + prime_ * 148 + 93) |= 64;
                                                                *start.offset(si + prime_ * 150 + 95) |= 1;
                                                                *start.offset(si + prime_ * 154 + 97) |= 16;
                                                                *start.offset(si + prime_ * 160 + 101) |= 4;
                                                                *start.offset(si + prime_ * 162 + 102) |= 32;
                                                                *start.offset(si + prime_ * 168 + 106) |= 8;
                                                                *start.offset(si + prime_ * 172 + 108) |= 128;
                                                                *start.offset(si + prime_ * 174 + 110) |= 2;
                                                                *start.offset(si + prime_ * 178 + 112) |= 64;

                                                                si += prime_ * 180 + 114
                                                            }
                                                            if si >= len { wi = 240; break 'outer; }
                                                            *start.offset(si) |= 1; si += prime_ * 4 + 2;
                                                            break 'label241
                                                           }
                                                           if si >= len { wi = 241; break 'outer; }
                                                           *start.offset(si) |= 16; si += prime_ * 6 + 4;
                                                           break 'label242
                                                          }
                                                          if si >= len { wi = 242; break 'outer; }
                                                          *start.offset(si) |= 4; si += prime_ * 2 + 1;
                                                          break 'label243
                                                         }
                                                         if si >= len { wi = 243; break 'outer; }
                                                         *start.offset(si) |= 32; si += prime_ * 6 + 4;
                                                         break 'label244
                                                        }
                                                        if si >= len { wi = 244; break 'outer; }
                                                        *start.offset(si) |= 8; si += prime_ * 4 + 2;
                                                        break 'label245
                                                       }
                                                       if si >= len { wi = 245; break 'outer; }
                                                       *start.offset(si) |= 128; si += prime_ * 2 + 2;
                                                       break 'label246
                                                      }
                                                      if si >= len { wi = 246; break 'outer; }
                                                      *start.offset(si) |= 2; si += prime_ * 4 + 2;
                                                      break 'label247
                                                     }
                                                     if si >= len { wi = 247; break 'outer; }
                                                     *start.offset(si) |= 64; si += prime_ * 2 + 2;
                                                     break 'label248
                                                    }
                                                    if si >= len { wi = 248; break 'outer; }
                                                    *start.offset(si) |= 1; si += prime_ * 4 + 2;
                                                    break 'label249
                                                   }
                                                   if si >= len { wi = 249; break 'outer; }
                                                   *start.offset(si) |= 16; si += prime_ * 6 + 4;
                                                   break 'label250
                                                  }
                                                  if si >= len { wi = 250; break 'outer; }
                                                  *start.offset(si) |= 4; si += prime_ * 2 + 1;
                                                  break 'label251
                                                 }
                                                 if si >= len { wi = 251; break 'outer; }
                                                 *start.offset(si) |= 32; si += prime_ * 6 + 4;
                                                 break 'label252
                                                }
                                                if si >= len { wi = 252; break 'outer; }
                                                *start.offset(si) |= 8; si += prime_ * 4 + 2;
                                                break 'label253
                                               }
                                               if si >= len { wi = 253; break 'outer; }
                                               *start.offset(si) |= 128; si += prime_ * 2 + 2;
                                               break 'label254
                                              }
                                              if si >= len { wi = 254; break 'outer; }
                                              *start.offset(si) |= 2; si += prime_ * 4 + 2;
                                              break 'label255
                                             }
                                             if si >= len { wi = 255; break 'outer; }
                                             *start.offset(si) |= 64; si += prime_ * 2 + 2;
                                             break 'label256
                                            }
                                            if si >= len { wi = 256; break 'outer; }
                                            *start.offset(si) |= 1; si += prime_ * 4 + 2;
                                            break 'label257
                                           }
                                           if si >= len { wi = 257; break 'outer; }
                                           *start.offset(si) |= 16; si += prime_ * 6 + 4;
                                           break 'label258
                                          }
                                          if si >= len { wi = 258; break 'outer; }
                                          *start.offset(si) |= 4; si += prime_ * 2 + 1;
                                          break 'label259
                                         }
                                         if si >= len { wi = 259; break 'outer; }
                                         *start.offset(si) |= 32; si += prime_ * 6 + 4;
                                         break 'label260
                                        }
                                        if si >= len { wi = 260; break 'outer; }
                                        *start.offset(si) |= 8; si += prime_ * 4 + 2;
                                        break 'label261
                                       }
                                       if si >= len { wi = 261; break 'outer; }
                                       *start.offset(si) |= 128; si += prime_ * 2 + 2;
                                       break 'label262
                                      }
                                      if si >= len { wi = 262; break 'outer; }
                                      *start.offset(si) |= 2; si += prime_ * 4 + 2;
                                      break 'label263
                                     }
                                     if si >= len { wi = 263; break 'outer; }
                                     *start.offset(si) |= 64; si += prime_ * 2 + 2;
                                     break 'label264
                                    }
                                    if si >= len { wi = 264; break 'outer; }
                                    *start.offset(si) |= 1; si += prime_ * 4 + 2;
                                    break 'label265
                                   }
                                   if si >= len { wi = 265; break 'outer; }
                                   *start.offset(si) |= 16; si += prime_ * 6 + 4;
                                   break 'label266
                                  }
                                  if si >= len { wi = 266; break 'outer; }
                                  *start.offset(si) |= 4; si += prime_ * 2 + 1;
                                  break 'label267
                                 }
                                 if si >= len { wi = 267; break 'outer; }
                                 *start.offset(si) |= 32; si += prime_ * 6 + 4;
                                 break 'label268
                                }
                                if si >= len { wi = 268; break 'outer; }
                                *start.offset(si) |= 8; si += prime_ * 4 + 2;
                                break 'label269
                               }
                               if si >= len { wi = 269; break 'outer; }
                               *start.offset(si) |= 128; si += prime_ * 2 + 2;
                               break 'label270
                              }
                              if si >= len { wi = 270; break 'outer; }
                              *start.offset(si) |= 2; si += prime_ * 4 + 2;
                              break 'label271
                             }
                             if si >= len { wi = 271; break 'outer; }
                             *start.offset(si) |= 64; si += prime_ * 2 + 2;
                             break 'label272
                            }
                            if si >= len { wi = 272; break 'outer; }
                            *start.offset(si) |= 1; si += prime_ * 4 + 2;
                            break 'label273
                           }
                           if si >= len { wi = 273; break 'outer; }
                           *start.offset(si) |= 16; si += prime_ * 6 + 4;
                           break 'label274
                          }
                          if si >= len { wi = 274; break 'outer; }
                          *start.offset(si) |= 4; si += prime_ * 2 + 1;
                          break 'label275
                         }
                         if si >= len { wi = 275; break 'outer; }
                         *start.offset(si) |= 32; si += prime_ * 6 + 4;
                         break 'label276
                        }
                        if si >= len { wi = 276; break 'outer; }
                        *start.offset(si) |= 8; si += prime_ * 4 + 2;
                        break 'label277
                       }
                       if si >= len { wi = 277; break 'outer; }
                       *start.offset(si) |= 128; si += prime_ * 2 + 2;
                       break 'label278
                      }
                      if si >= len { wi = 278; break 'outer; }
                      *start.offset(si) |= 2; si += prime_ * 4 + 2;
                      break 'label279
                     }
                     if si >= len { wi = 279; break 'outer; }
                     *start.offset(si) |= 64; si += prime_ * 2 + 2;
                     break 'label280
                    }
                    if si >= len { wi = 280; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 2;
                    break 'label281
                   }
                   if si >= len { wi = 281; break 'outer; }
                   *start.offset(si) |= 16; si += prime_ * 6 + 4;
                   break 'label282
                  }
                  if si >= len { wi = 282; break 'outer; }
                  *start.offset(si) |= 4; si += prime_ * 2 + 1;
                  break 'label283
                 }
                 if si >= len { wi = 283; break 'outer; }
                 *start.offset(si) |= 32; si += prime_ * 6 + 4;
                 break 'label284
                }
                if si >= len { wi = 284; break 'outer; }
                *start.offset(si) |= 8; si += prime_ * 4 + 2;
                break 'label285
               }
               if si >= len { wi = 285; break 'outer; }
               *start.offset(si) |= 128; si += prime_ * 2 + 2;
               break 'label286
              }
              if si >= len { wi = 286; break 'outer; }
              *start.offset(si) |= 2; si += prime_ * 4 + 2;
              break 'label287
             }
             if si >= len { wi = 287; break 'outer; }
             *start.offset(si) |= 64; si += prime_ * 2 + 2;
             wi = 240
            }
        }
        288...335 => { // 30 * x + 23
            loop {
             'label335: loop {
              'label334: loop {
               'label333: loop {
                'label332: loop {
                 'label331: loop {
                  'label330: loop {
                   'label329: loop {
                    'label328: loop {
                     'label327: loop {
                      'label326: loop {
                       'label325: loop {
                        'label324: loop {
                         'label323: loop {
                          'label322: loop {
                           'label321: loop {
                            'label320: loop {
                             'label319: loop {
                              'label318: loop {
                               'label317: loop {
                                'label316: loop {
                                 'label315: loop {
                                  'label314: loop {
                                   'label313: loop {
                                    'label312: loop {
                                     'label311: loop {
                                      'label310: loop {
                                       'label309: loop {
                                        'label308: loop {
                                         'label307: loop {
                                          'label306: loop {
                                           'label305: loop {
                                            'label304: loop {
                                             'label303: loop {
                                              'label302: loop {
                                               'label301: loop {
                                                'label300: loop {
                                                 'label299: loop {
                                                  'label298: loop {
                                                   'label297: loop {
                                                    'label296: loop {
                                                     'label295: loop {
                                                      'label294: loop {
                                                       'label293: loop {
                                                        'label292: loop {
                                                         'label291: loop {
                                                          'label290: loop {
                                                           'label289: loop {
                                                            'label288: loop {
                                                             match wi {
                                                              288 => break 'label288,
                                                              289 => break 'label289,
                                                              290 => break 'label290,
                                                              291 => break 'label291,
                                                              292 => break 'label292,
                                                              293 => break 'label293,
                                                              294 => break 'label294,
                                                              295 => break 'label295,
                                                              296 => break 'label296,
                                                              297 => break 'label297,
                                                              298 => break 'label298,
                                                              299 => break 'label299,
                                                              300 => break 'label300,
                                                              301 => break 'label301,
                                                              302 => break 'label302,
                                                              303 => break 'label303,
                                                              304 => break 'label304,
                                                              305 => break 'label305,
                                                              306 => break 'label306,
                                                              307 => break 'label307,
                                                              308 => break 'label308,
                                                              309 => break 'label309,
                                                              310 => break 'label310,
                                                              311 => break 'label311,
                                                              312 => break 'label312,
                                                              313 => break 'label313,
                                                              314 => break 'label314,
                                                              315 => break 'label315,
                                                              316 => break 'label316,
                                                              317 => break 'label317,
                                                              318 => break 'label318,
                                                              319 => break 'label319,
                                                              320 => break 'label320,
                                                              321 => break 'label321,
                                                              322 => break 'label322,
                                                              323 => break 'label323,
                                                              324 => break 'label324,
                                                              325 => break 'label325,
                                                              326 => break 'label326,
                                                              327 => break 'label327,
                                                              328 => break 'label328,
                                                              329 => break 'label329,
                                                              330 => break 'label330,
                                                              331 => break 'label331,
                                                              332 => break 'label332,
                                                              333 => break 'label333,
                                                              334 => break 'label334,
                                                              _ => break 'label335,
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
                                                                *start.offset(si + prime_ * 30 + 23) |= 32;
                                                                *start.offset(si + prime_ * 36 + 28) |= 2;
                                                                *start.offset(si + prime_ * 38 + 29) |= 64;
                                                                *start.offset(si + prime_ * 44 + 34) |= 4;
                                                                *start.offset(si + prime_ * 48 + 37) |= 8;
                                                                *start.offset(si + prime_ * 50 + 38) |= 128;
                                                                *start.offset(si + prime_ * 54 + 42) |= 1;
                                                                *start.offset(si + prime_ * 56 + 43) |= 16;
                                                                *start.offset(si + prime_ * 60 + 46) |= 32;
                                                                *start.offset(si + prime_ * 66 + 51) |= 2;
                                                                *start.offset(si + prime_ * 68 + 52) |= 64;
                                                                *start.offset(si + prime_ * 74 + 57) |= 4;
                                                                *start.offset(si + prime_ * 78 + 60) |= 8;
                                                                *start.offset(si + prime_ * 80 + 61) |= 128;
                                                                *start.offset(si + prime_ * 84 + 65) |= 1;
                                                                *start.offset(si + prime_ * 86 + 66) |= 16;
                                                                *start.offset(si + prime_ * 90 + 69) |= 32;
                                                                *start.offset(si + prime_ * 96 + 74) |= 2;
                                                                *start.offset(si + prime_ * 98 + 75) |= 64;
                                                                *start.offset(si + prime_ * 104 + 80) |= 4;
                                                                *start.offset(si + prime_ * 108 + 83) |= 8;
                                                                *start.offset(si + prime_ * 110 + 84) |= 128;
                                                                *start.offset(si + prime_ * 114 + 88) |= 1;
                                                                *start.offset(si + prime_ * 116 + 89) |= 16;
                                                                *start.offset(si + prime_ * 120 + 92) |= 32;
                                                                *start.offset(si + prime_ * 126 + 97) |= 2;
                                                                *start.offset(si + prime_ * 128 + 98) |= 64;
                                                                *start.offset(si + prime_ * 134 + 103) |= 4;
                                                                *start.offset(si + prime_ * 138 + 106) |= 8;
                                                                *start.offset(si + prime_ * 140 + 107) |= 128;
                                                                *start.offset(si + prime_ * 144 + 111) |= 1;
                                                                *start.offset(si + prime_ * 146 + 112) |= 16;
                                                                *start.offset(si + prime_ * 150 + 115) |= 32;
                                                                *start.offset(si + prime_ * 156 + 120) |= 2;
                                                                *start.offset(si + prime_ * 158 + 121) |= 64;
                                                                *start.offset(si + prime_ * 164 + 126) |= 4;
                                                                *start.offset(si + prime_ * 168 + 129) |= 8;
                                                                *start.offset(si + prime_ * 170 + 130) |= 128;
                                                                *start.offset(si + prime_ * 174 + 134) |= 1;
                                                                *start.offset(si + prime_ * 176 + 135) |= 16;

                                                                si += prime_ * 180 + 138
                                                            }
                                                            if si >= len { wi = 288; break 'outer; }
                                                            *start.offset(si) |= 32; si += prime_ * 6 + 5;
                                                            break 'label289
                                                           }
                                                           if si >= len { wi = 289; break 'outer; }
                                                           *start.offset(si) |= 2; si += prime_ * 2 + 1;
                                                           break 'label290
                                                          }
                                                          if si >= len { wi = 290; break 'outer; }
                                                          *start.offset(si) |= 64; si += prime_ * 6 + 5;
                                                          break 'label291
                                                         }
                                                         if si >= len { wi = 291; break 'outer; }
                                                         *start.offset(si) |= 4; si += prime_ * 4 + 3;
                                                         break 'label292
                                                        }
                                                        if si >= len { wi = 292; break 'outer; }
                                                        *start.offset(si) |= 8; si += prime_ * 2 + 1;
                                                        break 'label293
                                                       }
                                                       if si >= len { wi = 293; break 'outer; }
                                                       *start.offset(si) |= 128; si += prime_ * 4 + 4;
                                                       break 'label294
                                                      }
                                                      if si >= len { wi = 294; break 'outer; }
                                                      *start.offset(si) |= 1; si += prime_ * 2 + 1;
                                                      break 'label295
                                                     }
                                                     if si >= len { wi = 295; break 'outer; }
                                                     *start.offset(si) |= 16; si += prime_ * 4 + 3;
                                                     break 'label296
                                                    }
                                                    if si >= len { wi = 296; break 'outer; }
                                                    *start.offset(si) |= 32; si += prime_ * 6 + 5;
                                                    break 'label297
                                                   }
                                                   if si >= len { wi = 297; break 'outer; }
                                                   *start.offset(si) |= 2; si += prime_ * 2 + 1;
                                                   break 'label298
                                                  }
                                                  if si >= len { wi = 298; break 'outer; }
                                                  *start.offset(si) |= 64; si += prime_ * 6 + 5;
                                                  break 'label299
                                                 }
                                                 if si >= len { wi = 299; break 'outer; }
                                                 *start.offset(si) |= 4; si += prime_ * 4 + 3;
                                                 break 'label300
                                                }
                                                if si >= len { wi = 300; break 'outer; }
                                                *start.offset(si) |= 8; si += prime_ * 2 + 1;
                                                break 'label301
                                               }
                                               if si >= len { wi = 301; break 'outer; }
                                               *start.offset(si) |= 128; si += prime_ * 4 + 4;
                                               break 'label302
                                              }
                                              if si >= len { wi = 302; break 'outer; }
                                              *start.offset(si) |= 1; si += prime_ * 2 + 1;
                                              break 'label303
                                             }
                                             if si >= len { wi = 303; break 'outer; }
                                             *start.offset(si) |= 16; si += prime_ * 4 + 3;
                                             break 'label304
                                            }
                                            if si >= len { wi = 304; break 'outer; }
                                            *start.offset(si) |= 32; si += prime_ * 6 + 5;
                                            break 'label305
                                           }
                                           if si >= len { wi = 305; break 'outer; }
                                           *start.offset(si) |= 2; si += prime_ * 2 + 1;
                                           break 'label306
                                          }
                                          if si >= len { wi = 306; break 'outer; }
                                          *start.offset(si) |= 64; si += prime_ * 6 + 5;
                                          break 'label307
                                         }
                                         if si >= len { wi = 307; break 'outer; }
                                         *start.offset(si) |= 4; si += prime_ * 4 + 3;
                                         break 'label308
                                        }
                                        if si >= len { wi = 308; break 'outer; }
                                        *start.offset(si) |= 8; si += prime_ * 2 + 1;
                                        break 'label309
                                       }
                                       if si >= len { wi = 309; break 'outer; }
                                       *start.offset(si) |= 128; si += prime_ * 4 + 4;
                                       break 'label310
                                      }
                                      if si >= len { wi = 310; break 'outer; }
                                      *start.offset(si) |= 1; si += prime_ * 2 + 1;
                                      break 'label311
                                     }
                                     if si >= len { wi = 311; break 'outer; }
                                     *start.offset(si) |= 16; si += prime_ * 4 + 3;
                                     break 'label312
                                    }
                                    if si >= len { wi = 312; break 'outer; }
                                    *start.offset(si) |= 32; si += prime_ * 6 + 5;
                                    break 'label313
                                   }
                                   if si >= len { wi = 313; break 'outer; }
                                   *start.offset(si) |= 2; si += prime_ * 2 + 1;
                                   break 'label314
                                  }
                                  if si >= len { wi = 314; break 'outer; }
                                  *start.offset(si) |= 64; si += prime_ * 6 + 5;
                                  break 'label315
                                 }
                                 if si >= len { wi = 315; break 'outer; }
                                 *start.offset(si) |= 4; si += prime_ * 4 + 3;
                                 break 'label316
                                }
                                if si >= len { wi = 316; break 'outer; }
                                *start.offset(si) |= 8; si += prime_ * 2 + 1;
                                break 'label317
                               }
                               if si >= len { wi = 317; break 'outer; }
                               *start.offset(si) |= 128; si += prime_ * 4 + 4;
                               break 'label318
                              }
                              if si >= len { wi = 318; break 'outer; }
                              *start.offset(si) |= 1; si += prime_ * 2 + 1;
                              break 'label319
                             }
                             if si >= len { wi = 319; break 'outer; }
                             *start.offset(si) |= 16; si += prime_ * 4 + 3;
                             break 'label320
                            }
                            if si >= len { wi = 320; break 'outer; }
                            *start.offset(si) |= 32; si += prime_ * 6 + 5;
                            break 'label321
                           }
                           if si >= len { wi = 321; break 'outer; }
                           *start.offset(si) |= 2; si += prime_ * 2 + 1;
                           break 'label322
                          }
                          if si >= len { wi = 322; break 'outer; }
                          *start.offset(si) |= 64; si += prime_ * 6 + 5;
                          break 'label323
                         }
                         if si >= len { wi = 323; break 'outer; }
                         *start.offset(si) |= 4; si += prime_ * 4 + 3;
                         break 'label324
                        }
                        if si >= len { wi = 324; break 'outer; }
                        *start.offset(si) |= 8; si += prime_ * 2 + 1;
                        break 'label325
                       }
                       if si >= len { wi = 325; break 'outer; }
                       *start.offset(si) |= 128; si += prime_ * 4 + 4;
                       break 'label326
                      }
                      if si >= len { wi = 326; break 'outer; }
                      *start.offset(si) |= 1; si += prime_ * 2 + 1;
                      break 'label327
                     }
                     if si >= len { wi = 327; break 'outer; }
                     *start.offset(si) |= 16; si += prime_ * 4 + 3;
                     break 'label328
                    }
                    if si >= len { wi = 328; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 6 + 5;
                    break 'label329
                   }
                   if si >= len { wi = 329; break 'outer; }
                   *start.offset(si) |= 2; si += prime_ * 2 + 1;
                   break 'label330
                  }
                  if si >= len { wi = 330; break 'outer; }
                  *start.offset(si) |= 64; si += prime_ * 6 + 5;
                  break 'label331
                 }
                 if si >= len { wi = 331; break 'outer; }
                 *start.offset(si) |= 4; si += prime_ * 4 + 3;
                 break 'label332
                }
                if si >= len { wi = 332; break 'outer; }
                *start.offset(si) |= 8; si += prime_ * 2 + 1;
                break 'label333
               }
               if si >= len { wi = 333; break 'outer; }
               *start.offset(si) |= 128; si += prime_ * 4 + 4;
               break 'label334
              }
              if si >= len { wi = 334; break 'outer; }
              *start.offset(si) |= 1; si += prime_ * 2 + 1;
              break 'label335
             }
             if si >= len { wi = 335; break 'outer; }
             *start.offset(si) |= 16; si += prime_ * 4 + 3;
             wi = 288
            }
        }
        336...383 => { // 30 * x + 29
            loop {
             'label383: loop {
              'label382: loop {
               'label381: loop {
                'label380: loop {
                 'label379: loop {
                  'label378: loop {
                   'label377: loop {
                    'label376: loop {
                     'label375: loop {
                      'label374: loop {
                       'label373: loop {
                        'label372: loop {
                         'label371: loop {
                          'label370: loop {
                           'label369: loop {
                            'label368: loop {
                             'label367: loop {
                              'label366: loop {
                               'label365: loop {
                                'label364: loop {
                                 'label363: loop {
                                  'label362: loop {
                                   'label361: loop {
                                    'label360: loop {
                                     'label359: loop {
                                      'label358: loop {
                                       'label357: loop {
                                        'label356: loop {
                                         'label355: loop {
                                          'label354: loop {
                                           'label353: loop {
                                            'label352: loop {
                                             'label351: loop {
                                              'label350: loop {
                                               'label349: loop {
                                                'label348: loop {
                                                 'label347: loop {
                                                  'label346: loop {
                                                   'label345: loop {
                                                    'label344: loop {
                                                     'label343: loop {
                                                      'label342: loop {
                                                       'label341: loop {
                                                        'label340: loop {
                                                         'label339: loop {
                                                          'label338: loop {
                                                           'label337: loop {
                                                            'label336: loop {
                                                             match wi {
                                                              336 => break 'label336,
                                                              337 => break 'label337,
                                                              338 => break 'label338,
                                                              339 => break 'label339,
                                                              340 => break 'label340,
                                                              341 => break 'label341,
                                                              342 => break 'label342,
                                                              343 => break 'label343,
                                                              344 => break 'label344,
                                                              345 => break 'label345,
                                                              346 => break 'label346,
                                                              347 => break 'label347,
                                                              348 => break 'label348,
                                                              349 => break 'label349,
                                                              350 => break 'label350,
                                                              351 => break 'label351,
                                                              352 => break 'label352,
                                                              353 => break 'label353,
                                                              354 => break 'label354,
                                                              355 => break 'label355,
                                                              356 => break 'label356,
                                                              357 => break 'label357,
                                                              358 => break 'label358,
                                                              359 => break 'label359,
                                                              360 => break 'label360,
                                                              361 => break 'label361,
                                                              362 => break 'label362,
                                                              363 => break 'label363,
                                                              364 => break 'label364,
                                                              365 => break 'label365,
                                                              366 => break 'label366,
                                                              367 => break 'label367,
                                                              368 => break 'label368,
                                                              369 => break 'label369,
                                                              370 => break 'label370,
                                                              371 => break 'label371,
                                                              372 => break 'label372,
                                                              373 => break 'label373,
                                                              374 => break 'label374,
                                                              375 => break 'label375,
                                                              376 => break 'label376,
                                                              377 => break 'label377,
                                                              378 => break 'label378,
                                                              379 => break 'label379,
                                                              380 => break 'label380,
                                                              381 => break 'label381,
                                                              382 => break 'label382,
                                                              _ => break 'label383,
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
                                                                *start.offset(si + prime_ * 30 + 29) |= 1;
                                                                *start.offset(si + prime_ * 32 + 30) |= 128;
                                                                *start.offset(si + prime_ * 38 + 36) |= 64;
                                                                *start.offset(si + prime_ * 42 + 40) |= 32;
                                                                *start.offset(si + prime_ * 44 + 42) |= 16;
                                                                *start.offset(si + prime_ * 48 + 46) |= 8;
                                                                *start.offset(si + prime_ * 50 + 48) |= 4;
                                                                *start.offset(si + prime_ * 54 + 52) |= 2;
                                                                *start.offset(si + prime_ * 60 + 58) |= 1;
                                                                *start.offset(si + prime_ * 62 + 59) |= 128;
                                                                *start.offset(si + prime_ * 68 + 65) |= 64;
                                                                *start.offset(si + prime_ * 72 + 69) |= 32;
                                                                *start.offset(si + prime_ * 74 + 71) |= 16;
                                                                *start.offset(si + prime_ * 78 + 75) |= 8;
                                                                *start.offset(si + prime_ * 80 + 77) |= 4;
                                                                *start.offset(si + prime_ * 84 + 81) |= 2;
                                                                *start.offset(si + prime_ * 90 + 87) |= 1;
                                                                *start.offset(si + prime_ * 92 + 88) |= 128;
                                                                *start.offset(si + prime_ * 98 + 94) |= 64;
                                                                *start.offset(si + prime_ * 102 + 98) |= 32;
                                                                *start.offset(si + prime_ * 104 + 100) |= 16;
                                                                *start.offset(si + prime_ * 108 + 104) |= 8;
                                                                *start.offset(si + prime_ * 110 + 106) |= 4;
                                                                *start.offset(si + prime_ * 114 + 110) |= 2;
                                                                *start.offset(si + prime_ * 120 + 116) |= 1;
                                                                *start.offset(si + prime_ * 122 + 117) |= 128;
                                                                *start.offset(si + prime_ * 128 + 123) |= 64;
                                                                *start.offset(si + prime_ * 132 + 127) |= 32;
                                                                *start.offset(si + prime_ * 134 + 129) |= 16;
                                                                *start.offset(si + prime_ * 138 + 133) |= 8;
                                                                *start.offset(si + prime_ * 140 + 135) |= 4;
                                                                *start.offset(si + prime_ * 144 + 139) |= 2;
                                                                *start.offset(si + prime_ * 150 + 145) |= 1;
                                                                *start.offset(si + prime_ * 152 + 146) |= 128;
                                                                *start.offset(si + prime_ * 158 + 152) |= 64;
                                                                *start.offset(si + prime_ * 162 + 156) |= 32;
                                                                *start.offset(si + prime_ * 164 + 158) |= 16;
                                                                *start.offset(si + prime_ * 168 + 162) |= 8;
                                                                *start.offset(si + prime_ * 170 + 164) |= 4;
                                                                *start.offset(si + prime_ * 174 + 168) |= 2;

                                                                si += prime_ * 180 + 174
                                                            }
                                                            if si >= len { wi = 336; break 'outer; }
                                                            *start.offset(si) |= 1; si += prime_ * 2 + 1;
                                                            break 'label337
                                                           }
                                                           if si >= len { wi = 337; break 'outer; }
                                                           *start.offset(si) |= 128; si += prime_ * 6 + 6;
                                                           break 'label338
                                                          }
                                                          if si >= len { wi = 338; break 'outer; }
                                                          *start.offset(si) |= 64; si += prime_ * 4 + 4;
                                                          break 'label339
                                                         }
                                                         if si >= len { wi = 339; break 'outer; }
                                                         *start.offset(si) |= 32; si += prime_ * 2 + 2;
                                                         break 'label340
                                                        }
                                                        if si >= len { wi = 340; break 'outer; }
                                                        *start.offset(si) |= 16; si += prime_ * 4 + 4;
                                                        break 'label341
                                                       }
                                                       if si >= len { wi = 341; break 'outer; }
                                                       *start.offset(si) |= 8; si += prime_ * 2 + 2;
                                                       break 'label342
                                                      }
                                                      if si >= len { wi = 342; break 'outer; }
                                                      *start.offset(si) |= 4; si += prime_ * 4 + 4;
                                                      break 'label343
                                                     }
                                                     if si >= len { wi = 343; break 'outer; }
                                                     *start.offset(si) |= 2; si += prime_ * 6 + 6;
                                                     break 'label344
                                                    }
                                                    if si >= len { wi = 344; break 'outer; }
                                                    *start.offset(si) |= 1; si += prime_ * 2 + 1;
                                                    break 'label345
                                                   }
                                                   if si >= len { wi = 345; break 'outer; }
                                                   *start.offset(si) |= 128; si += prime_ * 6 + 6;
                                                   break 'label346
                                                  }
                                                  if si >= len { wi = 346; break 'outer; }
                                                  *start.offset(si) |= 64; si += prime_ * 4 + 4;
                                                  break 'label347
                                                 }
                                                 if si >= len { wi = 347; break 'outer; }
                                                 *start.offset(si) |= 32; si += prime_ * 2 + 2;
                                                 break 'label348
                                                }
                                                if si >= len { wi = 348; break 'outer; }
                                                *start.offset(si) |= 16; si += prime_ * 4 + 4;
                                                break 'label349
                                               }
                                               if si >= len { wi = 349; break 'outer; }
                                               *start.offset(si) |= 8; si += prime_ * 2 + 2;
                                               break 'label350
                                              }
                                              if si >= len { wi = 350; break 'outer; }
                                              *start.offset(si) |= 4; si += prime_ * 4 + 4;
                                              break 'label351
                                             }
                                             if si >= len { wi = 351; break 'outer; }
                                             *start.offset(si) |= 2; si += prime_ * 6 + 6;
                                             break 'label352
                                            }
                                            if si >= len { wi = 352; break 'outer; }
                                            *start.offset(si) |= 1; si += prime_ * 2 + 1;
                                            break 'label353
                                           }
                                           if si >= len { wi = 353; break 'outer; }
                                           *start.offset(si) |= 128; si += prime_ * 6 + 6;
                                           break 'label354
                                          }
                                          if si >= len { wi = 354; break 'outer; }
                                          *start.offset(si) |= 64; si += prime_ * 4 + 4;
                                          break 'label355
                                         }
                                         if si >= len { wi = 355; break 'outer; }
                                         *start.offset(si) |= 32; si += prime_ * 2 + 2;
                                         break 'label356
                                        }
                                        if si >= len { wi = 356; break 'outer; }
                                        *start.offset(si) |= 16; si += prime_ * 4 + 4;
                                        break 'label357
                                       }
                                       if si >= len { wi = 357; break 'outer; }
                                       *start.offset(si) |= 8; si += prime_ * 2 + 2;
                                       break 'label358
                                      }
                                      if si >= len { wi = 358; break 'outer; }
                                      *start.offset(si) |= 4; si += prime_ * 4 + 4;
                                      break 'label359
                                     }
                                     if si >= len { wi = 359; break 'outer; }
                                     *start.offset(si) |= 2; si += prime_ * 6 + 6;
                                     break 'label360
                                    }
                                    if si >= len { wi = 360; break 'outer; }
                                    *start.offset(si) |= 1; si += prime_ * 2 + 1;
                                    break 'label361
                                   }
                                   if si >= len { wi = 361; break 'outer; }
                                   *start.offset(si) |= 128; si += prime_ * 6 + 6;
                                   break 'label362
                                  }
                                  if si >= len { wi = 362; break 'outer; }
                                  *start.offset(si) |= 64; si += prime_ * 4 + 4;
                                  break 'label363
                                 }
                                 if si >= len { wi = 363; break 'outer; }
                                 *start.offset(si) |= 32; si += prime_ * 2 + 2;
                                 break 'label364
                                }
                                if si >= len { wi = 364; break 'outer; }
                                *start.offset(si) |= 16; si += prime_ * 4 + 4;
                                break 'label365
                               }
                               if si >= len { wi = 365; break 'outer; }
                               *start.offset(si) |= 8; si += prime_ * 2 + 2;
                               break 'label366
                              }
                              if si >= len { wi = 366; break 'outer; }
                              *start.offset(si) |= 4; si += prime_ * 4 + 4;
                              break 'label367
                             }
                             if si >= len { wi = 367; break 'outer; }
                             *start.offset(si) |= 2; si += prime_ * 6 + 6;
                             break 'label368
                            }
                            if si >= len { wi = 368; break 'outer; }
                            *start.offset(si) |= 1; si += prime_ * 2 + 1;
                            break 'label369
                           }
                           if si >= len { wi = 369; break 'outer; }
                           *start.offset(si) |= 128; si += prime_ * 6 + 6;
                           break 'label370
                          }
                          if si >= len { wi = 370; break 'outer; }
                          *start.offset(si) |= 64; si += prime_ * 4 + 4;
                          break 'label371
                         }
                         if si >= len { wi = 371; break 'outer; }
                         *start.offset(si) |= 32; si += prime_ * 2 + 2;
                         break 'label372
                        }
                        if si >= len { wi = 372; break 'outer; }
                        *start.offset(si) |= 16; si += prime_ * 4 + 4;
                        break 'label373
                       }
                       if si >= len { wi = 373; break 'outer; }
                       *start.offset(si) |= 8; si += prime_ * 2 + 2;
                       break 'label374
                      }
                      if si >= len { wi = 374; break 'outer; }
                      *start.offset(si) |= 4; si += prime_ * 4 + 4;
                      break 'label375
                     }
                     if si >= len { wi = 375; break 'outer; }
                     *start.offset(si) |= 2; si += prime_ * 6 + 6;
                     break 'label376
                    }
                    if si >= len { wi = 376; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 1;
                    break 'label377
                   }
                   if si >= len { wi = 377; break 'outer; }
                   *start.offset(si) |= 128; si += prime_ * 6 + 6;
                   break 'label378
                  }
                  if si >= len { wi = 378; break 'outer; }
                  *start.offset(si) |= 64; si += prime_ * 4 + 4;
                  break 'label379
                 }
                 if si >= len { wi = 379; break 'outer; }
                 *start.offset(si) |= 32; si += prime_ * 2 + 2;
                 break 'label380
                }
                if si >= len { wi = 380; break 'outer; }
                *start.offset(si) |= 16; si += prime_ * 4 + 4;
                break 'label381
               }
               if si >= len { wi = 381; break 'outer; }
               *start.offset(si) |= 8; si += prime_ * 2 + 2;
               break 'label382
              }
              if si >= len { wi = 382; break 'outer; }
              *start.offset(si) |= 4; si += prime_ * 4 + 4;
              break 'label383
             }
             if si >= len { wi = 383; break 'outer; }
             *start.offset(si) |= 2; si += prime_ * 6 + 6;
             wi = 336
            }
        }
        _ => unreachable!("{}", wi),
    }
    }
    *si_ = (si as usize).wrapping_sub(bytes.len());
    *wi_ = wi;
}

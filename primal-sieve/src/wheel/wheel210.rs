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
                if wi <= 0 {
                    if si >= len { wi = 0; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 6 + 0;
                }
                if wi <= 1 {
                    if si >= len { wi = 1; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 0;
                }
                if wi <= 2 {
                    if si >= len { wi = 2; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 0;
                }
                if wi <= 3 {
                    if si >= len { wi = 3; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 0;
                }
                if wi <= 4 {
                    if si >= len { wi = 4; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 0;
                }
                if wi <= 5 {
                    if si >= len { wi = 5; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 0;
                }
                if wi <= 6 {
                    if si >= len { wi = 6; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 6 + 0;
                }
                if wi <= 7 {
                    if si >= len { wi = 7; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 1;
                }
                if wi <= 8 {
                    if si >= len { wi = 8; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 6 + 0;
                }
                if wi <= 9 {
                    if si >= len { wi = 9; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 0;
                }
                if wi <= 10 {
                    if si >= len { wi = 10; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 0;
                }
                if wi <= 11 {
                    if si >= len { wi = 11; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 0;
                }
                if wi <= 12 {
                    if si >= len { wi = 12; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 0;
                }
                if wi <= 13 {
                    if si >= len { wi = 13; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 0;
                }
                if wi <= 14 {
                    if si >= len { wi = 14; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 6 + 0;
                }
                if wi <= 15 {
                    if si >= len { wi = 15; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 1;
                }
                if wi <= 16 {
                    if si >= len { wi = 16; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 6 + 0;
                }
                if wi <= 17 {
                    if si >= len { wi = 17; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 0;
                }
                if wi <= 18 {
                    if si >= len { wi = 18; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 0;
                }
                if wi <= 19 {
                    if si >= len { wi = 19; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 0;
                }
                if wi <= 20 {
                    if si >= len { wi = 20; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 0;
                }
                if wi <= 21 {
                    if si >= len { wi = 21; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 0;
                }
                if wi <= 22 {
                    if si >= len { wi = 22; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 6 + 0;
                }
                if wi <= 23 {
                    if si >= len { wi = 23; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 1;
                }
                if wi <= 24 {
                    if si >= len { wi = 24; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 6 + 0;
                }
                if wi <= 25 {
                    if si >= len { wi = 25; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 0;
                }
                if wi <= 26 {
                    if si >= len { wi = 26; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 0;
                }
                if wi <= 27 {
                    if si >= len { wi = 27; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 0;
                }
                if wi <= 28 {
                    if si >= len { wi = 28; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 0;
                }
                if wi <= 29 {
                    if si >= len { wi = 29; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 0;
                }
                if wi <= 30 {
                    if si >= len { wi = 30; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 6 + 0;
                }
                if wi <= 31 {
                    if si >= len { wi = 31; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 1;
                }
                if wi <= 32 {
                    if si >= len { wi = 32; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 6 + 0;
                }
                if wi <= 33 {
                    if si >= len { wi = 33; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 0;
                }
                if wi <= 34 {
                    if si >= len { wi = 34; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 0;
                }
                if wi <= 35 {
                    if si >= len { wi = 35; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 0;
                }
                if wi <= 36 {
                    if si >= len { wi = 36; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 0;
                }
                if wi <= 37 {
                    if si >= len { wi = 37; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 0;
                }
                if wi <= 38 {
                    if si >= len { wi = 38; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 6 + 0;
                }
                if wi <= 39 {
                    if si >= len { wi = 39; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 1;
                }
                if wi <= 40 {
                    if si >= len { wi = 40; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 6 + 0;
                }
                if wi <= 41 {
                    if si >= len { wi = 41; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 0;
                }
                if wi <= 42 {
                    if si >= len { wi = 42; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 0;
                }
                if wi <= 43 {
                    if si >= len { wi = 43; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 0;
                }
                if wi <= 44 {
                    if si >= len { wi = 44; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 0;
                }
                if wi <= 45 {
                    if si >= len { wi = 45; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 0;
                }
                if wi <= 46 {
                    if si >= len { wi = 46; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 6 + 0;
                }
                if wi <= 47 {
                    if si >= len { wi = 47; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 1;
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
                wi = 0
            }
        }
        48...95 => { // 30 * x + 7
            loop {
                if wi <= 48 {
                    if si >= len { wi = 48; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 1;
                }
                if wi <= 49 {
                    if si >= len { wi = 49; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 1;
                }
                if wi <= 50 {
                    if si >= len { wi = 50; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 0;
                }
                if wi <= 51 {
                    if si >= len { wi = 51; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 1;
                }
                if wi <= 52 {
                    if si >= len { wi = 52; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 1;
                }
                if wi <= 53 {
                    if si >= len { wi = 53; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 6 + 1;
                }
                if wi <= 54 {
                    if si >= len { wi = 54; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 1;
                }
                if wi <= 55 {
                    if si >= len { wi = 55; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 6 + 1;
                }
                if wi <= 56 {
                    if si >= len { wi = 56; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 1;
                }
                if wi <= 57 {
                    if si >= len { wi = 57; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 1;
                }
                if wi <= 58 {
                    if si >= len { wi = 58; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 0;
                }
                if wi <= 59 {
                    if si >= len { wi = 59; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 1;
                }
                if wi <= 60 {
                    if si >= len { wi = 60; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 1;
                }
                if wi <= 61 {
                    if si >= len { wi = 61; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 6 + 1;
                }
                if wi <= 62 {
                    if si >= len { wi = 62; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 1;
                }
                if wi <= 63 {
                    if si >= len { wi = 63; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 6 + 1;
                }
                if wi <= 64 {
                    if si >= len { wi = 64; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 1;
                }
                if wi <= 65 {
                    if si >= len { wi = 65; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 1;
                }
                if wi <= 66 {
                    if si >= len { wi = 66; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 0;
                }
                if wi <= 67 {
                    if si >= len { wi = 67; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 1;
                }
                if wi <= 68 {
                    if si >= len { wi = 68; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 1;
                }
                if wi <= 69 {
                    if si >= len { wi = 69; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 6 + 1;
                }
                if wi <= 70 {
                    if si >= len { wi = 70; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 1;
                }
                if wi <= 71 {
                    if si >= len { wi = 71; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 6 + 1;
                }
                if wi <= 72 {
                    if si >= len { wi = 72; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 1;
                }
                if wi <= 73 {
                    if si >= len { wi = 73; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 1;
                }
                if wi <= 74 {
                    if si >= len { wi = 74; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 0;
                }
                if wi <= 75 {
                    if si >= len { wi = 75; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 1;
                }
                if wi <= 76 {
                    if si >= len { wi = 76; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 1;
                }
                if wi <= 77 {
                    if si >= len { wi = 77; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 6 + 1;
                }
                if wi <= 78 {
                    if si >= len { wi = 78; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 1;
                }
                if wi <= 79 {
                    if si >= len { wi = 79; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 6 + 1;
                }
                if wi <= 80 {
                    if si >= len { wi = 80; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 1;
                }
                if wi <= 81 {
                    if si >= len { wi = 81; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 1;
                }
                if wi <= 82 {
                    if si >= len { wi = 82; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 0;
                }
                if wi <= 83 {
                    if si >= len { wi = 83; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 1;
                }
                if wi <= 84 {
                    if si >= len { wi = 84; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 1;
                }
                if wi <= 85 {
                    if si >= len { wi = 85; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 6 + 1;
                }
                if wi <= 86 {
                    if si >= len { wi = 86; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 1;
                }
                if wi <= 87 {
                    if si >= len { wi = 87; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 6 + 1;
                }
                if wi <= 88 {
                    if si >= len { wi = 88; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 1;
                }
                if wi <= 89 {
                    if si >= len { wi = 89; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 1;
                }
                if wi <= 90 {
                    if si >= len { wi = 90; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 0;
                }
                if wi <= 91 {
                    if si >= len { wi = 91; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 1;
                }
                if wi <= 92 {
                    if si >= len { wi = 92; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 1;
                }
                if wi <= 93 {
                    if si >= len { wi = 93; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 6 + 1;
                }
                if wi <= 94 {
                    if si >= len { wi = 94; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 1;
                }
                if wi <= 95 {
                    if si >= len { wi = 95; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 6 + 1;
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
                wi = 48
            }
        }
        96...143 => { // 30 * x + 11
            loop {
                if wi <= 96 {
                    if si >= len { wi = 96; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 0;
                }
                if wi <= 97 {
                    if si >= len { wi = 97; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 2;
                }
                if wi <= 98 {
                    if si >= len { wi = 98; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 0;
                }
                if wi <= 99 {
                    if si >= len { wi = 99; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 2;
                }
                if wi <= 100 {
                    if si >= len { wi = 100; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 6 + 2;
                }
                if wi <= 101 {
                    if si >= len { wi = 101; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 1;
                }
                if wi <= 102 {
                    if si >= len { wi = 102; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 6 + 2;
                }
                if wi <= 103 {
                    if si >= len { wi = 103; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 2;
                }
                if wi <= 104 {
                    if si >= len { wi = 104; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 0;
                }
                if wi <= 105 {
                    if si >= len { wi = 105; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 2;
                }
                if wi <= 106 {
                    if si >= len { wi = 106; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 0;
                }
                if wi <= 107 {
                    if si >= len { wi = 107; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 2;
                }
                if wi <= 108 {
                    if si >= len { wi = 108; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 6 + 2;
                }
                if wi <= 109 {
                    if si >= len { wi = 109; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 1;
                }
                if wi <= 110 {
                    if si >= len { wi = 110; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 6 + 2;
                }
                if wi <= 111 {
                    if si >= len { wi = 111; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 2;
                }
                if wi <= 112 {
                    if si >= len { wi = 112; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 0;
                }
                if wi <= 113 {
                    if si >= len { wi = 113; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 2;
                }
                if wi <= 114 {
                    if si >= len { wi = 114; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 0;
                }
                if wi <= 115 {
                    if si >= len { wi = 115; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 2;
                }
                if wi <= 116 {
                    if si >= len { wi = 116; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 6 + 2;
                }
                if wi <= 117 {
                    if si >= len { wi = 117; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 1;
                }
                if wi <= 118 {
                    if si >= len { wi = 118; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 6 + 2;
                }
                if wi <= 119 {
                    if si >= len { wi = 119; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 2;
                }
                if wi <= 120 {
                    if si >= len { wi = 120; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 0;
                }
                if wi <= 121 {
                    if si >= len { wi = 121; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 2;
                }
                if wi <= 122 {
                    if si >= len { wi = 122; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 0;
                }
                if wi <= 123 {
                    if si >= len { wi = 123; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 2;
                }
                if wi <= 124 {
                    if si >= len { wi = 124; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 6 + 2;
                }
                if wi <= 125 {
                    if si >= len { wi = 125; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 1;
                }
                if wi <= 126 {
                    if si >= len { wi = 126; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 6 + 2;
                }
                if wi <= 127 {
                    if si >= len { wi = 127; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 2;
                }
                if wi <= 128 {
                    if si >= len { wi = 128; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 0;
                }
                if wi <= 129 {
                    if si >= len { wi = 129; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 2;
                }
                if wi <= 130 {
                    if si >= len { wi = 130; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 0;
                }
                if wi <= 131 {
                    if si >= len { wi = 131; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 2;
                }
                if wi <= 132 {
                    if si >= len { wi = 132; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 6 + 2;
                }
                if wi <= 133 {
                    if si >= len { wi = 133; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 1;
                }
                if wi <= 134 {
                    if si >= len { wi = 134; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 6 + 2;
                }
                if wi <= 135 {
                    if si >= len { wi = 135; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 2;
                }
                if wi <= 136 {
                    if si >= len { wi = 136; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 0;
                }
                if wi <= 137 {
                    if si >= len { wi = 137; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 2;
                }
                if wi <= 138 {
                    if si >= len { wi = 138; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 0;
                }
                if wi <= 139 {
                    if si >= len { wi = 139; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 2;
                }
                if wi <= 140 {
                    if si >= len { wi = 140; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 6 + 2;
                }
                if wi <= 141 {
                    if si >= len { wi = 141; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 1;
                }
                if wi <= 142 {
                    if si >= len { wi = 142; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 6 + 2;
                }
                if wi <= 143 {
                    if si >= len { wi = 143; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 2;
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
                wi = 96
            }
        }
        144...191 => { // 30 * x + 13
            loop {
                if wi <= 144 {
                    if si >= len { wi = 144; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 2;
                }
                if wi <= 145 {
                    if si >= len { wi = 145; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 1;
                }
                if wi <= 146 {
                    if si >= len { wi = 146; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 1;
                }
                if wi <= 147 {
                    if si >= len { wi = 147; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 6 + 3;
                }
                if wi <= 148 {
                    if si >= len { wi = 148; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 1;
                }
                if wi <= 149 {
                    if si >= len { wi = 149; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 6 + 3;
                }
                if wi <= 150 {
                    if si >= len { wi = 150; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 1;
                }
                if wi <= 151 {
                    if si >= len { wi = 151; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 1;
                }
                if wi <= 152 {
                    if si >= len { wi = 152; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 2;
                }
                if wi <= 153 {
                    if si >= len { wi = 153; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 1;
                }
                if wi <= 154 {
                    if si >= len { wi = 154; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 1;
                }
                if wi <= 155 {
                    if si >= len { wi = 155; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 6 + 3;
                }
                if wi <= 156 {
                    if si >= len { wi = 156; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 1;
                }
                if wi <= 157 {
                    if si >= len { wi = 157; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 6 + 3;
                }
                if wi <= 158 {
                    if si >= len { wi = 158; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 1;
                }
                if wi <= 159 {
                    if si >= len { wi = 159; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 1;
                }
                if wi <= 160 {
                    if si >= len { wi = 160; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 2;
                }
                if wi <= 161 {
                    if si >= len { wi = 161; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 1;
                }
                if wi <= 162 {
                    if si >= len { wi = 162; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 1;
                }
                if wi <= 163 {
                    if si >= len { wi = 163; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 6 + 3;
                }
                if wi <= 164 {
                    if si >= len { wi = 164; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 1;
                }
                if wi <= 165 {
                    if si >= len { wi = 165; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 6 + 3;
                }
                if wi <= 166 {
                    if si >= len { wi = 166; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 1;
                }
                if wi <= 167 {
                    if si >= len { wi = 167; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 1;
                }
                if wi <= 168 {
                    if si >= len { wi = 168; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 2;
                }
                if wi <= 169 {
                    if si >= len { wi = 169; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 1;
                }
                if wi <= 170 {
                    if si >= len { wi = 170; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 1;
                }
                if wi <= 171 {
                    if si >= len { wi = 171; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 6 + 3;
                }
                if wi <= 172 {
                    if si >= len { wi = 172; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 1;
                }
                if wi <= 173 {
                    if si >= len { wi = 173; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 6 + 3;
                }
                if wi <= 174 {
                    if si >= len { wi = 174; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 1;
                }
                if wi <= 175 {
                    if si >= len { wi = 175; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 1;
                }
                if wi <= 176 {
                    if si >= len { wi = 176; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 2;
                }
                if wi <= 177 {
                    if si >= len { wi = 177; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 1;
                }
                if wi <= 178 {
                    if si >= len { wi = 178; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 1;
                }
                if wi <= 179 {
                    if si >= len { wi = 179; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 6 + 3;
                }
                if wi <= 180 {
                    if si >= len { wi = 180; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 1;
                }
                if wi <= 181 {
                    if si >= len { wi = 181; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 6 + 3;
                }
                if wi <= 182 {
                    if si >= len { wi = 182; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 1;
                }
                if wi <= 183 {
                    if si >= len { wi = 183; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 1;
                }
                if wi <= 184 {
                    if si >= len { wi = 184; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 4 + 2;
                }
                if wi <= 185 {
                    if si >= len { wi = 185; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 1;
                }
                if wi <= 186 {
                    if si >= len { wi = 186; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 1;
                }
                if wi <= 187 {
                    if si >= len { wi = 187; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 6 + 3;
                }
                if wi <= 188 {
                    if si >= len { wi = 188; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 2 + 1;
                }
                if wi <= 189 {
                    if si >= len { wi = 189; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 6 + 3;
                }
                if wi <= 190 {
                    if si >= len { wi = 190; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 1;
                }
                if wi <= 191 {
                    if si >= len { wi = 191; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 1;
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
                wi = 144
            }
        }
        192...239 => { // 30 * x + 17
            loop {
                if wi <= 192 {
                    if si >= len { wi = 192; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 1;
                }
                if wi <= 193 {
                    if si >= len { wi = 193; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 3;
                }
                if wi <= 194 {
                    if si >= len { wi = 194; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 6 + 3;
                }
                if wi <= 195 {
                    if si >= len { wi = 195; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 1;
                }
                if wi <= 196 {
                    if si >= len { wi = 196; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 6 + 3;
                }
                if wi <= 197 {
                    if si >= len { wi = 197; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 3;
                }
                if wi <= 198 {
                    if si >= len { wi = 198; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 1;
                }
                if wi <= 199 {
                    if si >= len { wi = 199; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 2;
                }
                if wi <= 200 {
                    if si >= len { wi = 200; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 1;
                }
                if wi <= 201 {
                    if si >= len { wi = 201; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 3;
                }
                if wi <= 202 {
                    if si >= len { wi = 202; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 6 + 3;
                }
                if wi <= 203 {
                    if si >= len { wi = 203; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 1;
                }
                if wi <= 204 {
                    if si >= len { wi = 204; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 6 + 3;
                }
                if wi <= 205 {
                    if si >= len { wi = 205; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 3;
                }
                if wi <= 206 {
                    if si >= len { wi = 206; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 1;
                }
                if wi <= 207 {
                    if si >= len { wi = 207; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 2;
                }
                if wi <= 208 {
                    if si >= len { wi = 208; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 1;
                }
                if wi <= 209 {
                    if si >= len { wi = 209; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 3;
                }
                if wi <= 210 {
                    if si >= len { wi = 210; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 6 + 3;
                }
                if wi <= 211 {
                    if si >= len { wi = 211; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 1;
                }
                if wi <= 212 {
                    if si >= len { wi = 212; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 6 + 3;
                }
                if wi <= 213 {
                    if si >= len { wi = 213; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 3;
                }
                if wi <= 214 {
                    if si >= len { wi = 214; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 1;
                }
                if wi <= 215 {
                    if si >= len { wi = 215; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 2;
                }
                if wi <= 216 {
                    if si >= len { wi = 216; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 1;
                }
                if wi <= 217 {
                    if si >= len { wi = 217; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 3;
                }
                if wi <= 218 {
                    if si >= len { wi = 218; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 6 + 3;
                }
                if wi <= 219 {
                    if si >= len { wi = 219; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 1;
                }
                if wi <= 220 {
                    if si >= len { wi = 220; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 6 + 3;
                }
                if wi <= 221 {
                    if si >= len { wi = 221; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 3;
                }
                if wi <= 222 {
                    if si >= len { wi = 222; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 1;
                }
                if wi <= 223 {
                    if si >= len { wi = 223; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 2;
                }
                if wi <= 224 {
                    if si >= len { wi = 224; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 1;
                }
                if wi <= 225 {
                    if si >= len { wi = 225; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 3;
                }
                if wi <= 226 {
                    if si >= len { wi = 226; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 6 + 3;
                }
                if wi <= 227 {
                    if si >= len { wi = 227; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 1;
                }
                if wi <= 228 {
                    if si >= len { wi = 228; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 6 + 3;
                }
                if wi <= 229 {
                    if si >= len { wi = 229; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 3;
                }
                if wi <= 230 {
                    if si >= len { wi = 230; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 1;
                }
                if wi <= 231 {
                    if si >= len { wi = 231; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 2;
                }
                if wi <= 232 {
                    if si >= len { wi = 232; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 1;
                }
                if wi <= 233 {
                    if si >= len { wi = 233; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 3;
                }
                if wi <= 234 {
                    if si >= len { wi = 234; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 6 + 3;
                }
                if wi <= 235 {
                    if si >= len { wi = 235; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 1;
                }
                if wi <= 236 {
                    if si >= len { wi = 236; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 6 + 3;
                }
                if wi <= 237 {
                    if si >= len { wi = 237; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 3;
                }
                if wi <= 238 {
                    if si >= len { wi = 238; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 1;
                }
                if wi <= 239 {
                    if si >= len { wi = 239; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 2;
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
                wi = 192
            }
        }
        240...287 => { // 30 * x + 19
            loop {
                if wi <= 240 {
                    if si >= len { wi = 240; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 2;
                }
                if wi <= 241 {
                    if si >= len { wi = 241; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 6 + 4;
                }
                if wi <= 242 {
                    if si >= len { wi = 242; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 1;
                }
                if wi <= 243 {
                    if si >= len { wi = 243; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 6 + 4;
                }
                if wi <= 244 {
                    if si >= len { wi = 244; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 2;
                }
                if wi <= 245 {
                    if si >= len { wi = 245; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 2;
                }
                if wi <= 246 {
                    if si >= len { wi = 246; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 2;
                }
                if wi <= 247 {
                    if si >= len { wi = 247; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 2;
                }
                if wi <= 248 {
                    if si >= len { wi = 248; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 2;
                }
                if wi <= 249 {
                    if si >= len { wi = 249; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 6 + 4;
                }
                if wi <= 250 {
                    if si >= len { wi = 250; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 1;
                }
                if wi <= 251 {
                    if si >= len { wi = 251; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 6 + 4;
                }
                if wi <= 252 {
                    if si >= len { wi = 252; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 2;
                }
                if wi <= 253 {
                    if si >= len { wi = 253; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 2;
                }
                if wi <= 254 {
                    if si >= len { wi = 254; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 2;
                }
                if wi <= 255 {
                    if si >= len { wi = 255; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 2;
                }
                if wi <= 256 {
                    if si >= len { wi = 256; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 2;
                }
                if wi <= 257 {
                    if si >= len { wi = 257; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 6 + 4;
                }
                if wi <= 258 {
                    if si >= len { wi = 258; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 1;
                }
                if wi <= 259 {
                    if si >= len { wi = 259; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 6 + 4;
                }
                if wi <= 260 {
                    if si >= len { wi = 260; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 2;
                }
                if wi <= 261 {
                    if si >= len { wi = 261; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 2;
                }
                if wi <= 262 {
                    if si >= len { wi = 262; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 2;
                }
                if wi <= 263 {
                    if si >= len { wi = 263; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 2;
                }
                if wi <= 264 {
                    if si >= len { wi = 264; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 2;
                }
                if wi <= 265 {
                    if si >= len { wi = 265; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 6 + 4;
                }
                if wi <= 266 {
                    if si >= len { wi = 266; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 1;
                }
                if wi <= 267 {
                    if si >= len { wi = 267; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 6 + 4;
                }
                if wi <= 268 {
                    if si >= len { wi = 268; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 2;
                }
                if wi <= 269 {
                    if si >= len { wi = 269; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 2;
                }
                if wi <= 270 {
                    if si >= len { wi = 270; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 2;
                }
                if wi <= 271 {
                    if si >= len { wi = 271; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 2;
                }
                if wi <= 272 {
                    if si >= len { wi = 272; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 2;
                }
                if wi <= 273 {
                    if si >= len { wi = 273; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 6 + 4;
                }
                if wi <= 274 {
                    if si >= len { wi = 274; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 1;
                }
                if wi <= 275 {
                    if si >= len { wi = 275; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 6 + 4;
                }
                if wi <= 276 {
                    if si >= len { wi = 276; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 2;
                }
                if wi <= 277 {
                    if si >= len { wi = 277; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 2;
                }
                if wi <= 278 {
                    if si >= len { wi = 278; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 2;
                }
                if wi <= 279 {
                    if si >= len { wi = 279; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 2;
                }
                if wi <= 280 {
                    if si >= len { wi = 280; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 4 + 2;
                }
                if wi <= 281 {
                    if si >= len { wi = 281; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 6 + 4;
                }
                if wi <= 282 {
                    if si >= len { wi = 282; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 2 + 1;
                }
                if wi <= 283 {
                    if si >= len { wi = 283; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 6 + 4;
                }
                if wi <= 284 {
                    if si >= len { wi = 284; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 4 + 2;
                }
                if wi <= 285 {
                    if si >= len { wi = 285; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 2 + 2;
                }
                if wi <= 286 {
                    if si >= len { wi = 286; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 4 + 2;
                }
                if wi <= 287 {
                    if si >= len { wi = 287; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 2 + 2;
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
                wi = 240
            }
        }
        288...335 => { // 30 * x + 23
            loop {
                if wi <= 288 {
                    if si >= len { wi = 288; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 6 + 5;
                }
                if wi <= 289 {
                    if si >= len { wi = 289; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 1;
                }
                if wi <= 290 {
                    if si >= len { wi = 290; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 6 + 5;
                }
                if wi <= 291 {
                    if si >= len { wi = 291; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 3;
                }
                if wi <= 292 {
                    if si >= len { wi = 292; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 1;
                }
                if wi <= 293 {
                    if si >= len { wi = 293; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 4;
                }
                if wi <= 294 {
                    if si >= len { wi = 294; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 1;
                }
                if wi <= 295 {
                    if si >= len { wi = 295; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 3;
                }
                if wi <= 296 {
                    if si >= len { wi = 296; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 6 + 5;
                }
                if wi <= 297 {
                    if si >= len { wi = 297; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 1;
                }
                if wi <= 298 {
                    if si >= len { wi = 298; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 6 + 5;
                }
                if wi <= 299 {
                    if si >= len { wi = 299; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 3;
                }
                if wi <= 300 {
                    if si >= len { wi = 300; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 1;
                }
                if wi <= 301 {
                    if si >= len { wi = 301; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 4;
                }
                if wi <= 302 {
                    if si >= len { wi = 302; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 1;
                }
                if wi <= 303 {
                    if si >= len { wi = 303; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 3;
                }
                if wi <= 304 {
                    if si >= len { wi = 304; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 6 + 5;
                }
                if wi <= 305 {
                    if si >= len { wi = 305; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 1;
                }
                if wi <= 306 {
                    if si >= len { wi = 306; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 6 + 5;
                }
                if wi <= 307 {
                    if si >= len { wi = 307; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 3;
                }
                if wi <= 308 {
                    if si >= len { wi = 308; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 1;
                }
                if wi <= 309 {
                    if si >= len { wi = 309; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 4;
                }
                if wi <= 310 {
                    if si >= len { wi = 310; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 1;
                }
                if wi <= 311 {
                    if si >= len { wi = 311; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 3;
                }
                if wi <= 312 {
                    if si >= len { wi = 312; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 6 + 5;
                }
                if wi <= 313 {
                    if si >= len { wi = 313; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 1;
                }
                if wi <= 314 {
                    if si >= len { wi = 314; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 6 + 5;
                }
                if wi <= 315 {
                    if si >= len { wi = 315; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 3;
                }
                if wi <= 316 {
                    if si >= len { wi = 316; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 1;
                }
                if wi <= 317 {
                    if si >= len { wi = 317; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 4;
                }
                if wi <= 318 {
                    if si >= len { wi = 318; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 1;
                }
                if wi <= 319 {
                    if si >= len { wi = 319; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 3;
                }
                if wi <= 320 {
                    if si >= len { wi = 320; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 6 + 5;
                }
                if wi <= 321 {
                    if si >= len { wi = 321; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 1;
                }
                if wi <= 322 {
                    if si >= len { wi = 322; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 6 + 5;
                }
                if wi <= 323 {
                    if si >= len { wi = 323; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 3;
                }
                if wi <= 324 {
                    if si >= len { wi = 324; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 1;
                }
                if wi <= 325 {
                    if si >= len { wi = 325; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 4;
                }
                if wi <= 326 {
                    if si >= len { wi = 326; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 1;
                }
                if wi <= 327 {
                    if si >= len { wi = 327; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 3;
                }
                if wi <= 328 {
                    if si >= len { wi = 328; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 6 + 5;
                }
                if wi <= 329 {
                    if si >= len { wi = 329; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 2 + 1;
                }
                if wi <= 330 {
                    if si >= len { wi = 330; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 6 + 5;
                }
                if wi <= 331 {
                    if si >= len { wi = 331; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 3;
                }
                if wi <= 332 {
                    if si >= len { wi = 332; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 1;
                }
                if wi <= 333 {
                    if si >= len { wi = 333; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 4 + 4;
                }
                if wi <= 334 {
                    if si >= len { wi = 334; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 1;
                }
                if wi <= 335 {
                    if si >= len { wi = 335; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 3;
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
                wi = 288
            }
        }
        336...383 => { // 30 * x + 29
            loop {
                if wi <= 336 {
                    if si >= len { wi = 336; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 1;
                }
                if wi <= 337 {
                    if si >= len { wi = 337; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 6 + 6;
                }
                if wi <= 338 {
                    if si >= len { wi = 338; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 4;
                }
                if wi <= 339 {
                    if si >= len { wi = 339; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 2;
                }
                if wi <= 340 {
                    if si >= len { wi = 340; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 4;
                }
                if wi <= 341 {
                    if si >= len { wi = 341; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 2;
                }
                if wi <= 342 {
                    if si >= len { wi = 342; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 4;
                }
                if wi <= 343 {
                    if si >= len { wi = 343; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 6 + 6;
                }
                if wi <= 344 {
                    if si >= len { wi = 344; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 1;
                }
                if wi <= 345 {
                    if si >= len { wi = 345; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 6 + 6;
                }
                if wi <= 346 {
                    if si >= len { wi = 346; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 4;
                }
                if wi <= 347 {
                    if si >= len { wi = 347; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 2;
                }
                if wi <= 348 {
                    if si >= len { wi = 348; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 4;
                }
                if wi <= 349 {
                    if si >= len { wi = 349; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 2;
                }
                if wi <= 350 {
                    if si >= len { wi = 350; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 4;
                }
                if wi <= 351 {
                    if si >= len { wi = 351; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 6 + 6;
                }
                if wi <= 352 {
                    if si >= len { wi = 352; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 1;
                }
                if wi <= 353 {
                    if si >= len { wi = 353; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 6 + 6;
                }
                if wi <= 354 {
                    if si >= len { wi = 354; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 4;
                }
                if wi <= 355 {
                    if si >= len { wi = 355; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 2;
                }
                if wi <= 356 {
                    if si >= len { wi = 356; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 4;
                }
                if wi <= 357 {
                    if si >= len { wi = 357; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 2;
                }
                if wi <= 358 {
                    if si >= len { wi = 358; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 4;
                }
                if wi <= 359 {
                    if si >= len { wi = 359; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 6 + 6;
                }
                if wi <= 360 {
                    if si >= len { wi = 360; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 1;
                }
                if wi <= 361 {
                    if si >= len { wi = 361; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 6 + 6;
                }
                if wi <= 362 {
                    if si >= len { wi = 362; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 4;
                }
                if wi <= 363 {
                    if si >= len { wi = 363; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 2;
                }
                if wi <= 364 {
                    if si >= len { wi = 364; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 4;
                }
                if wi <= 365 {
                    if si >= len { wi = 365; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 2;
                }
                if wi <= 366 {
                    if si >= len { wi = 366; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 4;
                }
                if wi <= 367 {
                    if si >= len { wi = 367; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 6 + 6;
                }
                if wi <= 368 {
                    if si >= len { wi = 368; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 1;
                }
                if wi <= 369 {
                    if si >= len { wi = 369; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 6 + 6;
                }
                if wi <= 370 {
                    if si >= len { wi = 370; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 4;
                }
                if wi <= 371 {
                    if si >= len { wi = 371; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 2;
                }
                if wi <= 372 {
                    if si >= len { wi = 372; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 4;
                }
                if wi <= 373 {
                    if si >= len { wi = 373; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 2;
                }
                if wi <= 374 {
                    if si >= len { wi = 374; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 4;
                }
                if wi <= 375 {
                    if si >= len { wi = 375; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 6 + 6;
                }
                if wi <= 376 {
                    if si >= len { wi = 376; break 'outer; }
                    *start.offset(si) |= 1; si += prime_ * 2 + 1;
                }
                if wi <= 377 {
                    if si >= len { wi = 377; break 'outer; }
                    *start.offset(si) |= 128; si += prime_ * 6 + 6;
                }
                if wi <= 378 {
                    if si >= len { wi = 378; break 'outer; }
                    *start.offset(si) |= 64; si += prime_ * 4 + 4;
                }
                if wi <= 379 {
                    if si >= len { wi = 379; break 'outer; }
                    *start.offset(si) |= 32; si += prime_ * 2 + 2;
                }
                if wi <= 380 {
                    if si >= len { wi = 380; break 'outer; }
                    *start.offset(si) |= 16; si += prime_ * 4 + 4;
                }
                if wi <= 381 {
                    if si >= len { wi = 381; break 'outer; }
                    *start.offset(si) |= 8; si += prime_ * 2 + 2;
                }
                if wi <= 382 {
                    if si >= len { wi = 382; break 'outer; }
                    *start.offset(si) |= 4; si += prime_ * 4 + 4;
                }
                if wi <= 383 {
                    if si >= len { wi = 383; break 'outer; }
                    *start.offset(si) |= 2; si += prime_ * 6 + 6;
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
                wi = 336
            }
        }
        _ => unreachable!("{}", wi),
    }
    break 'outer;
    }
    *si_ = (si as usize).wrapping_sub(bytes.len());
    *wi_ = wi;
}

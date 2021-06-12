#![no_std]
#![feature(core_intrinsics)]
#![feature(test)]
#![feature(stdsimd)]
#![feature(stmt_expr_attributes)]

use core::default::Default;

use core::arch::x86_64::*;

macro_rules! constify_imm8 {
    ($imm8:expr, $expand:ident) => {
        #[allow(overflowing_literals)]
        // Cannot use shorthad like x @ 0...254 => $expand!(x),
        // because some intrinsics expect u32 whereas others i32!
        match ($imm8) & 0b1111_1111 {
            0 => $expand!(0),
            1 => $expand!(1),
            2 => $expand!(2),
            3 => $expand!(3),
            4 => $expand!(4),
            5 => $expand!(5),
            6 => $expand!(6),
            7 => $expand!(7),
            8 => $expand!(8),
            9 => $expand!(9),
            10 => $expand!(10),
            11 => $expand!(11),
            12 => $expand!(12),
            13 => $expand!(13),
            14 => $expand!(14),
            15 => $expand!(15),
            16 => $expand!(16),
            17 => $expand!(17),
            18 => $expand!(18),
            19 => $expand!(19),
            20 => $expand!(20),
            21 => $expand!(21),
            22 => $expand!(22),
            23 => $expand!(23),
            24 => $expand!(24),
            25 => $expand!(25),
            26 => $expand!(26),
            27 => $expand!(27),
            28 => $expand!(28),
            29 => $expand!(29),
            30 => $expand!(30),
            31 => $expand!(31),
            32 => $expand!(32),
            33 => $expand!(33),
            34 => $expand!(34),
            35 => $expand!(35),
            36 => $expand!(36),
            37 => $expand!(37),
            38 => $expand!(38),
            39 => $expand!(39),
            40 => $expand!(40),
            41 => $expand!(41),
            42 => $expand!(42),
            43 => $expand!(43),
            44 => $expand!(44),
            45 => $expand!(45),
            46 => $expand!(46),
            47 => $expand!(47),
            48 => $expand!(48),
            49 => $expand!(49),
            50 => $expand!(50),
            51 => $expand!(51),
            52 => $expand!(52),
            53 => $expand!(53),
            54 => $expand!(54),
            55 => $expand!(55),
            56 => $expand!(56),
            57 => $expand!(57),
            58 => $expand!(58),
            59 => $expand!(59),
            60 => $expand!(60),
            61 => $expand!(61),
            62 => $expand!(62),
            63 => $expand!(63),
            64 => $expand!(64),
            65 => $expand!(65),
            66 => $expand!(66),
            67 => $expand!(67),
            68 => $expand!(68),
            69 => $expand!(69),
            70 => $expand!(70),
            71 => $expand!(71),
            72 => $expand!(72),
            73 => $expand!(73),
            74 => $expand!(74),
            75 => $expand!(75),
            76 => $expand!(76),
            77 => $expand!(77),
            78 => $expand!(78),
            79 => $expand!(79),
            80 => $expand!(80),
            81 => $expand!(81),
            82 => $expand!(82),
            83 => $expand!(83),
            84 => $expand!(84),
            85 => $expand!(85),
            86 => $expand!(86),
            87 => $expand!(87),
            88 => $expand!(88),
            89 => $expand!(89),
            90 => $expand!(90),
            91 => $expand!(91),
            92 => $expand!(92),
            93 => $expand!(93),
            94 => $expand!(94),
            95 => $expand!(95),
            96 => $expand!(96),
            97 => $expand!(97),
            98 => $expand!(98),
            99 => $expand!(99),
            100 => $expand!(100),
            101 => $expand!(101),
            102 => $expand!(102),
            103 => $expand!(103),
            104 => $expand!(104),
            105 => $expand!(105),
            106 => $expand!(106),
            107 => $expand!(107),
            108 => $expand!(108),
            109 => $expand!(109),
            110 => $expand!(110),
            111 => $expand!(111),
            112 => $expand!(112),
            113 => $expand!(113),
            114 => $expand!(114),
            115 => $expand!(115),
            116 => $expand!(116),
            117 => $expand!(117),
            118 => $expand!(118),
            119 => $expand!(119),
            120 => $expand!(120),
            121 => $expand!(121),
            122 => $expand!(122),
            123 => $expand!(123),
            124 => $expand!(124),
            125 => $expand!(125),
            126 => $expand!(126),
            127 => $expand!(127),
            128 => $expand!(128),
            129 => $expand!(129),
            130 => $expand!(130),
            131 => $expand!(131),
            132 => $expand!(132),
            133 => $expand!(133),
            134 => $expand!(134),
            135 => $expand!(135),
            136 => $expand!(136),
            137 => $expand!(137),
            138 => $expand!(138),
            139 => $expand!(139),
            140 => $expand!(140),
            141 => $expand!(141),
            142 => $expand!(142),
            143 => $expand!(143),
            144 => $expand!(144),
            145 => $expand!(145),
            146 => $expand!(146),
            147 => $expand!(147),
            148 => $expand!(148),
            149 => $expand!(149),
            150 => $expand!(150),
            151 => $expand!(151),
            152 => $expand!(152),
            153 => $expand!(153),
            154 => $expand!(154),
            155 => $expand!(155),
            156 => $expand!(156),
            157 => $expand!(157),
            158 => $expand!(158),
            159 => $expand!(159),
            160 => $expand!(160),
            161 => $expand!(161),
            162 => $expand!(162),
            163 => $expand!(163),
            164 => $expand!(164),
            165 => $expand!(165),
            166 => $expand!(166),
            167 => $expand!(167),
            168 => $expand!(168),
            169 => $expand!(169),
            170 => $expand!(170),
            171 => $expand!(171),
            172 => $expand!(172),
            173 => $expand!(173),
            174 => $expand!(174),
            175 => $expand!(175),
            176 => $expand!(176),
            177 => $expand!(177),
            178 => $expand!(178),
            179 => $expand!(179),
            180 => $expand!(180),
            181 => $expand!(181),
            182 => $expand!(182),
            183 => $expand!(183),
            184 => $expand!(184),
            185 => $expand!(185),
            186 => $expand!(186),
            187 => $expand!(187),
            188 => $expand!(188),
            189 => $expand!(189),
            190 => $expand!(190),
            191 => $expand!(191),
            192 => $expand!(192),
            193 => $expand!(193),
            194 => $expand!(194),
            195 => $expand!(195),
            196 => $expand!(196),
            197 => $expand!(197),
            198 => $expand!(198),
            199 => $expand!(199),
            200 => $expand!(200),
            201 => $expand!(201),
            202 => $expand!(202),
            203 => $expand!(203),
            204 => $expand!(204),
            205 => $expand!(205),
            206 => $expand!(206),
            207 => $expand!(207),
            208 => $expand!(208),
            209 => $expand!(209),
            210 => $expand!(210),
            211 => $expand!(211),
            212 => $expand!(212),
            213 => $expand!(213),
            214 => $expand!(214),
            215 => $expand!(215),
            216 => $expand!(216),
            217 => $expand!(217),
            218 => $expand!(218),
            219 => $expand!(219),
            220 => $expand!(220),
            221 => $expand!(221),
            222 => $expand!(222),
            223 => $expand!(223),
            224 => $expand!(224),
            225 => $expand!(225),
            226 => $expand!(226),
            227 => $expand!(227),
            228 => $expand!(228),
            229 => $expand!(229),
            230 => $expand!(230),
            231 => $expand!(231),
            232 => $expand!(232),
            233 => $expand!(233),
            234 => $expand!(234),
            235 => $expand!(235),
            236 => $expand!(236),
            237 => $expand!(237),
            238 => $expand!(238),
            239 => $expand!(239),
            240 => $expand!(240),
            241 => $expand!(241),
            242 => $expand!(242),
            243 => $expand!(243),
            244 => $expand!(244),
            245 => $expand!(245),
            246 => $expand!(246),
            247 => $expand!(247),
            248 => $expand!(248),
            249 => $expand!(249),
            250 => $expand!(250),
            251 => $expand!(251),
            252 => $expand!(252),
            253 => $expand!(253),
            254 => $expand!(254),
            _ => $expand!(255),
        }
    };
}

const S11: i32 = 7;
const S12: i32 = 12;
const S13: i32 = 17;
const S14: i32 = 22;
const S21: i32 = 5;
const S22: i32 = 9;
const S23: i32 = 14;
const S24: i32 = 20;
const S31: i32 = 4;
const S32: i32 = 11;
const S33: i32 = 16;
const S34: i32 = 23;
const S41: i32 = 6;
const S42: i32 = 10;
const S43: i32 = 15;
const S44: i32 = 21;

const PAD: [u8; 64] = [
    0x80, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

fn encode(data: &[u64], output: &mut [u8]) {
    let len = data.len();

    let mut i = 0;
    let mut j = 0;

    while j < len {

        output[j] = (data[i] & 0xff) as u8;
        output[j+1] = ((data[i] >> 8) & 0xff) as u8;
        output[j+2] = ((data[i] >> 16) & 0xff) as u8;
        output[j+3] = ((data[i] >> 24) & 0xff) as u8;

        i += 1;
        j += 4;
    }
}

fn encode_32(data: &[u32], output: &mut [u8]) {
    let len = 4;

    let mut i = 0;
    let mut j = 0;

    while j < len {

        output[j] = (data[i] & 0xff) as u8;
        output[j+1] = ((data[i] >> 8) & 0xff) as u8;
        output[j+2] = ((data[i] >> 16) & 0xff) as u8;
        output[j+3] = ((data[i] >> 24) & 0xff) as u8;

        i += 1;
        j += 4;
    }
}

#[inline(always)]
fn rotate_left_avx2(x: __m256i, n: i32) -> __m256i {
    macro_rules! slli {
            ($amt_const:expr) => {
                unsafe { _mm256_slli_epi32(x, $amt_const) }
            };
        }

    macro_rules! srli {
            ($amt_const:expr) => {
                unsafe { _mm256_srli_epi32(x, $amt_const) }
            };
        }

    let right_n = 32 - n;
    let left = constify_imm8!(n, slli);
    let right = constify_imm8!(right_n, srli);

    unsafe { _mm256_or_si256(left, right) }
}

#[inline(always)]
fn md5_f_avx2(x: __m256i, y: __m256i, z: __m256i) -> __m256i {
    unsafe { _mm256_or_si256(_mm256_and_si256(x, y), _mm256_andnot_si256(x, z)) }
}

#[inline(always)]
fn md5_g_avx2(x: __m256i, y: __m256i, z: __m256i) -> __m256i {
    unsafe { _mm256_or_si256(_mm256_and_si256(x, z), _mm256_andnot_si256(z, y)) }
}

#[inline(always)]
fn md5_h_avx2(x: __m256i, y: __m256i, z: __m256i) -> __m256i {
    unsafe { _mm256_xor_si256(x, _mm256_xor_si256(y, z)) }
}

#[inline(always)]
fn md5_i_avx2(x: __m256i, y: __m256i, z: __m256i) -> __m256i {
    unsafe { _mm256_xor_si256(y, _mm256_or_si256(x, _mm256_andnot_si256(z, _mm256_cmpeq_epi32(z, z)))) }
}

pub struct MD5_AVX2 {
    state: [__m256i; 4],
    count: [u64; 2],
    b1: [u8; 64],
    b2: [u8; 64],
    b3: [u8; 64],
    b4: [u8; 64],
    b5: [u8; 64],
    b6: [u8; 64],
    b7: [u8; 64],
    b8: [u8; 64],
}

macro_rules! MD5_AVX2_FUNCTION {
    ($f:ident, $a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr, $ac:expr) => {
        let res = $f($b, $c, $d);

        $a = unsafe { _mm256_add_epi32($a, _mm256_add_epi32(_mm256_add_epi32(res, $x), $ac)) };
        $a = rotate_left_avx2($a, $s);
        $a = unsafe { _mm256_add_epi32($a, $b) };
    }
}

impl MD5_AVX2 {

    fn transform_blocks(&mut self) {
        let mut a: __m256i = self.state[0];
        let mut b: __m256i = self.state[1];
        let mut c: __m256i = self.state[2];
        let mut d: __m256i = self.state[3];

        let mut x: [__m256i; 16] = [ unsafe { _mm256_set_epi32(0, 0, 0, 0, 0, 0, 0, 0) } ; 16];

        for (index, index2) in [0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60].iter().enumerate() {

            let index2_u = *index2 as usize;

            x[index] = unsafe { _mm256_set_epi32(
                (self.b8[index2_u] as i32) | ((self.b8[index2_u+1] as i32) << 8) | ((self.b8[index2_u+2] as i32) << 16) | ((self.b8[index2_u+3] as i32) << 24),
                (self.b7[index2_u] as i32) | ((self.b7[index2_u+1] as i32) << 8) | ((self.b7[index2_u+2] as i32) << 16) | ((self.b7[index2_u+3] as i32) << 24),
                (self.b6[index2_u] as i32) | ((self.b6[index2_u+1] as i32) << 8) | ((self.b6[index2_u+2] as i32) << 16) | ((self.b6[index2_u+3] as i32) << 24),
                (self.b5[index2_u] as i32) | ((self.b5[index2_u+1] as i32) << 8) | ((self.b5[index2_u+2] as i32) << 16) | ((self.b5[index2_u+3] as i32) << 24),
                (self.b4[index2_u] as i32) | ((self.b4[index2_u+1] as i32) << 8) | ((self.b4[index2_u+2] as i32) << 16) | ((self.b4[index2_u+3] as i32) << 24),
                (self.b3[index2_u] as i32) | ((self.b3[index2_u+1] as i32) << 8) | ((self.b3[index2_u+2] as i32) << 16) | ((self.b3[index2_u+3] as i32) << 24),
                (self.b2[index2_u] as i32) | ((self.b2[index2_u+1] as i32) << 8) | ((self.b2[index2_u+2] as i32) << 16) | ((self.b2[index2_u+3] as i32) << 24),
                (self.b1[index2_u] as i32) | ((self.b1[index2_u+1] as i32) << 8) | ((self.b1[index2_u+2] as i32) << 16) | ((self.b1[index2_u+3] as i32) << 24)
            )};
        }

        // round 1 F
        MD5_AVX2_FUNCTION!(md5_f_avx2, a, b, c, d, x[ 0], S11, _mm256_set1_epi32((0xd76aa478 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, d, a, b, c, x[ 1], S12, _mm256_set1_epi32((0xe8c7b756 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, c, d, a, b, x[ 2], S13, _mm256_set1_epi32((0x242070db as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, b, c, d, a, x[ 3], S14, _mm256_set1_epi32((0xc1bdceee as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, a, b, c, d, x[ 4], S11, _mm256_set1_epi32((0xf57c0faf as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, d, a, b, c, x[ 5], S12, _mm256_set1_epi32((0x4787c62a as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, c, d, a, b, x[ 6], S13, _mm256_set1_epi32((0xa8304613 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, b, c, d, a, x[ 7], S14, _mm256_set1_epi32((0xfd469501 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, a, b, c, d, x[ 8], S11, _mm256_set1_epi32((0x698098d8 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, d, a, b, c, x[ 9], S12, _mm256_set1_epi32((0x8b44f7af as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, c, d, a, b, x[10], S13, _mm256_set1_epi32((0xffff5bb1 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, b, c, d, a, x[11], S14, _mm256_set1_epi32((0x895cd7be as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, a, b, c, d, x[12], S11, _mm256_set1_epi32((0x6b901122 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, d, a, b, c, x[13], S12, _mm256_set1_epi32((0xfd987193 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, c, d, a, b, x[14], S13, _mm256_set1_epi32((0xa679438e as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, b, c, d, a, x[15], S14, _mm256_set1_epi32((0x49b40821 as u32) as i32));

        // round 2 G
        MD5_AVX2_FUNCTION!(md5_g_avx2, a, b, c, d, x[ 1], S21, _mm256_set1_epi32((0xf61e2562 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, d, a, b, c, x[ 6], S22, _mm256_set1_epi32((0xc040b340 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, c, d, a, b, x[11], S23, _mm256_set1_epi32((0x265e5a51 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, b, c, d, a, x[ 0], S24, _mm256_set1_epi32((0xe9b6c7aa as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, a, b, c, d, x[ 5], S21, _mm256_set1_epi32((0xd62f105d as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, d, a, b, c, x[10], S22, _mm256_set1_epi32(( 0x2441453 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, c, d, a, b, x[15], S23, _mm256_set1_epi32((0xd8a1e681 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, b, c, d, a, x[ 4], S24, _mm256_set1_epi32((0xe7d3fbc8 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, a, b, c, d, x[ 9], S21, _mm256_set1_epi32((0x21e1cde6 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, d, a, b, c, x[14], S22, _mm256_set1_epi32((0xc33707d6 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, c, d, a, b, x[ 3], S23, _mm256_set1_epi32((0xf4d50d87 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, b, c, d, a, x[ 8], S24, _mm256_set1_epi32((0x455a14ed as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, a, b, c, d, x[13], S21, _mm256_set1_epi32((0xa9e3e905 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, d, a, b, c, x[ 2], S22, _mm256_set1_epi32((0xfcefa3f8 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, c, d, a, b, x[ 7], S23, _mm256_set1_epi32((0x676f02d9 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, b, c, d, a, x[12], S24, _mm256_set1_epi32((0x8d2a4c8a as u32) as i32));

        // round 3 H
        MD5_AVX2_FUNCTION!(md5_h_avx2, a, b, c, d, x[ 5], S31, _mm256_set1_epi32((0xfffa3942 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, d, a, b, c, x[ 8], S32, _mm256_set1_epi32((0x8771f681 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, c, d, a, b, x[11], S33, _mm256_set1_epi32((0x6d9d6122 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, b, c, d, a, x[14], S34, _mm256_set1_epi32((0xfde5380c as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, a, b, c, d, x[ 1], S31, _mm256_set1_epi32((0xa4beea44 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, d, a, b, c, x[ 4], S32, _mm256_set1_epi32((0x4bdecfa9 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, c, d, a, b, x[ 7], S33, _mm256_set1_epi32((0xf6bb4b60 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, b, c, d, a, x[10], S34, _mm256_set1_epi32((0xbebfbc70 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, a, b, c, d, x[13], S31, _mm256_set1_epi32((0x289b7ec6 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, d, a, b, c, x[ 0], S32, _mm256_set1_epi32((0xeaa127fa as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, c, d, a, b, x[ 3], S33, _mm256_set1_epi32((0xd4ef3085 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, b, c, d, a, x[ 6], S34, _mm256_set1_epi32(( 0x4881d05 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, a, b, c, d, x[ 9], S31, _mm256_set1_epi32((0xd9d4d039 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, d, a, b, c, x[12], S32, _mm256_set1_epi32((0xe6db99e5 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, c, d, a, b, x[15], S33, _mm256_set1_epi32((0x1fa27cf8 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, b, c, d, a, x[ 2], S34, _mm256_set1_epi32((0xc4ac5665 as u32) as i32));

        // round 4 I
        MD5_AVX2_FUNCTION!(md5_i_avx2, a, b, c, d, x[ 0], S41, _mm256_set1_epi32((0xf4292244 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, d, a, b, c, x[ 7], S42, _mm256_set1_epi32((0x432aff97 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, c, d, a, b, x[14], S43, _mm256_set1_epi32((0xab9423a7 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, b, c, d, a, x[ 5], S44, _mm256_set1_epi32((0xfc93a039 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, a, b, c, d, x[12], S41, _mm256_set1_epi32((0x655b59c3 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, d, a, b, c, x[ 3], S42, _mm256_set1_epi32((0x8f0ccc92 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, c, d, a, b, x[10], S43, _mm256_set1_epi32((0xffeff47d as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, b, c, d, a, x[ 1], S44, _mm256_set1_epi32((0x85845dd1 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, a, b, c, d, x[ 8], S41, _mm256_set1_epi32((0x6fa87e4f as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, d, a, b, c, x[15], S42, _mm256_set1_epi32((0xfe2ce6e0 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, c, d, a, b, x[ 6], S43, _mm256_set1_epi32((0xa3014314 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, b, c, d, a, x[13], S44, _mm256_set1_epi32((0x4e0811a1 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, a, b, c, d, x[ 4], S41, _mm256_set1_epi32((0xf7537e82 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, d, a, b, c, x[11], S42, _mm256_set1_epi32((0xbd3af235 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, c, d, a, b, x[ 2], S43, _mm256_set1_epi32((0x2ad7d2bb as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, b, c, d, a, x[ 9], S44, _mm256_set1_epi32((0xeb86d391 as u32) as i32));

        self.state[0] = unsafe { _mm256_add_epi32(self.state[0], a) };
        self.state[1] = unsafe { _mm256_add_epi32(self.state[1], b) };
        self.state[2] = unsafe { _mm256_add_epi32(self.state[2], c) };
        self.state[3] = unsafe { _mm256_add_epi32(self.state[3], d) };

        // reset
    }

    fn transform(&mut self, b1: &[u8], b2: &[u8], b3: &[u8], b4: &[u8], b5: &[u8], b6: &[u8], b7: &[u8], b8: &[u8]) {
        let mut a: __m256i = self.state[0];
        let mut b: __m256i = self.state[1];
        let mut c: __m256i = self.state[2];
        let mut d: __m256i = self.state[3];

        let mut x: [__m256i; 16] = [ unsafe { _mm256_set_epi32(0, 0, 0, 0, 0, 0, 0, 0) } ; 16];

        for (index, index2) in [0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60].iter().enumerate() {

            let index2_u = *index2 as usize;

            x[index] = unsafe { _mm256_set_epi32(
                (b8[index2_u] as i32) | ((b8[index2_u+1] as i32) << 8) | ((b8[index2_u+2] as i32) << 16) | ((b8[index2_u+3] as i32) << 24),
                (b7[index2_u] as i32) | ((b7[index2_u+1] as i32) << 8) | ((b7[index2_u+2] as i32) << 16) | ((b7[index2_u+3] as i32) << 24),
                (b6[index2_u] as i32) | ((b6[index2_u+1] as i32) << 8) | ((b6[index2_u+2] as i32) << 16) | ((b6[index2_u+3] as i32) << 24),
                (b5[index2_u] as i32) | ((b5[index2_u+1] as i32) << 8) | ((b5[index2_u+2] as i32) << 16) | ((b5[index2_u+3] as i32) << 24),
                (b4[index2_u] as i32) | ((b4[index2_u+1] as i32) << 8) | ((b4[index2_u+2] as i32) << 16) | ((b4[index2_u+3] as i32) << 24),
                (b3[index2_u] as i32) | ((b3[index2_u+1] as i32) << 8) | ((b3[index2_u+2] as i32) << 16) | ((b3[index2_u+3] as i32) << 24),
                (b2[index2_u] as i32) | ((b2[index2_u+1] as i32) << 8) | ((b2[index2_u+2] as i32) << 16) | ((b2[index2_u+3] as i32) << 24),
                (b1[index2_u] as i32) | ((b1[index2_u+1] as i32) << 8) | ((b1[index2_u+2] as i32) << 16) | ((b1[index2_u+3] as i32) << 24)
            )};
        }

        // round 1 F
        MD5_AVX2_FUNCTION!(md5_f_avx2, a, b, c, d, x[ 0], S11, _mm256_set1_epi32((0xd76aa478 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, d, a, b, c, x[ 1], S12, _mm256_set1_epi32((0xe8c7b756 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, c, d, a, b, x[ 2], S13, _mm256_set1_epi32((0x242070db as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, b, c, d, a, x[ 3], S14, _mm256_set1_epi32((0xc1bdceee as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, a, b, c, d, x[ 4], S11, _mm256_set1_epi32((0xf57c0faf as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, d, a, b, c, x[ 5], S12, _mm256_set1_epi32((0x4787c62a as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, c, d, a, b, x[ 6], S13, _mm256_set1_epi32((0xa8304613 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, b, c, d, a, x[ 7], S14, _mm256_set1_epi32((0xfd469501 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, a, b, c, d, x[ 8], S11, _mm256_set1_epi32((0x698098d8 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, d, a, b, c, x[ 9], S12, _mm256_set1_epi32((0x8b44f7af as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, c, d, a, b, x[10], S13, _mm256_set1_epi32((0xffff5bb1 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, b, c, d, a, x[11], S14, _mm256_set1_epi32((0x895cd7be as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, a, b, c, d, x[12], S11, _mm256_set1_epi32((0x6b901122 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, d, a, b, c, x[13], S12, _mm256_set1_epi32((0xfd987193 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, c, d, a, b, x[14], S13, _mm256_set1_epi32((0xa679438e as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, b, c, d, a, x[15], S14, _mm256_set1_epi32((0x49b40821 as u32) as i32));

        // round 2 G
        MD5_AVX2_FUNCTION!(md5_g_avx2, a, b, c, d, x[ 1], S21, _mm256_set1_epi32((0xf61e2562 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, d, a, b, c, x[ 6], S22, _mm256_set1_epi32((0xc040b340 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, c, d, a, b, x[11], S23, _mm256_set1_epi32((0x265e5a51 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, b, c, d, a, x[ 0], S24, _mm256_set1_epi32((0xe9b6c7aa as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, a, b, c, d, x[ 5], S21, _mm256_set1_epi32((0xd62f105d as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, d, a, b, c, x[10], S22, _mm256_set1_epi32(( 0x2441453 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, c, d, a, b, x[15], S23, _mm256_set1_epi32((0xd8a1e681 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, b, c, d, a, x[ 4], S24, _mm256_set1_epi32((0xe7d3fbc8 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, a, b, c, d, x[ 9], S21, _mm256_set1_epi32((0x21e1cde6 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, d, a, b, c, x[14], S22, _mm256_set1_epi32((0xc33707d6 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, c, d, a, b, x[ 3], S23, _mm256_set1_epi32((0xf4d50d87 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, b, c, d, a, x[ 8], S24, _mm256_set1_epi32((0x455a14ed as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, a, b, c, d, x[13], S21, _mm256_set1_epi32((0xa9e3e905 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, d, a, b, c, x[ 2], S22, _mm256_set1_epi32((0xfcefa3f8 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, c, d, a, b, x[ 7], S23, _mm256_set1_epi32((0x676f02d9 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_g_avx2, b, c, d, a, x[12], S24, _mm256_set1_epi32((0x8d2a4c8a as u32) as i32));

        // round 3 H
        MD5_AVX2_FUNCTION!(md5_h_avx2, a, b, c, d, x[ 5], S31, _mm256_set1_epi32((0xfffa3942 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, d, a, b, c, x[ 8], S32, _mm256_set1_epi32((0x8771f681 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, c, d, a, b, x[11], S33, _mm256_set1_epi32((0x6d9d6122 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, b, c, d, a, x[14], S34, _mm256_set1_epi32((0xfde5380c as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, a, b, c, d, x[ 1], S31, _mm256_set1_epi32((0xa4beea44 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, d, a, b, c, x[ 4], S32, _mm256_set1_epi32((0x4bdecfa9 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, c, d, a, b, x[ 7], S33, _mm256_set1_epi32((0xf6bb4b60 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, b, c, d, a, x[10], S34, _mm256_set1_epi32((0xbebfbc70 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, a, b, c, d, x[13], S31, _mm256_set1_epi32((0x289b7ec6 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, d, a, b, c, x[ 0], S32, _mm256_set1_epi32((0xeaa127fa as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, c, d, a, b, x[ 3], S33, _mm256_set1_epi32((0xd4ef3085 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, b, c, d, a, x[ 6], S34, _mm256_set1_epi32(( 0x4881d05 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, a, b, c, d, x[ 9], S31, _mm256_set1_epi32((0xd9d4d039 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, d, a, b, c, x[12], S32, _mm256_set1_epi32((0xe6db99e5 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, c, d, a, b, x[15], S33, _mm256_set1_epi32((0x1fa27cf8 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, b, c, d, a, x[ 2], S34, _mm256_set1_epi32((0xc4ac5665 as u32) as i32));

        // round 4 I
        MD5_AVX2_FUNCTION!(md5_i_avx2, a, b, c, d, x[ 0], S41, _mm256_set1_epi32((0xf4292244 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, d, a, b, c, x[ 7], S42, _mm256_set1_epi32((0x432aff97 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, c, d, a, b, x[14], S43, _mm256_set1_epi32((0xab9423a7 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, b, c, d, a, x[ 5], S44, _mm256_set1_epi32((0xfc93a039 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, a, b, c, d, x[12], S41, _mm256_set1_epi32((0x655b59c3 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, d, a, b, c, x[ 3], S42, _mm256_set1_epi32((0x8f0ccc92 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, c, d, a, b, x[10], S43, _mm256_set1_epi32((0xffeff47d as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, b, c, d, a, x[ 1], S44, _mm256_set1_epi32((0x85845dd1 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, a, b, c, d, x[ 8], S41, _mm256_set1_epi32((0x6fa87e4f as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, d, a, b, c, x[15], S42, _mm256_set1_epi32((0xfe2ce6e0 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, c, d, a, b, x[ 6], S43, _mm256_set1_epi32((0xa3014314 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, b, c, d, a, x[13], S44, _mm256_set1_epi32((0x4e0811a1 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, a, b, c, d, x[ 4], S41, _mm256_set1_epi32((0xf7537e82 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, d, a, b, c, x[11], S42, _mm256_set1_epi32((0xbd3af235 as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, c, d, a, b, x[ 2], S43, _mm256_set1_epi32((0x2ad7d2bb as u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, b, c, d, a, x[ 9], S44, _mm256_set1_epi32((0xeb86d391 as u32) as i32));

        self.state[0] = unsafe { _mm256_add_epi32(self.state[0], a) };
        self.state[1] = unsafe { _mm256_add_epi32(self.state[1], b) };
        self.state[2] = unsafe { _mm256_add_epi32(self.state[2], c) };
        self.state[3] = unsafe { _mm256_add_epi32(self.state[3], d) };

        // reset
    }

    #[inline(always)]
    fn update1(&mut self, data: &[u8]) {
        self.update(data, data, data, data, data, data, data, data);
    }

    #[inline(always)]
    fn update2(&mut self, data1: &[u8], data2: &[u8]) {
        self.update(data1, data2, data2, data2, data2, data2, data2, data2);
    }

    #[inline(always)]
    fn update3(&mut self, data1: &[u8], data2: &[u8], data3: &[u8]) {
        self.update(data1, data2, data3, data3, data3, data3, data3, data3);
    }

    #[inline(always)]
    fn update4(&mut self, data1: &[u8], data2: &[u8], data3: &[u8], data4: &[u8]) {
        self.update(data1, data2, data3, data4, data4, data4, data4, data4);
    }

    #[inline(always)]
    fn update5(&mut self, data1: &[u8], data2: &[u8], data3: &[u8], data4: &[u8], data5: &[u8]) {
        self.update(data1, data2, data3, data4, data5, data5, data5, data5);
    }

    #[inline(always)]
    fn update6(&mut self, data1: &[u8], data2: &[u8], data3: &[u8], data4: &[u8], data5: &[u8], data6: &[u8]) {
        self.update(data1, data2, data3, data4, data5, data6, data6, data6);
    }

    #[inline(always)]
    fn update7(&mut self, data1: &[u8], data2: &[u8], data3: &[u8], data4: &[u8], data5: &[u8], data6: &[u8], data7: &[u8]) {
        self.update(data1, data2, data3, data4, data5, data6, data7, data7);
    }

    #[inline(always)]
    fn update8(&mut self, data1: &[u8], data2: &[u8], data3: &[u8], data4: &[u8], data5: &[u8], data6: &[u8], data7: &[u8], data8: &[u8]) {
        self.update(data1, data2, data3, data4, data5, data6, data7, data8);
    }

    fn update(&mut self, data1: &[u8], data2: &[u8], data3: &[u8], data4: &[u8], data5: &[u8], data6: &[u8], data7: &[u8], data8: &[u8]) {
        let mut i = 0;
        let mut index = (self.count[0] >> 3) & 0x3F;
        let data_len = data1.len();

        self.count[0] += (data_len << 3) as u64;
        if self.count[0] < (data_len << 3) as u64 {
            self.count[1] += 1;
        }

        self.count[1] += (data_len >> 29) as u64;

        let partial_len = 64 - index;
        let partial_len_usize = partial_len as usize;

        if data_len >= partial_len_usize {

            let index_usize = index as usize;

            unsafe { core::intrinsics::copy_nonoverlapping(data1.as_ptr(), self.b1[(index_usize)..].as_mut_ptr(), partial_len_usize); }
            unsafe { core::intrinsics::copy_nonoverlapping(data2.as_ptr(), self.b2[(index_usize)..].as_mut_ptr(), partial_len_usize); }
            unsafe { core::intrinsics::copy_nonoverlapping(data3.as_ptr(), self.b3[(index_usize)..].as_mut_ptr(), partial_len_usize); }
            unsafe { core::intrinsics::copy_nonoverlapping(data4.as_ptr(), self.b4[(index_usize)..].as_mut_ptr(), partial_len_usize); }
            unsafe { core::intrinsics::copy_nonoverlapping(data5.as_ptr(), self.b5[(index_usize)..].as_mut_ptr(), partial_len_usize); }
            unsafe { core::intrinsics::copy_nonoverlapping(data6.as_ptr(), self.b6[(index_usize)..].as_mut_ptr(), partial_len_usize); }
            unsafe { core::intrinsics::copy_nonoverlapping(data7.as_ptr(), self.b7[(index_usize)..].as_mut_ptr(), partial_len_usize); }
            unsafe { core::intrinsics::copy_nonoverlapping(data8.as_ptr(), self.b8[(index_usize)..].as_mut_ptr(), partial_len_usize); }

            self.transform_blocks();

            i = partial_len;
            while i + 63 < data_len as u64 {

                let subindex = i as usize;

                self.transform(
                    &data1[subindex..],
                    &data2[subindex..],
                    &data3[subindex..],
                    &data4[subindex..],
                    &data5[subindex..],
                    &data6[subindex..],
                    &data7[subindex..],
                    &data8[subindex..],
                );

                i += 64;
            }

            index = 0;

        } else {
            i = 0;
        }

        let i_usize = i as usize;
        let index_usize = index as usize;
        let copy_n = data_len - (i as usize);

        unsafe { core::intrinsics::copy_nonoverlapping(data1[i_usize..].as_ptr(), self.b1[(index_usize)..].as_mut_ptr(), copy_n); }
        unsafe { core::intrinsics::copy_nonoverlapping(data2[i_usize..].as_ptr(), self.b2[(index_usize)..].as_mut_ptr(), copy_n); }
        unsafe { core::intrinsics::copy_nonoverlapping(data3[i_usize..].as_ptr(), self.b3[(index_usize)..].as_mut_ptr(), copy_n); }
        unsafe { core::intrinsics::copy_nonoverlapping(data4[i_usize..].as_ptr(), self.b4[(index_usize)..].as_mut_ptr(), copy_n); }
        unsafe { core::intrinsics::copy_nonoverlapping(data5[i_usize..].as_ptr(), self.b5[(index_usize)..].as_mut_ptr(), copy_n); }
        unsafe { core::intrinsics::copy_nonoverlapping(data6[i_usize..].as_ptr(), self.b6[(index_usize)..].as_mut_ptr(), copy_n); }
        unsafe { core::intrinsics::copy_nonoverlapping(data7[i_usize..].as_ptr(), self.b7[(index_usize)..].as_mut_ptr(), copy_n); }
        unsafe { core::intrinsics::copy_nonoverlapping(data8[i_usize..].as_ptr(), self.b8[(index_usize)..].as_mut_ptr(), copy_n); }
    }

    fn finalize(&mut self) -> [[u8; 16]; 8] {
        let mut bits: [u8; 8] = [0; 8];

        encode(&self.count[..], &mut bits[..]);

        let index = (self.count[0] >> 3) & 0x3f;
        let padlen = if index < 56 { 56 - index } else { 120 - index };

        self.update1(&PAD[..(padlen as usize)]);
        self.update1(&bits);

        let mut digests: [[u8; 16]; 8] = [[0;16];8];

        for i in 0..8 {
            for j in 0..4 {
                let split_m256i = unsafe { core::mem::transmute::<&__m256i, &[u32;8]>(&self.state[j]) };
                encode_32(&split_m256i[i..], &mut digests[i][(j*4)..]);
            }
        }

        digests
    }

    pub fn digest(input: &[u8]) -> [[u8; 16]; 8] {
        let mut test = Self::default();
        test.update1(input);
        return test.finalize();
    }

    pub fn digest2(input1: &[u8], input2: &[u8]) {
        let mut object = Self::default();
        object.update2(input1, input2);
        object.finalize();
    }

    pub fn digest3(input1: &[u8], input2: &[u8], input3: &[u8]) {
        let mut object = Self::default();
        object.update3(input1, input2, input3);
        object.finalize();
    }

    pub fn digest4(input1: &[u8], input2: &[u8], input3: &[u8], input4: &[u8]) {
        let mut object = Self::default();
        object.update4(input1, input2, input3, input4);
        object.finalize();
    }

    pub fn digest5(input1: &[u8], input2: &[u8], input3: &[u8], input4: &[u8], input5: &[u8]) {
        let mut object = Self::default();
        object.update5(input1, input2, input3, input4, input5);
        object.finalize();
    }

    pub fn digest6(input1: &[u8], input2: &[u8], input3: &[u8], input4: &[u8], input5: &[u8], input6: &[u8]) {
        let mut object = Self::default();
        object.update6(input1, input2, input3, input4, input5, input6);
        object.finalize();
    }

    pub fn digest7(input1: &[u8], input2: &[u8], input3: &[u8], input4: &[u8], input5: &[u8], input6: &[u8], input7: &[u8]) {
        let mut object = Self::default();
        object.update7(input1, input2, input3, input4, input5, input6, input7);
        object.finalize();
    }

    pub fn digest8(input1: &[u8],input2: &[u8],input3: &[u8],input4: &[u8],input5: &[u8],input6: &[u8],input7: &[u8],input8: &[u8]) {
        let mut test = Self::default();
        test.update8(input1, input2, input3, input4, input5, input6, input7, input8);
        test.finalize();
    }
}

impl Default for MD5_AVX2 {
    fn default() -> Self {
        Self {
            state: [
                unsafe { _mm256_set1_epi32((0x67452301 as u32) as i32) },
                unsafe { _mm256_set1_epi32((0xefcdab89 as u32) as i32) },
                unsafe { _mm256_set1_epi32((0x98badcfe as u32) as i32) },
                unsafe { _mm256_set1_epi32((0x10325476 as u32) as i32) },
            ],
            count: [0, 0],
            b1: [0; 64],
            b2: [0; 64],
            b3: [0; 64],
            b4: [0; 64],
            b5: [0; 64],
            b6: [0; 64],
            b7: [0; 64],
            b8: [0; 64],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    #[test]
    fn md5_32bytes() {
        let hash = MD5_AVX2::digest("wqvDrDLilCUevxUw5fWEuVc6y6ElCrHg".as_bytes());
        assert_eq!(hash[0], [0x69, 0xca, 0x36, 0x72, 0x1d, 0xd0, 0x20, 0x97, 0x82, 0xc3, 0x76, 0x17, 0xf2, 0x20, 0xab, 0x82]);
    }

    #[test]
    fn md5_64bytes() {
        let hash = MD5_AVX2::digest("K7CN3VzXyY63NXmW15TKA4O6vJtVrLc7I0B5qHRtBir5PkwSt6xgJopOCunPk2ky".as_bytes());
        assert_eq!(hash[0], [0x6f, 0xbe, 0xd0, 0x7f, 0xcc, 0x87, 0x90, 0xb2, 0xcb, 0x56, 0x03, 0x02, 0xa5, 0xc7, 0x5e, 0x56]);
    }

    #[test]
    fn md5_128bytes() {
        let hash = MD5_AVX2::digest("KAjb6sifm7DwdyJyMXT3np6WZVfXJiEskX1fN7V8YOatxuRkpHYZmqDXY2Kn2pfnV63l0bodaXjRdVF5m2z1bC7QpdQi3UHRI9KAqWs0vO0QjT5XtkTXKlaRK4CiBsT1".as_bytes());
        assert_eq!(hash[0], [0x59, 0xa4, 0x18, 0xec, 0xcd, 0xff, 0x43, 0xeb, 0xfa, 0xb1, 0x63, 0x91, 0x6c, 0x60, 0x2c, 0x15]);
    }

    #[test]
    fn md5_256bytes() {
        let hash = MD5_AVX2::digest("QnpFg2P1SEQ0L9tcNwBROCW7jVtFeMt0RuF7QODKkgD75CPDi1pAB1GtMcq0G1pmNE6J3IuPpF33uPtOs4sNwU7lKcnF8SU016PKWPeVEpuKQ2ksT9enIf1hVrzlypOkhFTFhIS28IT9OQZ3BS3693487mSb6QNuuaBCD8yNWWlo74c79EFWUWNaAmRcSxVaNcbDa80SovlnL8lyO2yS7XlmE7rPmLI4IvPtko3QguI4Th2JPrVnM7QCCjMgvlIO".as_bytes());
        assert_eq!(hash[0], [0xe5, 0x0b, 0xe7, 0x33, 0x07, 0x72, 0xe0, 0x89, 0xf6, 0x5e, 0x0e, 0xfd, 0x98, 0x69, 0xc9, 0xe3]);
    }

    #[bench]
    fn bench_sha256_32bytes(b: &mut Bencher) {
        b.iter(|| {
            MD5_AVX2::digest("wqvDrDLilCUevxUw5fWEuVc6y6ElCrHg".as_bytes());
        });
    }

    #[bench]
    fn bench_md5_64bytes(b: &mut Bencher) {
        b.iter(|| {
            MD5_AVX2::digest("K7CN3VzXyY63NXmW15TKA4O6vJtVrLc7I0B5qHRtBir5PkwSt6xgJopOCunPk2ky".as_bytes());
        });
    }

    #[bench]
    fn bench_sha256_128bytes(b: &mut Bencher) {
        b.iter(|| {
            MD5_AVX2::digest("KAjb6sifm7DwdyJyMXT3np6WZVfXJiEskX1fN7V8YOatxuRkpHYZmqDXY2Kn2pfnV63l0bodaXjRdVF5m2z1bC7QpdQi3UHRI9KAqWs0vO0QjT5XtkTXKlaRK4CiBsT1".as_bytes());
        });
    }

    #[bench]
    fn bench_sha256_256bytes(b: &mut Bencher) {
        b.iter(|| {
            MD5_AVX2::digest("QnpFg2P1SEQ0L9tcNwBROCW7jVtFeMt0RuF7QODKkgD75CPDi1pAB1GtMcq0G1pmNE6J3IuPpF33uPtOs4sNwU7lKcnF8SU016PKWPeVEpuKQ2ksT9enIf1hVrzlypOkhFTFhIS28IT9OQZ3BS3693487mSb6QNuuaBCD8yNWWlo74c79EFWUWNaAmRcSxVaNcbDa80SovlnL8lyO2yS7XlmE7rPmLI4IvPtko3QguI4Th2JPrVnM7QCCjMgvlIO".as_bytes());
        });
    }
}

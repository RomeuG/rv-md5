use core::arch::x86_64::*;

use crate::utils::*;

macro_rules! MD5_AVX2_FUNCTION {
    ($f:ident, $a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr, $ac:expr) => {
        let res = $f($b, $c, $d);

        $a = unsafe { _mm256_add_epi32($a, _mm256_add_epi32(_mm256_add_epi32(res, $x), $ac)) };
        $a = rotate_left_avx2($a, $s);
        $a = unsafe { _mm256_add_epi32($a, $b) };
    }
}

macro_rules! MD5_AVX2_FUNCTION_G {
    ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr, $ac:expr) => {
        $a = unsafe { _mm256_add_epi32($a, _mm256_add_epi32($x, $ac)) };
        $a = unsafe { _mm256_add_epi32($a, _mm256_andnot_si256($d, $c)) };
        $a = unsafe { _mm256_add_epi32($a, _mm256_and_si256($d, $b)) };
        $a = rotate_left_avx2($a, $s);
        $a = unsafe { _mm256_add_epi32($a, $b) };
    }
}

#[inline(always)]
fn md5_f_avx2(x: __m256i, y: __m256i, z: __m256i) -> __m256i {
    unsafe { _mm256_or_si256(_mm256_and_si256(x, y), _mm256_andnot_si256(x, z)) }
}

#[inline(always)]
fn md5_h_avx2(x: __m256i, y: __m256i, z: __m256i) -> __m256i {
    unsafe { _mm256_xor_si256(x, _mm256_xor_si256(y, z)) }
}

#[inline(always)]
fn md5_i_avx2(x: __m256i, y: __m256i, z: __m256i) -> __m256i {
    // NOTE: optimization possible, review
    unsafe { _mm256_xor_si256(y, _mm256_or_si256(x, _mm256_andnot_si256(z, _mm256_cmpeq_epi32(z, z)))) }
}

pub struct Md5Avx2 {
    state: [__m256i; 4],
    count: [u64; 2],
    blocks: [[u8; 64]; 8],
}

impl Md5Avx2 {

    fn transform_blocks(&mut self) {
        let mut a: __m256i = self.state[0];
        let mut b: __m256i = self.state[1];
        let mut c: __m256i = self.state[2];
        let mut d: __m256i = self.state[3];

        let mut x: [__m256i; 16] = [ unsafe { _mm256_set_epi32(0, 0, 0, 0, 0, 0, 0, 0) } ; 16];

        for (index, index2) in [0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60].iter().enumerate() {

            let index2_u = *index2 as usize;

            x[index] = unsafe { _mm256_set_epi32(
                (self.blocks[7][index2_u] as i32) | ((self.blocks[7][index2_u+1] as i32) << 8) | ((self.blocks[7][index2_u+2] as i32) << 16) | ((self.blocks[7][index2_u+3] as i32) << 24),
                (self.blocks[6][index2_u] as i32) | ((self.blocks[6][index2_u+1] as i32) << 8) | ((self.blocks[6][index2_u+2] as i32) << 16) | ((self.blocks[6][index2_u+3] as i32) << 24),
                (self.blocks[5][index2_u] as i32) | ((self.blocks[5][index2_u+1] as i32) << 8) | ((self.blocks[5][index2_u+2] as i32) << 16) | ((self.blocks[5][index2_u+3] as i32) << 24),
                (self.blocks[4][index2_u] as i32) | ((self.blocks[4][index2_u+1] as i32) << 8) | ((self.blocks[4][index2_u+2] as i32) << 16) | ((self.blocks[4][index2_u+3] as i32) << 24),
                (self.blocks[3][index2_u] as i32) | ((self.blocks[3][index2_u+1] as i32) << 8) | ((self.blocks[3][index2_u+2] as i32) << 16) | ((self.blocks[3][index2_u+3] as i32) << 24),
                (self.blocks[2][index2_u] as i32) | ((self.blocks[2][index2_u+1] as i32) << 8) | ((self.blocks[2][index2_u+2] as i32) << 16) | ((self.blocks[2][index2_u+3] as i32) << 24),
                (self.blocks[1][index2_u] as i32) | ((self.blocks[1][index2_u+1] as i32) << 8) | ((self.blocks[1][index2_u+2] as i32) << 16) | ((self.blocks[1][index2_u+3] as i32) << 24),
                (self.blocks[0][index2_u] as i32) | ((self.blocks[0][index2_u+1] as i32) << 8) | ((self.blocks[0][index2_u+2] as i32) << 16) | ((self.blocks[0][index2_u+3] as i32) << 24)
            )};
        }

        // round 1 F
        MD5_AVX2_FUNCTION!(md5_f_avx2, a, b, c, d, x[ 0], 7, _mm256_set1_epi32((0xd76aa478_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, d, a, b, c, x[ 1], 12, _mm256_set1_epi32((0xe8c7b756_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, c, d, a, b, x[ 2], 17, _mm256_set1_epi32((0x242070db_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, b, c, d, a, x[ 3], 22, _mm256_set1_epi32((0xc1bdceee_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, a, b, c, d, x[ 4], 7, _mm256_set1_epi32((0xf57c0faf_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, d, a, b, c, x[ 5], 12, _mm256_set1_epi32((0x4787c62a_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, c, d, a, b, x[ 6], 17, _mm256_set1_epi32((0xa8304613_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, b, c, d, a, x[ 7], 22, _mm256_set1_epi32((0xfd469501_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, a, b, c, d, x[ 8], 7, _mm256_set1_epi32((0x698098d8_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, d, a, b, c, x[ 9], 12, _mm256_set1_epi32((0x8b44f7af_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, c, d, a, b, x[10], 17, _mm256_set1_epi32((0xffff5bb1_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, b, c, d, a, x[11], 22, _mm256_set1_epi32((0x895cd7be_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, a, b, c, d, x[12], 7, _mm256_set1_epi32((0x6b901122_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, d, a, b, c, x[13], 12, _mm256_set1_epi32((0xfd987193_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, c, d, a, b, x[14], 17, _mm256_set1_epi32((0xa679438e_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, b, c, d, a, x[15], 22, _mm256_set1_epi32((0x49b40821_u32) as i32));


        // round 2 G
        MD5_AVX2_FUNCTION_G!(a, b, c, d, x[ 1], 5, _mm256_set1_epi32((0xf61e2562_u32) as i32));
        MD5_AVX2_FUNCTION_G!(d, a, b, c, x[ 6], 9, _mm256_set1_epi32((0xc040b340_u32) as i32));
        MD5_AVX2_FUNCTION_G!(c, d, a, b, x[11], 14, _mm256_set1_epi32((0x265e5a51_u32) as i32));
        MD5_AVX2_FUNCTION_G!(b, c, d, a, x[ 0], 20, _mm256_set1_epi32((0xe9b6c7aa_u32) as i32));
        MD5_AVX2_FUNCTION_G!(a, b, c, d, x[ 5], 5, _mm256_set1_epi32((0xd62f105d_u32) as i32));
        MD5_AVX2_FUNCTION_G!(d, a, b, c, x[10], 9, _mm256_set1_epi32(( 0x2441453_u32) as i32));
        MD5_AVX2_FUNCTION_G!(c, d, a, b, x[15], 14, _mm256_set1_epi32((0xd8a1e681_u32) as i32));
        MD5_AVX2_FUNCTION_G!(b, c, d, a, x[ 4], 20, _mm256_set1_epi32((0xe7d3fbc8_u32) as i32));
        MD5_AVX2_FUNCTION_G!(a, b, c, d, x[ 9], 5, _mm256_set1_epi32((0x21e1cde6_u32) as i32));
        MD5_AVX2_FUNCTION_G!(d, a, b, c, x[14], 9, _mm256_set1_epi32((0xc33707d6_u32) as i32));
        MD5_AVX2_FUNCTION_G!(c, d, a, b, x[ 3], 14, _mm256_set1_epi32((0xf4d50d87_u32) as i32));
        MD5_AVX2_FUNCTION_G!(b, c, d, a, x[ 8], 20, _mm256_set1_epi32((0x455a14ed_u32) as i32));
        MD5_AVX2_FUNCTION_G!(a, b, c, d, x[13], 5, _mm256_set1_epi32((0xa9e3e905_u32) as i32));
        MD5_AVX2_FUNCTION_G!(d, a, b, c, x[ 2], 9, _mm256_set1_epi32((0xfcefa3f8_u32) as i32));
        MD5_AVX2_FUNCTION_G!(c, d, a, b, x[ 7], 14, _mm256_set1_epi32((0x676f02d9_u32) as i32));
        MD5_AVX2_FUNCTION_G!(b, c, d, a, x[12], 20, _mm256_set1_epi32((0x8d2a4c8a_u32) as i32));

        // round 3 H
        MD5_AVX2_FUNCTION!(md5_h_avx2, a, b, c, d, x[ 5], 4, _mm256_set1_epi32((0xfffa3942_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, d, a, b, c, x[ 8], 11, _mm256_set1_epi32((0x8771f681_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, c, d, a, b, x[11], 16, _mm256_set1_epi32((0x6d9d6122_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, b, c, d, a, x[14], 23, _mm256_set1_epi32((0xfde5380c_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, a, b, c, d, x[ 1], 4, _mm256_set1_epi32((0xa4beea44_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, d, a, b, c, x[ 4], 11, _mm256_set1_epi32((0x4bdecfa9_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, c, d, a, b, x[ 7], 16, _mm256_set1_epi32((0xf6bb4b60_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, b, c, d, a, x[10], 23, _mm256_set1_epi32((0xbebfbc70_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, a, b, c, d, x[13], 4, _mm256_set1_epi32((0x289b7ec6_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, d, a, b, c, x[ 0], 11, _mm256_set1_epi32((0xeaa127fa_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, c, d, a, b, x[ 3], 16, _mm256_set1_epi32((0xd4ef3085_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, b, c, d, a, x[ 6], 23, _mm256_set1_epi32(( 0x4881d05_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, a, b, c, d, x[ 9], 4, _mm256_set1_epi32((0xd9d4d039_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, d, a, b, c, x[12], 11, _mm256_set1_epi32((0xe6db99e5_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, c, d, a, b, x[15], 16, _mm256_set1_epi32((0x1fa27cf8_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, b, c, d, a, x[ 2], 23, _mm256_set1_epi32((0xc4ac5665_u32) as i32));


        // round 4 I
        MD5_AVX2_FUNCTION!(md5_i_avx2, a, b, c, d, x[ 0], 6, _mm256_set1_epi32((0xf4292244_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, d, a, b, c, x[ 7], 10, _mm256_set1_epi32((0x432aff97_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, c, d, a, b, x[14], 15, _mm256_set1_epi32((0xab9423a7_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, b, c, d, a, x[ 5], 21, _mm256_set1_epi32((0xfc93a039_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, a, b, c, d, x[12], 6, _mm256_set1_epi32((0x655b59c3_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, d, a, b, c, x[ 3], 10, _mm256_set1_epi32((0x8f0ccc92_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, c, d, a, b, x[10], 15, _mm256_set1_epi32((0xffeff47d_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, b, c, d, a, x[ 1], 21, _mm256_set1_epi32((0x85845dd1_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, a, b, c, d, x[ 8], 6, _mm256_set1_epi32((0x6fa87e4f_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, d, a, b, c, x[15], 10, _mm256_set1_epi32((0xfe2ce6e0_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, c, d, a, b, x[ 6], 15, _mm256_set1_epi32((0xa3014314_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, b, c, d, a, x[13], 21, _mm256_set1_epi32((0x4e0811a1_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, a, b, c, d, x[ 4], 6, _mm256_set1_epi32((0xf7537e82_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, d, a, b, c, x[11], 10, _mm256_set1_epi32((0xbd3af235_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, c, d, a, b, x[ 2], 15, _mm256_set1_epi32((0x2ad7d2bb_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, b, c, d, a, x[ 9], 21, _mm256_set1_epi32((0xeb86d391_u32) as i32));

        self.state[0] = unsafe { _mm256_add_epi32(self.state[0], a) };
        self.state[1] = unsafe { _mm256_add_epi32(self.state[1], b) };
        self.state[2] = unsafe { _mm256_add_epi32(self.state[2], c) };
        self.state[3] = unsafe { _mm256_add_epi32(self.state[3], d) };
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
        MD5_AVX2_FUNCTION!(md5_f_avx2, a, b, c, d, x[ 0], 7, _mm256_set1_epi32((0xd76aa478_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, d, a, b, c, x[ 1], 12, _mm256_set1_epi32((0xe8c7b756_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, c, d, a, b, x[ 2], 17, _mm256_set1_epi32((0x242070db_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, b, c, d, a, x[ 3], 22, _mm256_set1_epi32((0xc1bdceee_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, a, b, c, d, x[ 4], 7, _mm256_set1_epi32((0xf57c0faf_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, d, a, b, c, x[ 5], 12, _mm256_set1_epi32((0x4787c62a_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, c, d, a, b, x[ 6], 17, _mm256_set1_epi32((0xa8304613_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, b, c, d, a, x[ 7], 22, _mm256_set1_epi32((0xfd469501_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, a, b, c, d, x[ 8], 7, _mm256_set1_epi32((0x698098d8_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, d, a, b, c, x[ 9], 12, _mm256_set1_epi32((0x8b44f7af_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, c, d, a, b, x[10], 17, _mm256_set1_epi32((0xffff5bb1_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, b, c, d, a, x[11], 22, _mm256_set1_epi32((0x895cd7be_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, a, b, c, d, x[12], 7, _mm256_set1_epi32((0x6b901122_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, d, a, b, c, x[13], 12, _mm256_set1_epi32((0xfd987193_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, c, d, a, b, x[14], 17, _mm256_set1_epi32((0xa679438e_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_f_avx2, b, c, d, a, x[15], 22, _mm256_set1_epi32((0x49b40821_u32) as i32));

        // round 2 G
        MD5_AVX2_FUNCTION_G!(a, b, c, d, x[ 1], 5, _mm256_set1_epi32((0xf61e2562_u32) as i32));
        MD5_AVX2_FUNCTION_G!(d, a, b, c, x[ 6], 9, _mm256_set1_epi32((0xc040b340_u32) as i32));
        MD5_AVX2_FUNCTION_G!(c, d, a, b, x[11], 14, _mm256_set1_epi32((0x265e5a51_u32) as i32));
        MD5_AVX2_FUNCTION_G!(b, c, d, a, x[ 0], 20, _mm256_set1_epi32((0xe9b6c7aa_u32) as i32));
        MD5_AVX2_FUNCTION_G!(a, b, c, d, x[ 5], 5, _mm256_set1_epi32((0xd62f105d_u32) as i32));
        MD5_AVX2_FUNCTION_G!(d, a, b, c, x[10], 9, _mm256_set1_epi32(( 0x2441453_u32) as i32));
        MD5_AVX2_FUNCTION_G!(c, d, a, b, x[15], 14, _mm256_set1_epi32((0xd8a1e681_u32) as i32));
        MD5_AVX2_FUNCTION_G!(b, c, d, a, x[ 4], 20, _mm256_set1_epi32((0xe7d3fbc8_u32) as i32));
        MD5_AVX2_FUNCTION_G!(a, b, c, d, x[ 9], 5, _mm256_set1_epi32((0x21e1cde6_u32) as i32));
        MD5_AVX2_FUNCTION_G!(d, a, b, c, x[14], 9, _mm256_set1_epi32((0xc33707d6_u32) as i32));
        MD5_AVX2_FUNCTION_G!(c, d, a, b, x[ 3], 14, _mm256_set1_epi32((0xf4d50d87_u32) as i32));
        MD5_AVX2_FUNCTION_G!(b, c, d, a, x[ 8], 20, _mm256_set1_epi32((0x455a14ed_u32) as i32));
        MD5_AVX2_FUNCTION_G!(a, b, c, d, x[13], 5, _mm256_set1_epi32((0xa9e3e905_u32) as i32));
        MD5_AVX2_FUNCTION_G!(d, a, b, c, x[ 2], 9, _mm256_set1_epi32((0xfcefa3f8_u32) as i32));
        MD5_AVX2_FUNCTION_G!(c, d, a, b, x[ 7], 14, _mm256_set1_epi32((0x676f02d9_u32) as i32));
        MD5_AVX2_FUNCTION_G!(b, c, d, a, x[12], 20, _mm256_set1_epi32((0x8d2a4c8a_u32) as i32));

        // round 3 H
        MD5_AVX2_FUNCTION!(md5_h_avx2, a, b, c, d, x[ 5], 4, _mm256_set1_epi32((0xfffa3942_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, d, a, b, c, x[ 8], 11, _mm256_set1_epi32((0x8771f681_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, c, d, a, b, x[11], 16, _mm256_set1_epi32((0x6d9d6122_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, b, c, d, a, x[14], 23, _mm256_set1_epi32((0xfde5380c_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, a, b, c, d, x[ 1], 4, _mm256_set1_epi32((0xa4beea44_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, d, a, b, c, x[ 4], 11, _mm256_set1_epi32((0x4bdecfa9_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, c, d, a, b, x[ 7], 16, _mm256_set1_epi32((0xf6bb4b60_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, b, c, d, a, x[10], 23, _mm256_set1_epi32((0xbebfbc70_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, a, b, c, d, x[13], 4, _mm256_set1_epi32((0x289b7ec6_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, d, a, b, c, x[ 0], 11, _mm256_set1_epi32((0xeaa127fa_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, c, d, a, b, x[ 3], 16, _mm256_set1_epi32((0xd4ef3085_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, b, c, d, a, x[ 6], 23, _mm256_set1_epi32(( 0x4881d05_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, a, b, c, d, x[ 9], 4, _mm256_set1_epi32((0xd9d4d039_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, d, a, b, c, x[12], 11, _mm256_set1_epi32((0xe6db99e5_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, c, d, a, b, x[15], 16, _mm256_set1_epi32((0x1fa27cf8_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_h_avx2, b, c, d, a, x[ 2], 23, _mm256_set1_epi32((0xc4ac5665_u32) as i32));

        // round 4 I
        MD5_AVX2_FUNCTION!(md5_i_avx2, a, b, c, d, x[ 0], 6, _mm256_set1_epi32((0xf4292244_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, d, a, b, c, x[ 7], 10, _mm256_set1_epi32((0x432aff97_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, c, d, a, b, x[14], 15, _mm256_set1_epi32((0xab9423a7_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, b, c, d, a, x[ 5], 21, _mm256_set1_epi32((0xfc93a039_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, a, b, c, d, x[12], 6, _mm256_set1_epi32((0x655b59c3_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, d, a, b, c, x[ 3], 10, _mm256_set1_epi32((0x8f0ccc92_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, c, d, a, b, x[10], 15, _mm256_set1_epi32((0xffeff47d_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, b, c, d, a, x[ 1], 21, _mm256_set1_epi32((0x85845dd1_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, a, b, c, d, x[ 8], 6, _mm256_set1_epi32((0x6fa87e4f_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, d, a, b, c, x[15], 10, _mm256_set1_epi32((0xfe2ce6e0_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, c, d, a, b, x[ 6], 15, _mm256_set1_epi32((0xa3014314_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, b, c, d, a, x[13], 21, _mm256_set1_epi32((0x4e0811a1_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, a, b, c, d, x[ 4], 6, _mm256_set1_epi32((0xf7537e82_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, d, a, b, c, x[11], 10, _mm256_set1_epi32((0xbd3af235_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, c, d, a, b, x[ 2], 15, _mm256_set1_epi32((0x2ad7d2bb_u32) as i32));
        MD5_AVX2_FUNCTION!(md5_i_avx2, b, c, d, a, x[ 9], 21, _mm256_set1_epi32((0xeb86d391_u32) as i32));

        self.state[0] = unsafe { _mm256_add_epi32(self.state[0], a) };
        self.state[1] = unsafe { _mm256_add_epi32(self.state[1], b) };
        self.state[2] = unsafe { _mm256_add_epi32(self.state[2], c) };
        self.state[3] = unsafe { _mm256_add_epi32(self.state[3], d) };
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
        let mut i;
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

            unsafe { core::intrinsics::copy_nonoverlapping(data1.as_ptr(), self.blocks[0][(index_usize)..].as_mut_ptr(), partial_len_usize); }
            unsafe { core::intrinsics::copy_nonoverlapping(data2.as_ptr(), self.blocks[1][(index_usize)..].as_mut_ptr(), partial_len_usize); }
            unsafe { core::intrinsics::copy_nonoverlapping(data3.as_ptr(), self.blocks[2][(index_usize)..].as_mut_ptr(), partial_len_usize); }
            unsafe { core::intrinsics::copy_nonoverlapping(data4.as_ptr(), self.blocks[3][(index_usize)..].as_mut_ptr(), partial_len_usize); }
            unsafe { core::intrinsics::copy_nonoverlapping(data5.as_ptr(), self.blocks[4][(index_usize)..].as_mut_ptr(), partial_len_usize); }
            unsafe { core::intrinsics::copy_nonoverlapping(data6.as_ptr(), self.blocks[5][(index_usize)..].as_mut_ptr(), partial_len_usize); }
            unsafe { core::intrinsics::copy_nonoverlapping(data7.as_ptr(), self.blocks[6][(index_usize)..].as_mut_ptr(), partial_len_usize); }
            unsafe { core::intrinsics::copy_nonoverlapping(data8.as_ptr(), self.blocks[7][(index_usize)..].as_mut_ptr(), partial_len_usize); }

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

        unsafe { core::intrinsics::copy_nonoverlapping(data1[i_usize..].as_ptr(), self.blocks[0][(index_usize)..].as_mut_ptr(), copy_n); }
        unsafe { core::intrinsics::copy_nonoverlapping(data2[i_usize..].as_ptr(), self.blocks[1][(index_usize)..].as_mut_ptr(), copy_n); }
        unsafe { core::intrinsics::copy_nonoverlapping(data3[i_usize..].as_ptr(), self.blocks[2][(index_usize)..].as_mut_ptr(), copy_n); }
        unsafe { core::intrinsics::copy_nonoverlapping(data4[i_usize..].as_ptr(), self.blocks[3][(index_usize)..].as_mut_ptr(), copy_n); }
        unsafe { core::intrinsics::copy_nonoverlapping(data5[i_usize..].as_ptr(), self.blocks[4][(index_usize)..].as_mut_ptr(), copy_n); }
        unsafe { core::intrinsics::copy_nonoverlapping(data6[i_usize..].as_ptr(), self.blocks[5][(index_usize)..].as_mut_ptr(), copy_n); }
        unsafe { core::intrinsics::copy_nonoverlapping(data7[i_usize..].as_ptr(), self.blocks[6][(index_usize)..].as_mut_ptr(), copy_n); }
        unsafe { core::intrinsics::copy_nonoverlapping(data8[i_usize..].as_ptr(), self.blocks[7][(index_usize)..].as_mut_ptr(), copy_n); }
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
                encode_32_avx2(&split_m256i[i..], &mut digests[i][(j*4)..]);
            }
        }

        digests
    }

    pub fn digest(input: &[u8]) -> [[u8; 16]; 8] {
        let mut object = Self::default();
        object.update1(input);
        object.finalize()
    }

    pub fn digest2(input1: &[u8], input2: &[u8]) -> [[u8; 16]; 8] {
        let mut object = Self::default();
        object.update2(input1, input2);
        object.finalize()
    }

    pub fn digest3(input1: &[u8], input2: &[u8], input3: &[u8]) -> [[u8; 16]; 8] {
        let mut object = Self::default();
        object.update3(input1, input2, input3);
        object.finalize()
    }

    pub fn digest4(input1: &[u8], input2: &[u8], input3: &[u8], input4: &[u8]) -> [[u8; 16]; 8] {
        let mut object = Self::default();
        object.update4(input1, input2, input3, input4);
        object.finalize()
    }

    pub fn digest5(input1: &[u8], input2: &[u8], input3: &[u8], input4: &[u8], input5: &[u8]) -> [[u8; 16]; 8] {
        let mut object = Self::default();
        object.update5(input1, input2, input3, input4, input5);
        object.finalize()
    }

    pub fn digest6(input1: &[u8], input2: &[u8], input3: &[u8], input4: &[u8], input5: &[u8], input6: &[u8]) -> [[u8; 16]; 8] {
        let mut object = Self::default();
        object.update6(input1, input2, input3, input4, input5, input6);
        object.finalize()
    }

    pub fn digest7(input1: &[u8], input2: &[u8], input3: &[u8], input4: &[u8], input5: &[u8], input6: &[u8], input7: &[u8]) -> [[u8; 16]; 8] {
        let mut object = Self::default();
        object.update7(input1, input2, input3, input4, input5, input6, input7);
        object.finalize()
    }

    pub fn digest8(input1: &[u8],input2: &[u8],input3: &[u8],input4: &[u8],input5: &[u8],input6: &[u8],input7: &[u8],input8: &[u8]) -> [[u8; 16]; 8] {
        let mut object = Self::default();
        object.update8(input1, input2, input3, input4, input5, input6, input7, input8);
        object.finalize()
    }
}

impl Default for Md5Avx2 {
    fn default() -> Self {
        Self {
            state: [
                unsafe { _mm256_set1_epi32((0x67452301_u32) as i32) },
                unsafe { _mm256_set1_epi32((0xefcdab89_u32) as i32) },
                unsafe { _mm256_set1_epi32((0x98badcfe_u32) as i32) },
                unsafe { _mm256_set1_epi32((0x10325476_u32) as i32) },
            ],
            count: [0, 0],
            blocks: [[0; 64]; 8],
        }
    }
}



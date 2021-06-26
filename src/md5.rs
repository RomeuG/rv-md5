use crate::utils::*;

macro_rules! MD5_FUNCTION {
    ($f:ident, $a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr, $ac:expr) => {
        let res = $f($b, $c, $d);

        $a = $a.wrapping_add(res.wrapping_add($x.wrapping_add($ac)));
        $a = rotate_left($a, $s);
        $a = $a.wrapping_add($b);
    }
}

macro_rules! MD5_FUNCTION_G {
    ($a:expr, $b:expr, $c:expr, $d:expr, $x:expr, $s:expr, $ac:expr) => {
        $a = $a.wrapping_add($x.wrapping_add($ac));
        $a = $a.wrapping_add(!$d & $c);
        $a = $a.wrapping_add($d & $b);
        $a = rotate_left($a, $s);
        $a = $a.wrapping_add($b);
    }
}

#[inline(always)]
fn md5_f(x: u32, y: u32, z: u32) -> u32 {
    (x & y) | (!x & z)
}

#[inline(always)]
fn md5_h(x: u32, y: u32, z: u32) -> u32 {
    x ^ y ^ z
}

#[inline(always)]
fn md5_i(x: u32, y: u32, z: u32) -> u32 {
    y ^ (x | !z)
}


pub struct Md5 {
    state: [u32; 4],
    count: [u64; 2],
    block: [u8; 64],
}

impl Md5 {
    fn transform_blocks(&mut self) {
        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];

        let mut x: [u32; 16] = [0; 16];

        for (index, index2) in [0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60].iter().enumerate() {

            let index2_u = *index2 as usize;
            x[index] = (self.block[index2_u] as u32) | ((self.block[index2_u+1] as u32) << 8) | ((self.block[index2_u+2] as u32) << 16) | ((self.block[index2_u+3] as u32) << 24);
        }

        // round 1 F
        MD5_FUNCTION!(md5_f, a, b, c, d, x[ 0], 7, 0xd76aa478_u32);
        MD5_FUNCTION!(md5_f, d, a, b, c, x[ 1], 12, 0xe8c7b756_u32);
        MD5_FUNCTION!(md5_f, c, d, a, b, x[ 2], 17, 0x242070db_u32);
        MD5_FUNCTION!(md5_f, b, c, d, a, x[ 3], 22, 0xc1bdceee_u32);
        MD5_FUNCTION!(md5_f, a, b, c, d, x[ 4], 7, 0xf57c0faf_u32);
        MD5_FUNCTION!(md5_f, d, a, b, c, x[ 5], 12, 0x4787c62a_u32);
        MD5_FUNCTION!(md5_f, c, d, a, b, x[ 6], 17, 0xa8304613_u32);
        MD5_FUNCTION!(md5_f, b, c, d, a, x[ 7], 22, 0xfd469501_u32);
        MD5_FUNCTION!(md5_f, a, b, c, d, x[ 8], 7, 0x698098d8_u32);
        MD5_FUNCTION!(md5_f, d, a, b, c, x[ 9], 12, 0x8b44f7af_u32);
        MD5_FUNCTION!(md5_f, c, d, a, b, x[10], 17, 0xffff5bb1_u32);
        MD5_FUNCTION!(md5_f, b, c, d, a, x[11], 22, 0x895cd7be_u32);
        MD5_FUNCTION!(md5_f, a, b, c, d, x[12], 7, 0x6b901122_u32);
        MD5_FUNCTION!(md5_f, d, a, b, c, x[13], 12, 0xfd987193_u32);
        MD5_FUNCTION!(md5_f, c, d, a, b, x[14], 17, 0xa679438e_u32);
        MD5_FUNCTION!(md5_f, b, c, d, a, x[15], 22, 0x49b40821_u32);


        // round 2 G
        MD5_FUNCTION_G!(a, b, c, d, x[ 1], 5, 0xf61e2562_u32);
        MD5_FUNCTION_G!(d, a, b, c, x[ 6], 9, 0xc040b340_u32);
        MD5_FUNCTION_G!(c, d, a, b, x[11], 14, 0x265e5a51_u32);
        MD5_FUNCTION_G!(b, c, d, a, x[ 0], 20, 0xe9b6c7aa_u32);
        MD5_FUNCTION_G!(a, b, c, d, x[ 5], 5, 0xd62f105d_u32);
        MD5_FUNCTION_G!(d, a, b, c, x[10], 9,  0x2441453_u32);
        MD5_FUNCTION_G!(c, d, a, b, x[15], 14, 0xd8a1e681_u32);
        MD5_FUNCTION_G!(b, c, d, a, x[ 4], 20, 0xe7d3fbc8_u32);
        MD5_FUNCTION_G!(a, b, c, d, x[ 9], 5, 0x21e1cde6_u32);
        MD5_FUNCTION_G!(d, a, b, c, x[14], 9, 0xc33707d6_u32);
        MD5_FUNCTION_G!(c, d, a, b, x[ 3], 14, 0xf4d50d87_u32);
        MD5_FUNCTION_G!(b, c, d, a, x[ 8], 20, 0x455a14ed_u32);
        MD5_FUNCTION_G!(a, b, c, d, x[13], 5, 0xa9e3e905_u32);
        MD5_FUNCTION_G!(d, a, b, c, x[ 2], 9, 0xfcefa3f8_u32);
        MD5_FUNCTION_G!(c, d, a, b, x[ 7], 14, 0x676f02d9_u32);
        MD5_FUNCTION_G!(b, c, d, a, x[12], 20, 0x8d2a4c8a_u32);

        // round 3 H
        MD5_FUNCTION!(md5_h, a, b, c, d, x[ 5], 4, 0xfffa3942_u32);
        MD5_FUNCTION!(md5_h, d, a, b, c, x[ 8], 11, 0x8771f681_u32);
        MD5_FUNCTION!(md5_h, c, d, a, b, x[11], 16, 0x6d9d6122_u32);
        MD5_FUNCTION!(md5_h, b, c, d, a, x[14], 23, 0xfde5380c_u32);
        MD5_FUNCTION!(md5_h, a, b, c, d, x[ 1], 4, 0xa4beea44_u32);
        MD5_FUNCTION!(md5_h, d, a, b, c, x[ 4], 11, 0x4bdecfa9_u32);
        MD5_FUNCTION!(md5_h, c, d, a, b, x[ 7], 16, 0xf6bb4b60_u32);
        MD5_FUNCTION!(md5_h, b, c, d, a, x[10], 23, 0xbebfbc70_u32);
        MD5_FUNCTION!(md5_h, a, b, c, d, x[13], 4, 0x289b7ec6_u32);
        MD5_FUNCTION!(md5_h, d, a, b, c, x[ 0], 11, 0xeaa127fa_u32);
        MD5_FUNCTION!(md5_h, c, d, a, b, x[ 3], 16, 0xd4ef3085_u32);
        MD5_FUNCTION!(md5_h, b, c, d, a, x[ 6], 23,  0x4881d05_u32);
        MD5_FUNCTION!(md5_h, a, b, c, d, x[ 9], 4, 0xd9d4d039_u32);
        MD5_FUNCTION!(md5_h, d, a, b, c, x[12], 11, 0xe6db99e5_u32);
        MD5_FUNCTION!(md5_h, c, d, a, b, x[15], 16, 0x1fa27cf8_u32);
        MD5_FUNCTION!(md5_h, b, c, d, a, x[ 2], 23, 0xc4ac5665_u32);

        // round 4 I
        MD5_FUNCTION!(md5_i, a, b, c, d, x[ 0], 6, 0xf4292244_u32);
        MD5_FUNCTION!(md5_i, d, a, b, c, x[ 7], 10, 0x432aff97_u32);
        MD5_FUNCTION!(md5_i, c, d, a, b, x[14], 15, 0xab9423a7_u32);
        MD5_FUNCTION!(md5_i, b, c, d, a, x[ 5], 21, 0xfc93a039_u32);
        MD5_FUNCTION!(md5_i, a, b, c, d, x[12], 6, 0x655b59c3_u32);
        MD5_FUNCTION!(md5_i, d, a, b, c, x[ 3], 10, 0x8f0ccc92_u32);
        MD5_FUNCTION!(md5_i, c, d, a, b, x[10], 15, 0xffeff47d_u32);
        MD5_FUNCTION!(md5_i, b, c, d, a, x[ 1], 21, 0x85845dd1_u32);
        MD5_FUNCTION!(md5_i, a, b, c, d, x[ 8], 6, 0x6fa87e4f_u32);
        MD5_FUNCTION!(md5_i, d, a, b, c, x[15], 10, 0xfe2ce6e0_u32);
        MD5_FUNCTION!(md5_i, c, d, a, b, x[ 6], 15, 0xa3014314_u32);
        MD5_FUNCTION!(md5_i, b, c, d, a, x[13], 21, 0x4e0811a1_u32);
        MD5_FUNCTION!(md5_i, a, b, c, d, x[ 4], 6, 0xf7537e82_u32);
        MD5_FUNCTION!(md5_i, d, a, b, c, x[11], 10, 0xbd3af235_u32);
        MD5_FUNCTION!(md5_i, c, d, a, b, x[ 2], 15, 0x2ad7d2bb_u32);
        MD5_FUNCTION!(md5_i, b, c, d, a, x[ 9], 21, 0xeb86d391_u32);

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
    }

    fn transform(&mut self, block: &[u8]) {
        let mut a = self.state[0];
        let mut b = self.state[1];
        let mut c = self.state[2];
        let mut d = self.state[3];

        let mut x: [u32; 16] = [0; 16];

        for (index, index2) in [0, 4, 8, 12, 16, 20, 24, 28, 32, 36, 40, 44, 48, 52, 56, 60].iter().enumerate() {

            let index2_u = *index2 as usize;
            x[index] = (block[index2_u] as u32) | ((block[index2_u+1] as u32) << 8) | ((block[index2_u+2] as u32) << 16) | ((block[index2_u+3] as u32) << 24);
        }

        // round 1 F
        MD5_FUNCTION!(md5_f, a, b, c, d, x[ 0], 7, 0xd76aa478_u32);
        MD5_FUNCTION!(md5_f, d, a, b, c, x[ 1], 12, 0xe8c7b756_u32);
        MD5_FUNCTION!(md5_f, c, d, a, b, x[ 2], 17, 0x242070db_u32);
        MD5_FUNCTION!(md5_f, b, c, d, a, x[ 3], 22, 0xc1bdceee_u32);
        MD5_FUNCTION!(md5_f, a, b, c, d, x[ 4], 7, 0xf57c0faf_u32);
        MD5_FUNCTION!(md5_f, d, a, b, c, x[ 5], 12, 0x4787c62a_u32);
        MD5_FUNCTION!(md5_f, c, d, a, b, x[ 6], 17, 0xa8304613_u32);
        MD5_FUNCTION!(md5_f, b, c, d, a, x[ 7], 22, 0xfd469501_u32);
        MD5_FUNCTION!(md5_f, a, b, c, d, x[ 8], 7, 0x698098d8_u32);
        MD5_FUNCTION!(md5_f, d, a, b, c, x[ 9], 12, 0x8b44f7af_u32);
        MD5_FUNCTION!(md5_f, c, d, a, b, x[10], 17, 0xffff5bb1_u32);
        MD5_FUNCTION!(md5_f, b, c, d, a, x[11], 22, 0x895cd7be_u32);
        MD5_FUNCTION!(md5_f, a, b, c, d, x[12], 7, 0x6b901122_u32);
        MD5_FUNCTION!(md5_f, d, a, b, c, x[13], 12, 0xfd987193_u32);
        MD5_FUNCTION!(md5_f, c, d, a, b, x[14], 17, 0xa679438e_u32);
        MD5_FUNCTION!(md5_f, b, c, d, a, x[15], 22, 0x49b40821_u32);

        // round 2 G
        MD5_FUNCTION_G!(a, b, c, d, x[ 1], 5, 0xf61e2562_u32);
        MD5_FUNCTION_G!(d, a, b, c, x[ 6], 9, 0xc040b340_u32);
        MD5_FUNCTION_G!(c, d, a, b, x[11], 14, 0x265e5a51_u32);
        MD5_FUNCTION_G!(b, c, d, a, x[ 0], 20, 0xe9b6c7aa_u32);
        MD5_FUNCTION_G!(a, b, c, d, x[ 5], 5, 0xd62f105d_u32);
        MD5_FUNCTION_G!(d, a, b, c, x[10], 9,  0x2441453_u32);
        MD5_FUNCTION_G!(c, d, a, b, x[15], 14, 0xd8a1e681_u32);
        MD5_FUNCTION_G!(b, c, d, a, x[ 4], 20, 0xe7d3fbc8_u32);
        MD5_FUNCTION_G!(a, b, c, d, x[ 9], 5, 0x21e1cde6_u32);
        MD5_FUNCTION_G!(d, a, b, c, x[14], 9, 0xc33707d6_u32);
        MD5_FUNCTION_G!(c, d, a, b, x[ 3], 14, 0xf4d50d87_u32);
        MD5_FUNCTION_G!(b, c, d, a, x[ 8], 20, 0x455a14ed_u32);
        MD5_FUNCTION_G!(a, b, c, d, x[13], 5, 0xa9e3e905_u32);
        MD5_FUNCTION_G!(d, a, b, c, x[ 2], 9, 0xfcefa3f8_u32);
        MD5_FUNCTION_G!(c, d, a, b, x[ 7], 14, 0x676f02d9_u32);
        MD5_FUNCTION_G!(b, c, d, a, x[12], 20, 0x8d2a4c8a_u32);

        // round 3 H
        MD5_FUNCTION!(md5_h, a, b, c, d, x[ 5], 4, 0xfffa3942_u32);
        MD5_FUNCTION!(md5_h, d, a, b, c, x[ 8], 11, 0x8771f681_u32);
        MD5_FUNCTION!(md5_h, c, d, a, b, x[11], 16, 0x6d9d6122_u32);
        MD5_FUNCTION!(md5_h, b, c, d, a, x[14], 23, 0xfde5380c_u32);
        MD5_FUNCTION!(md5_h, a, b, c, d, x[ 1], 4, 0xa4beea44_u32);
        MD5_FUNCTION!(md5_h, d, a, b, c, x[ 4], 11, 0x4bdecfa9_u32);
        MD5_FUNCTION!(md5_h, c, d, a, b, x[ 7], 16, 0xf6bb4b60_u32);
        MD5_FUNCTION!(md5_h, b, c, d, a, x[10], 23, 0xbebfbc70_u32);
        MD5_FUNCTION!(md5_h, a, b, c, d, x[13], 4, 0x289b7ec6_u32);
        MD5_FUNCTION!(md5_h, d, a, b, c, x[ 0], 11, 0xeaa127fa_u32);
        MD5_FUNCTION!(md5_h, c, d, a, b, x[ 3], 16, 0xd4ef3085_u32);
        MD5_FUNCTION!(md5_h, b, c, d, a, x[ 6], 23,  0x4881d05_u32);
        MD5_FUNCTION!(md5_h, a, b, c, d, x[ 9], 4, 0xd9d4d039_u32);
        MD5_FUNCTION!(md5_h, d, a, b, c, x[12], 11, 0xe6db99e5_u32);
        MD5_FUNCTION!(md5_h, c, d, a, b, x[15], 16, 0x1fa27cf8_u32);
        MD5_FUNCTION!(md5_h, b, c, d, a, x[ 2], 23, 0xc4ac5665_u32);

        // round 4 I
        MD5_FUNCTION!(md5_i, a, b, c, d, x[ 0], 6, 0xf4292244_u32);
        MD5_FUNCTION!(md5_i, d, a, b, c, x[ 7], 10, 0x432aff97_u32);
        MD5_FUNCTION!(md5_i, c, d, a, b, x[14], 15, 0xab9423a7_u32);
        MD5_FUNCTION!(md5_i, b, c, d, a, x[ 5], 21, 0xfc93a039_u32);
        MD5_FUNCTION!(md5_i, a, b, c, d, x[12], 6, 0x655b59c3_u32);
        MD5_FUNCTION!(md5_i, d, a, b, c, x[ 3], 10, 0x8f0ccc92_u32);
        MD5_FUNCTION!(md5_i, c, d, a, b, x[10], 15, 0xffeff47d_u32);
        MD5_FUNCTION!(md5_i, b, c, d, a, x[ 1], 21, 0x85845dd1_u32);
        MD5_FUNCTION!(md5_i, a, b, c, d, x[ 8], 6, 0x6fa87e4f_u32);
        MD5_FUNCTION!(md5_i, d, a, b, c, x[15], 10, 0xfe2ce6e0_u32);
        MD5_FUNCTION!(md5_i, c, d, a, b, x[ 6], 15, 0xa3014314_u32);
        MD5_FUNCTION!(md5_i, b, c, d, a, x[13], 21, 0x4e0811a1_u32);
        MD5_FUNCTION!(md5_i, a, b, c, d, x[ 4], 6, 0xf7537e82_u32);
        MD5_FUNCTION!(md5_i, d, a, b, c, x[11], 10, 0xbd3af235_u32);
        MD5_FUNCTION!(md5_i, c, d, a, b, x[ 2], 15, 0x2ad7d2bb_u32);
        MD5_FUNCTION!(md5_i, b, c, d, a, x[ 9], 21, 0xeb86d391_u32);

        self.state[0] = self.state[0].wrapping_add(a);
        self.state[1] = self.state[1].wrapping_add(b);
        self.state[2] = self.state[2].wrapping_add(c);
        self.state[3] = self.state[3].wrapping_add(d);
    }

    fn update(&mut self, data: &[u8]) {
        let mut i;
        let mut index = (self.count[0] >> 3) & 0x3F;
        let data_len = data.len();

        self.count[0] += (data_len << 3) as u64;
        if self.count[0] < (data_len << 3) as u64 {
            self.count[1] += 1;
        }

        self.count[1] += (data_len >> 29) as u64;

        let partial_len = 64 - index;
        let partial_len_usize = partial_len as usize;

        if data_len >= partial_len_usize {

            let index_usize = index as usize;

            unsafe { core::intrinsics::copy_nonoverlapping(data.as_ptr(), self.block[(index_usize)..].as_mut_ptr(), partial_len_usize); }

            self.transform_blocks();

            i = partial_len;
            while i + 63 < data_len as u64 {

                let subindex = i as usize;

                self.transform(
                    &data[subindex..],
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

        unsafe { core::intrinsics::copy_nonoverlapping(data[i_usize..].as_ptr(), self.block[(index_usize)..].as_mut_ptr(), copy_n); }
    }

    fn finalize(&mut self) -> [u8; 16] {
        let mut bits: [u8; 8] = [0; 8];

        encode(&self.count[..], &mut bits[..]);

        let index = (self.count[0] >> 3) & 0x3f;
        let padlen = if index < 56 { 56 - index } else { 120 - index };

        self.update(&PAD[..(padlen as usize)]);
        self.update(&bits);

        let mut digests: [u8; 16] = [0;16];

        for j in 0..4 {
            encode_32(&self.state[j], &mut digests[(j*4)..]);
        }

        digests
    }

    pub fn digest(input: &[u8]) -> [u8; 16] {
        let mut object = Self::default();
        object.update(input);
        object.finalize()
    }
}

impl Default for Md5 {
    fn default() -> Self {
        Self {
            state: [
                0x67452301_u32,
                0xefcdab89_u32,
                0x98badcfe_u32,
                0x10325476_u32,
            ],
            count: [0, 0],
            block: [0; 64],
        }
    }
}



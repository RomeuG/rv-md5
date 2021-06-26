#![no_std]

#![feature(test)]
#![feature(stmt_expr_attributes)]

#![allow(clippy::many_single_char_names)]
#![allow(clippy::too_many_arguments)]

pub mod utils;
pub mod md5;
pub mod md5_avx2;

use md5::Md5;
use md5_avx2::Md5Avx2;

#[cfg(test)]
mod tests {
    use super::*;

    extern crate test;
    use test::Bencher;

    #[test]
    fn md5_32bytes() {
        let hash = Md5::digest("wqvDrDLilCUevxUw5fWEuVc6y6ElCrHg".as_bytes());
        assert_eq!(hash, [0x69, 0xca, 0x36, 0x72, 0x1d, 0xd0, 0x20, 0x97, 0x82, 0xc3, 0x76, 0x17, 0xf2, 0x20, 0xab, 0x82]);
    }

    #[test]
    fn md5_64bytes() {
        let hash = Md5::digest("K7CN3VzXyY63NXmW15TKA4O6vJtVrLc7I0B5qHRtBir5PkwSt6xgJopOCunPk2ky".as_bytes());
        assert_eq!(hash, [0x6f, 0xbe, 0xd0, 0x7f, 0xcc, 0x87, 0x90, 0xb2, 0xcb, 0x56, 0x03, 0x02, 0xa5, 0xc7, 0x5e, 0x56]);
    }

    #[test]
    fn md5_128bytes() {
        let hash = Md5::digest("KAjb6sifm7DwdyJyMXT3np6WZVfXJiEskX1fN7V8YOatxuRkpHYZmqDXY2Kn2pfnV63l0bodaXjRdVF5m2z1bC7QpdQi3UHRI9KAqWs0vO0QjT5XtkTXKlaRK4CiBsT1".as_bytes());
        assert_eq!(hash, [0x59, 0xa4, 0x18, 0xec, 0xcd, 0xff, 0x43, 0xeb, 0xfa, 0xb1, 0x63, 0x91, 0x6c, 0x60, 0x2c, 0x15]);
    }

    #[test]
    fn md5_256bytes() {
        let hash = Md5::digest("QnpFg2P1SEQ0L9tcNwBROCW7jVtFeMt0RuF7QODKkgD75CPDi1pAB1GtMcq0G1pmNE6J3IuPpF33uPtOs4sNwU7lKcnF8SU016PKWPeVEpuKQ2ksT9enIf1hVrzlypOkhFTFhIS28IT9OQZ3BS3693487mSb6QNuuaBCD8yNWWlo74c79EFWUWNaAmRcSxVaNcbDa80SovlnL8lyO2yS7XlmE7rPmLI4IvPtko3QguI4Th2JPrVnM7QCCjMgvlIO".as_bytes());
        assert_eq!(hash, [0xe5, 0x0b, 0xe7, 0x33, 0x07, 0x72, 0xe0, 0x89, 0xf6, 0x5e, 0x0e, 0xfd, 0x98, 0x69, 0xc9, 0xe3]);
    }

    #[test]
    fn md5_avx2_32bytes() {
        let hash = Md5Avx2::digest("wqvDrDLilCUevxUw5fWEuVc6y6ElCrHg".as_bytes());
        assert_eq!(hash[0], [0x69, 0xca, 0x36, 0x72, 0x1d, 0xd0, 0x20, 0x97, 0x82, 0xc3, 0x76, 0x17, 0xf2, 0x20, 0xab, 0x82]);
    }

    #[test]
    fn md5_avx2_64bytes() {
        let hash = Md5Avx2::digest("K7CN3VzXyY63NXmW15TKA4O6vJtVrLc7I0B5qHRtBir5PkwSt6xgJopOCunPk2ky".as_bytes());
        assert_eq!(hash[0], [0x6f, 0xbe, 0xd0, 0x7f, 0xcc, 0x87, 0x90, 0xb2, 0xcb, 0x56, 0x03, 0x02, 0xa5, 0xc7, 0x5e, 0x56]);
    }

    #[test]
    fn md5_avx2_128bytes() {
        let hash = Md5Avx2::digest("KAjb6sifm7DwdyJyMXT3np6WZVfXJiEskX1fN7V8YOatxuRkpHYZmqDXY2Kn2pfnV63l0bodaXjRdVF5m2z1bC7QpdQi3UHRI9KAqWs0vO0QjT5XtkTXKlaRK4CiBsT1".as_bytes());
        assert_eq!(hash[0], [0x59, 0xa4, 0x18, 0xec, 0xcd, 0xff, 0x43, 0xeb, 0xfa, 0xb1, 0x63, 0x91, 0x6c, 0x60, 0x2c, 0x15]);
    }

    #[test]
    fn md5_avx2_256bytes() {
        let hash = Md5Avx2::digest("QnpFg2P1SEQ0L9tcNwBROCW7jVtFeMt0RuF7QODKkgD75CPDi1pAB1GtMcq0G1pmNE6J3IuPpF33uPtOs4sNwU7lKcnF8SU016PKWPeVEpuKQ2ksT9enIf1hVrzlypOkhFTFhIS28IT9OQZ3BS3693487mSb6QNuuaBCD8yNWWlo74c79EFWUWNaAmRcSxVaNcbDa80SovlnL8lyO2yS7XlmE7rPmLI4IvPtko3QguI4Th2JPrVnM7QCCjMgvlIO".as_bytes());
        assert_eq!(hash[0], [0xe5, 0x0b, 0xe7, 0x33, 0x07, 0x72, 0xe0, 0x89, 0xf6, 0x5e, 0x0e, 0xfd, 0x98, 0x69, 0xc9, 0xe3]);
    }

    #[bench]
    fn bench_md5_32bytes_digest(b: &mut Bencher) {
        b.iter(|| {
            Md5::digest("wqvDrDLilCUevxUw5fWEuVc6y6ElCrHg".as_bytes());
        });
    }

    #[bench]
    fn bench_md5_64bytes_digest(b: &mut Bencher) {
        b.iter(|| {
            Md5::digest("K7CN3VzXyY63NXmW15TKA4O6vJtVrLc7I0B5qHRtBir5PkwSt6xgJopOCunPk2ky".as_bytes());
        });
    }

    #[bench]
    fn bench_md5_128bytes_digest(b: &mut Bencher) {
        b.iter(|| {
            Md5::digest("KAjb6sifm7DwdyJyMXT3np6WZVfXJiEskX1fN7V8YOatxuRkpHYZmqDXY2Kn2pfnV63l0bodaXjRdVF5m2z1bC7QpdQi3UHRI9KAqWs0vO0QjT5XtkTXKlaRK4CiBsT1".as_bytes());
        });
    }

    #[bench]
    fn bench_md5_256bytes_digest(b: &mut Bencher) {
        b.iter(|| {
            Md5::digest("QnpFg2P1SEQ0L9tcNwBROCW7jVtFeMt0RuF7QODKkgD75CPDi1pAB1GtMcq0G1pmNE6J3IuPpF33uPtOs4sNwU7lKcnF8SU016PKWPeVEpuKQ2ksT9enIf1hVrzlypOkhFTFhIS28IT9OQZ3BS3693487mSb6QNuuaBCD8yNWWlo74c79EFWUWNaAmRcSxVaNcbDa80SovlnL8lyO2yS7XlmE7rPmLI4IvPtko3QguI4Th2JPrVnM7QCCjMgvlIO".as_bytes());
        });
    }

    #[bench]
    fn bench_md5_avx2_32bytes_digest(b: &mut Bencher) {
        b.iter(|| {
            Md5Avx2::digest("wqvDrDLilCUevxUw5fWEuVc6y6ElCrHg".as_bytes());
        });
    }

    #[bench]
    fn bench_md5_avx2_64bytes_digest(b: &mut Bencher) {
        b.iter(|| {
            Md5Avx2::digest("K7CN3VzXyY63NXmW15TKA4O6vJtVrLc7I0B5qHRtBir5PkwSt6xgJopOCunPk2ky".as_bytes());
        });
    }

    #[bench]
    fn bench_md5_avx2_128bytes_digest(b: &mut Bencher) {
        b.iter(|| {
            Md5Avx2::digest("KAjb6sifm7DwdyJyMXT3np6WZVfXJiEskX1fN7V8YOatxuRkpHYZmqDXY2Kn2pfnV63l0bodaXjRdVF5m2z1bC7QpdQi3UHRI9KAqWs0vO0QjT5XtkTXKlaRK4CiBsT1".as_bytes());
        });
    }

    #[bench]
    fn bench_md5_avx2_256bytes_digest(b: &mut Bencher) {
        b.iter(|| {
            Md5Avx2::digest("QnpFg2P1SEQ0L9tcNwBROCW7jVtFeMt0RuF7QODKkgD75CPDi1pAB1GtMcq0G1pmNE6J3IuPpF33uPtOs4sNwU7lKcnF8SU016PKWPeVEpuKQ2ksT9enIf1hVrzlypOkhFTFhIS28IT9OQZ3BS3693487mSb6QNuuaBCD8yNWWlo74c79EFWUWNaAmRcSxVaNcbDa80SovlnL8lyO2yS7XlmE7rPmLI4IvPtko3QguI4Th2JPrVnM7QCCjMgvlIO".as_bytes());
        });
    }
}


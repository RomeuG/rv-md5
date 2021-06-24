MD5 in Rust
===============

This is an implementation of the MD5 hashing algorithm in Rust, without the use of the
Rust Standard Library. It includes a standard implementation and an AVX2 implementation.

Examples
========

This is the content of `examples/md5.rs`, using the normal RFC1321 MD5 implementation with
some changes:

```rust
use md5::Md5;

/// convert bytes to hex string
/// code taken from hex project: https://docs.rs/crate/hex/0.1.0/source/src/lib.rs
fn to_hex_string(data: &[u8]) -> String {
    static CHARS: &'static [u8] = b"0123456789abcdef";

    let bytes = data.as_ref();
    let mut v = Vec::with_capacity(bytes.len() * 2);
    for &byte in bytes.iter() {
        v.push(CHARS[(byte >> 4) as usize]);
        v.push(CHARS[(byte & 0xf) as usize]);
    }

    unsafe {
        String::from_utf8_unchecked(v)
    }
}

fn main() {
    let test_str1 = "this";
    let test_str2 = "this is";
    let test_str3 = "this is a";
    let test_str4 = "this is a test";
    let test_str5 = "this is a test string";
    let test_str6 = "this is a test string to";
    let test_str7 = "this is a test string to test";
    let test_str8 = "this is a test string to test md5";

    let hash1 = Md5::digest(test_str1.as_bytes());
    let hash2 = Md5::digest(test_str2.as_bytes());
    let hash3 = Md5::digest(test_str3.as_bytes());
    let hash4 = Md5::digest(test_str4.as_bytes());
    let hash5 = Md5::digest(test_str5.as_bytes());
    let hash6 = Md5::digest(test_str6.as_bytes());
    let hash7 = Md5::digest(test_str7.as_bytes());
    let hash8 = Md5::digest(test_str8.as_bytes());

    println!("{: <33} => {: <33}", test_str1, to_hex_string(&hash1));
    println!("{: <33} => {: <33}", test_str2, to_hex_string(&hash2));
    println!("{: <33} => {: <33}", test_str3, to_hex_string(&hash3));
    println!("{: <33} => {: <33}", test_str4, to_hex_string(&hash4));
    println!("{: <33} => {: <33}", test_str5, to_hex_string(&hash5));
    println!("{: <33} => {: <33}", test_str6, to_hex_string(&hash6));
    println!("{: <33} => {: <33}", test_str7, to_hex_string(&hash7));
    println!("{: <33} => {: <33}", test_str8, to_hex_string(&hash8));
}
```

If your CPU supports AVX2, youy may want to take advantage of it by using the AVX2
specific MD5 class, that is exemplified in `examples/md5_avx2.rs`:

```
use md5::Md5Avx2;

/// convert bytes to hex string
/// code taken from hex project: https://docs.rs/crate/hex/0.1.0/source/src/lib.rs
fn to_hex_string(data: &[u8]) -> String {
    static CHARS: &'static [u8] = b"0123456789abcdef";

    let bytes = data.as_ref();
    let mut v = Vec::with_capacity(bytes.len() * 2);
    for &byte in bytes.iter() {
        v.push(CHARS[(byte >> 4) as usize]);
        v.push(CHARS[(byte & 0xf) as usize]);
    }

    unsafe {
        String::from_utf8_unchecked(v)
    }
}

fn main() {
    let test_strs = vec![
        "this is a test string 1",
        "this is a test string 2",
        "this is a test string 3",
        "this is a test string 4",
        "this is a test string 5",
        "this is a test string 6",
        "this is a test string 7",
        "this is a test string 8",
    ];

    let hash = Md5Avx2::digest8(
        &test_strs[0].as_bytes(),
        &test_strs[1].as_bytes(),
        &test_strs[2].as_bytes(),
        &test_strs[3].as_bytes(),
        &test_strs[4].as_bytes(),
        &test_strs[5].as_bytes(),
        &test_strs[6].as_bytes(),
        &test_strs[7].as_bytes(),
    );

    for (i, digest) in hash.iter().enumerate() {
        println!("{} => {}", test_strs[i], to_hex_string(digest));
    }
}
```

Benchmarks
==========

```
test tests::bench_md5_128bytes_digest      ... bench:         354 ns/iter (+/- 47)
test tests::bench_md5_256bytes_digest      ... bench:         578 ns/iter (+/- 52)
test tests::bench_md5_32bytes_digest       ... bench:         140 ns/iter (+/- 46)
test tests::bench_md5_64bytes_digest       ... bench:         251 ns/iter (+/- 59)
test tests::bench_md5_avx2_128bytes_digest ... bench:         507 ns/iter (+/- 107)
test tests::bench_md5_avx2_256bytes_digest ... bench:         793 ns/iter (+/- 301)
test tests::bench_md5_avx2_32bytes_digest  ... bench:         230 ns/iter (+/- 60)
test tests::bench_md5_avx2_64bytes_digest  ... bench:         383 ns/iter (+/- 119)
```

On the AVX2 implementation
==========================

Thanks to the AVX2 implementation, it is possible to hash 8 strings at the same time. The
only limitation is that the size of all strings has to be the same. If there is any string
with a different size, then its result will be absolutely incorrect.

LICENSE
=======

Copyright 2021 Romeu Gomes

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

use std::env;

use md5::MD5_AVX2;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let text = &args[1];
        // let hash = Sha256::digest(text.as_bytes());

        MD5_AVX2::digest("QnpFg2P1SEQ0L9tcNwBROCW7jVtFeMt0RuF7QODKkgD75CPDi1pAB1GtMcq0G1pmNE6J3IuPpF33uPtOs4sNwU7lKcnF8SU016PKWPeVEpuKQ2ksT9enIf1hVrzlypOkhFTFhIS28IT9OQZ3BS3693487mSb6QNuuaBCD8yNWWlo74c79EFWUWNaAmRcSxVaNcbDa80SovlnL8lyO2yS7XlmE7rPmLI4IvPtko3QguI4Th2JPrVnM7QCCjMgvlIO".as_bytes());

        // println!("{}", to_hex_string(&hash));
    }
}

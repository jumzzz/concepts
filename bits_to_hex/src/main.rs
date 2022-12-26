
use substring::Substring;

#[allow(dead_code)]
fn bits_to_hex() {

    let bin_str = "00001111000011110000111100";
    let str_len = bin_str.len();

    let extra_bits = ((str_len % 8) > 0) as usize;
    let max_idx = str_len / 8 + extra_bits;

    
    println!("{},{},{}", str_len, max_idx, extra_bits);

    let mut hex_vals = Vec::new();

    for i in 0..max_idx {

        let i0 = i * 8;
        let i1 = (i + 1) * 8;

        let sub_bits = bin_str.substring(i0, i1);
        let hex_val = u8::from_str_radix(sub_bits, 2).unwrap();
        hex_vals.push(hex_val);

        println!("{}, {}, {}, 0x{:x}", i0, i1, sub_bits, hex_val);
    }
}

fn main() {
    bits_to_hex();
}
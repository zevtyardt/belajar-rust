use std::io::{stdin, stdout, Write};

fn ask(msg: &str) -> String {
    let mut kata = String::new();
    print!("{}", msg);
    stdout().flush().unwrap();
    stdin().read_line(&mut kata).unwrap();
    kata
}

fn main() {
    let ascii_lowercase = "abcdefghijklmnopqrstuvwxyz";
    let ascii_uppercase = ascii_lowercase.to_uppercase();
    let digits = "0123456789";
    let base64_alphabet = ascii_uppercase + ascii_lowercase + digits + "+/";

    let to_encode = ask("Base64 encoder: ");

    // ubah char ke binari
    let mut chunks_8bit = String::new();
    for char in to_encode.trim().as_bytes() {
        chunks_8bit.push_str(&format!("0{:b}", char).to_string())
    }

    // split 8 bit ke 6 bit
    let mut chunks_6bit = vec![];
    let lenght = chunks_8bit.len();
    for i in (0..lenght).step_by(6) {
        let e = if i + 6 < lenght { i + 6 } else { lenght };
        chunks_6bit.push(&chunks_8bit[i..e])
    }

    // hitung padding
    let last_6bit = chunks_6bit.last().unwrap();
    let padding_amount = if last_6bit.len() < 6 {
        (6 - last_6bit.len()) / 2
    } else {
        0
    };
    let padding = "=".repeat(padding_amount);

    let mut encoded = String::new();
    for bit in &chunks_6bit {
        let bit = format!("{:0<6}", bit);
        let index = usize::from_str_radix(&bit, 2).unwrap();
        encoded.push(base64_alphabet.chars().nth(index).unwrap())
    }
    encoded.push_str(&padding);

    println!(
        "Base64 encoded version of {} is: {}",
        to_encode.trim(),
        encoded
    );
}

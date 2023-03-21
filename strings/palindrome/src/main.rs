use std::io;
use std::io::Write;

pub fn palindrome(s: &str) -> bool {
    let mut chars = s.chars();
    for _ in 1..chars.clone().count() {
        let awal = chars.next();
        let akhir = chars.next_back();

        if (awal.is_some() && akhir.is_some()) && awal != akhir {
            return false;
        }
    }
    return true;

    /*

    alternative: https://github.com/TheAlgorithms/Rust/blob/master/src/string/palindrome.rs

    while let (Some(awal), Some(akhir)) = (chars.next(), chars.next_back()) {
        if awal != akhir {
            return false;
        }
    }
    */
}

fn main() {
    let mut kalimat = String::new();
    print!("masukan kata: ");
    io::stdout().flush().expect("");
    io::stdin()
        .read_line(&mut kalimat)
        .expect("opps, ada error");
    let kalimat = kalimat.trim();

    if palindrome(&kalimat) {
        println!("hasil: {:?} adalah palindrome", kalimat);
    } else {
        println!("hasil: {:?} bukan termasuk palindrome", kalimat);
    }
}

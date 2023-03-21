use std::io::{stdin, stdout, Write};

fn is_anagram(str1: &str, str2: &str) -> bool {
    let mut chars1: Vec<char> = str1.chars().collect();
    let mut chars2: Vec<char> = str2.chars().collect();
    chars1.sort();
    chars2.sort();
    chars1 == chars2
}

fn main() {
    let mut kata1 = String::new();
    let mut kata2 = String::new();

    print!("masukan kata pertama: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut kata1).unwrap();

    print!("masukan kata kedua: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut kata2).unwrap();

    let kata1 = kata1.trim();
    let kata2 = kata2.trim();

    if is_anagram(&kata1, &kata2) {
        println!("hasil: kata {:?} dan {:?} termasuk anagram", kata1, kata2);
    } else {
        println!("hasil: kata {:?} dan {:?} bukan termasuk anagram", kata1, kata2);
    }
}

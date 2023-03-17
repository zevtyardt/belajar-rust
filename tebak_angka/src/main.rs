use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::io::Write;

fn main() {
    let angka = rand::thread_rng().gen_range(1..10001);
    let mut percobaan = 0;

    println!("\nGAME SEDERHANA MENEBAK ANGKA DARI 1 SAMPAI 10000\n");

    loop {
        let mut tebakan = String::new();
        print!("tebakanmu (1 - 10000): ");
        io::stdout().flush().expect("");
        io::stdin()
            .read_line(&mut tebakan)
            .expect("oops, ada error");
        let tebakan: u32 = match tebakan.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        percobaan += 1;

        print!("hasil: ");
        io::stdout().flush().expect("");
        match tebakan.cmp(&angka) {
            Ordering::Less => println!("terlalu kecil"),
            Ordering::Greater => println!("terlalu besar"),
            Ordering::Equal => {
                println!("kamu menang di percobaan ke-{}", percobaan);
                break;
            }
        }
    }
}

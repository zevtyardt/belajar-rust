use std::io;
use std::io::Write;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let angka = rand::thread_rng().gen_range(1..101);
    let mut percobaan = 0;
    
    println!("\nGAME SEDERHANA MENEBAK ANGKA DARI 1 SAMPAI 100\n");

    loop {
        let mut tebakan = String::new();
        print!("tebakanmu (1 - 100): ");
        io::stdout().flush().expect("");
        io::stdin().read_line(&mut tebakan).expect("oops, ada error");
        let tebakan: u32 = match tebakan.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };
        percobaan += 1;

        print!("hasil: ");
        io::stdout().flush().expect("");
        match tebakan.cmp(&angka) {
            Ordering::Less => println!("terlalu kecil"),
            Ordering::Greater => println!("terlalu besar"),
            Ordering::Equal => {
                println!("kamu menang di percobaan ke-{}", percobaan);
                break
            }
        }
    }
}

use std::io;
use std::io::Write;

fn is_prime(m: &u64) -> bool {
    if *m as i64 == 1 {
        return true;
    }
    for divisor in 2..*m {
        if m % divisor == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    loop {
        let mut angka = String::new();
        print!("masukan angka: ");
        io::stdout().flush().expect("");
        io::stdin().read_line(&mut angka).expect("opps, ada error");
        let angka: u64 = match angka.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        let prima = is_prime(&angka);

        if prima {
            println!("hasil: {} adalah bilangan prima", angka);
        } else {
            println!("hasil: {} bukan bilangan prima", angka);
        }
        return;
    }
}

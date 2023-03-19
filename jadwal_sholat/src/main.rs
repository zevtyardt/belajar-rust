use std::io::{stdin, stdout, Write};

use regex::Regex;
use reqwest::blocking;
use select::{document::Document, predicate::{Class, Name}};

fn pilih(pilihan: &Vec<String>) -> String {
    if pilihan.len() == 1 {
        return pilihan.get(0).unwrap().to_string();
    }

    for (index, elem) in pilihan.iter().enumerate() {
        println!(" {}. {}", index + 1, elem);
    }

    loop {
        let mut buff = String::new();
        print!("pilih (1 - {}): ", pilihan.len());
        stdout().flush().unwrap();
        stdin().read_line(&mut buff).unwrap();

        if buff.trim().chars().all(|x| x.is_numeric()) {
            let angka: usize = buff.trim().parse().unwrap();
            if (angka as i32) >= 1 && angka <= pilihan.len() {
                return pilihan.get(angka - 1).unwrap().to_string();
            }
        }
    }
}

fn daftar_kota(kota: &str) -> Vec<String> {
    let re = Regex::new(r#""([a-zA-Z. ]+)"#).unwrap();
    let resp = blocking::get(format!("https://jadwal-sholat.tirto.id/cities?q={kota}")).unwrap();
    let body = resp.text().unwrap();
    let matches: Vec<regex::Match> = re.find_iter(&body).collect();

    let mut daftar_kota = Vec::new();
    for kota in matches {
        daftar_kota.push(String::from(kota.as_str().strip_prefix(r#"""#).unwrap()));
    }
    return daftar_kota;
}

fn tampilkan_jadwal(kota: &str) {
    let kota = kota.to_lowercase().replace(".", "").replace(" ", "-");
    let resp = blocking::get(format!("https://jadwal-sholat.tirto.id/{}", kota)).unwrap();

    let html = resp.text().unwrap();
    let dom = Document::from(html.as_str());

    let tr = dom.find(Class("currDate"))
        .next()
        .unwrap();

    let data: Vec<String> = tr.find(Name("td")).map(|x| x.text()).collect();

    let key= ["TANGGAL", "SUBUH", "DHUHA", "DZUHUR", "ASHAR", "MAGHRIB", "ISYA"];
    print!("\n  ");
    for i in 0..(data.len() - 1) {
        print!("{}  ", key[i]);
        if i as i32 == 0 {
            print!("{}", " ".repeat(data[i].len() - 7))
        }
    }
    print!("\n  ");
    for i in 0..(data.len() - 1) {
        print!("{}  ", data[i]);
        if i as i32 > 1 {
            print!("{}", " ".repeat(key[i].len() - data[i].len()))
        }
    }
    println!("\n");

}

fn main() {
    let mut nama_kota = String::new();
    print!("masukan nama kota: ");
    stdout().flush().unwrap();
    stdin().read_line(&mut nama_kota).unwrap();

    let daftar_kota = daftar_kota(&nama_kota);
    if daftar_kota.len() > 10 {
        println!(
            "  maaf, hasil pencarian kota terlalu banyak. gunakan kata kunci yang lebih spesifik"
        );
    } else {
        let kota = pilih(&daftar_kota);
        println!("menampilkan jadwal sholat di {}", kota);
        tampilkan_jadwal(&kota);
    }
}

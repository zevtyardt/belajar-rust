fn hitung(harga: &f64, pajak: &f64) -> f64 {
    harga * (1.0 + pajak)
}

fn main() {
    let harga = 4500.0;
    let pajak = 0.25;

    let total_harga = hitung(&harga, &pajak);

    println!("Harga awal: Rp. {}", harga);
    println!("Pajak: {}", pajak);
    println!("Total yang harus dibayar adalah Rp. {}", total_harga);
}

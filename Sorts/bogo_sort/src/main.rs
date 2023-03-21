use std::time::Instant;

use rand::{seq::SliceRandom, thread_rng};

fn is_sorted(array: &mut [i32], lenght: usize) -> bool {
    for i in 0..(lenght - 1) {
        if array[i] > array[i + 1] {
            return false;
        }
    }
    return true;
}

fn bogo_sort(array: &mut [i32]) {
    let mut rng = thread_rng();
    let lenght = array.len();

    while !is_sorted(array, lenght) {
        array.shuffle(&mut rng);
    }
}

fn main() {
    let mut array = [7, 6, 8, 88, 5, 5, -7, 8, 84];
    println!("sebelum: {:?}", array);

    let now = Instant::now();
    bogo_sort(&mut array);
    let elapsed = now.elapsed();

    println!("sesudah: {:?}", array);
    println!("waktu yang dibutuhkan: {} detik", elapsed.as_secs());
}

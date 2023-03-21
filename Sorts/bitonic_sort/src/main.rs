fn compare_and_swap(array: &mut [isize], i: i32, j: i32, dire: i32) {
    if (dire == 1 && array[i as usize] > array[j as usize]) || (dire == 0 && array[i as usize] < array[j as usize]) {
        array.swap(i as usize, j as usize);
    }
}

fn bitonic_merge(array: &mut [isize], low: i32, cnt: i32, dire: i32) {
    if cnt > 1 {
        let k = cnt / 2;
        for i in low..(low + k) {
            compare_and_swap(array, i, i + k, dire)
        }
        bitonic_merge(array, low, k, dire);
        bitonic_merge(array, low+ k, k, dire);
    }
}

fn bitonic_sort(array: &mut [isize], low: i32, cnt: i32, dire: i32) {
    if cnt > 1 {
        let k = cnt / 2;
        bitonic_sort(array, low, k, 1);
        bitonic_sort(array, low + k, k, 0);
        bitonic_merge(array, low, cnt, dire);
    }
}

fn main() {
    let mut array = [3, 7, 4, 8, 6, 2, 1, 5];
    let lenght = array.len() as i32;
    println!("array: {:?}", array);
    bitonic_sort(&mut array, 0, lenght, 0);
    println!("diurutkan dari terbesar ke terkecil: {:?}", array);
    bitonic_sort(&mut array, 0, lenght, 1);
    println!("diurutkan dari terkecil ke terbesar: {:?}", array);
}

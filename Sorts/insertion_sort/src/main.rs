fn insertion_sort(array: &mut [i32]) {
    for i in 1..array.len() {
        let cur = array[i];
        let mut j = i;
        while j > 0 && cur < array[j - 1] {
            array[j] = array[j - 1];
            j -= 1
        }
        array[j] = cur;
    }
}

fn main() {
    let mut array = [80, 9, 105, 66, 5];
    println!("sebelum: {:?}", array);
    insertion_sort(&mut array);
    println!("sesudah: {:?}", array);
}

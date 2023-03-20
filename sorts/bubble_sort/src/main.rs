fn bubble_sort(array: &mut [i32]) {
    let lenght = array.len();
    for i in 0..(lenght - 1) {
        let mut swapped = false;
        for j in 0..(lenght - 1 - i) {
            if array[j] > array[j + 1] {
                swapped = true;
                array.swap(j, j + 1);
            }
        }
        if !swapped {
            break;
        }
    }
}

fn main() {
    let mut array = [87, 2, 5, 1, 3, 5, 88, 3, 56];

    println!("sebelum: {:?}", array);
    bubble_sort(&mut array);
    println!("sesudah: {:?}", array);
}

fn insertion_sort(bucket: &mut Vec<usize>) {
    for i in 1..bucket.len() {
        let cur = bucket[i];
        let mut j = i;
        while j > 0 && cur < bucket[j - 1] {
            bucket[j] = bucket[j - 1];
            j -= 1
        }
        bucket[j] = cur;
    }
}

fn bucket_sort(array: &[usize]) -> Vec<usize> {
    let mut max_value = 0;
    let lenght = array.len();
    let mut bucket_list = vec![vec![]; lenght];

    for i in 0..(lenght - 1) {
        if max_value < array[i] {
            max_value = array[i];
        }
    }
    let size: usize = max_value / lenght;
    for i in 0..lenght {
        let j = array[i] / size;
        if j != lenght {
            bucket_list[j].push(array[i]);
        } else {
            bucket_list[lenght - 1].push(array[i]);
        }
    }

    let mut final_output = vec![];
    for i in 0..lenght {
        insertion_sort(&mut bucket_list[i]);
        final_output.extend(bucket_list[i].clone())
    }
    final_output

}

fn main() {
    let mut array = vec![80, 9, 105, 66, 5];
    println!("sebelum: {:?}", array);
    let array = bucket_sort(&mut array);
    println!("sesudah: {:?}", array);
}

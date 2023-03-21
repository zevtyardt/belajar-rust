use rand::{thread_rng, seq::SliceRandom};

fn bead_sort(sequence: &mut [usize]) {
    let mut max = sequence[0];
    for index in 0..sequence.len() {
        if sequence[index] > max {
            max = sequence[index];
        }
    }

    let mut beads = vec![vec![0; max]; sequence.len()];

    for i in 0..sequence.len() {
        for j in (0..sequence[i]).rev() {
            beads[i][j] = 1;
        }
    }

    for j in 0..max {
        let mut sum = 0;
        for i in 0..sequence.len() {
            sum += beads[i][j];
            beads[i][j] = 0;
        }

        for k in ((sequence.len() - sum)..sequence.len()).rev() {
            sequence[k] = j + 1;
        }
    }
}

fn main() {
    let mut rng = thread_rng();
    let mut sequence: Vec<usize> = (1..21).collect();
    sequence.shuffle(&mut rng);

    println!("sebelum: {:?}", sequence);
    bead_sort(&mut sequence);
    println!("sesudah: {:?}", sequence);
}

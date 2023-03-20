use std::io::{stdout, Write, stdin};


fn create_all_state(
    increment: i32,
    total_number: i32,
    level: i32,
    cur_list: &mut Vec<i32>,
    total_list: &mut Vec<Vec<i32>>) {
    if level == 0 {
        total_list.push(cur_list.clone());
        return;
    }

    for i in (increment..(total_number + 1)).step_by(increment as usize) {
        cur_list.push(i);
        create_all_state(i + 1, total_number, level - 1, cur_list, total_list);
        cur_list.pop();
    }
}


fn get_all_combinations(n: i32, k: i32) -> Vec<Vec<i32>> {
    let mut result = vec![];
    create_all_state(1, n, k, &mut vec![], &mut result);
    return result
}

fn input_number(msg: &str) -> i32 {
    loop {
        let mut buff = String::new();
        print!("{}", msg);
        stdout().flush().unwrap();
        stdin().read_line(&mut buff).unwrap();

        if buff.trim().chars().all(|x| x.is_numeric()){
            return buff.trim().parse::<i32>().unwrap();
        }
    }
}

fn main() {
    let n = input_number("n: ");
    let k = input_number("k: ");
    for i in get_all_combinations(n, k) {
        println!(" {}", i.into_iter().map(|x| x.to_string() + ", ").collect::<String>().strip_suffix(", ").unwrap())
    }
}

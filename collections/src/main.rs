use std::collections::HashMap;

fn main() {
    let mut vec = vec![2,4,11,3,23,56,12,2];
    let mid_num = middle(&mut vec);
    println!("middle num is {}", mid_num);
    let mode_num = mode(&vec);
    println!("mode num is {}", mode_num);
}

fn middle(vec: &mut Vec<i32>) -> i32 {
    vec.sort();
    let len = vec.len();
    let mid_idx = len / 2;
    vec[mid_idx]
}

fn mode(vec: &Vec<i32>) -> i32 {
    let mut mode_map = HashMap::new();
    let mut result = Vec::new();
    for n in vec {
        let count = mode_map.entry(n).or_insert(0);
        *count += 1;
    }
    let mut max_count: i32 = 0;
    for (num, c) in mode_map {
        if c > max_count {
            result = Vec::new();
            result.push(*num);
            max_count = c;
        } else if max_count == c {
            result.push(*num);
        }
    }
    result[0]
}

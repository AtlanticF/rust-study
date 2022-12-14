use std::collections::HashMap;

fn main() {
    // practice 1: 中位数和众数
    let mut vec = vec![2,4,11,3,23,56,12,2];
    let mid_num = middle(&mut vec);
    println!("middle num is {}", mid_num);
    let mode_num = mode(&vec);
    println!("mode num is {}", mode_num);
    
    // practice 2: Pig-Latin 每个单词的第一个辅音字母被移动到单词的结尾并增加 "ay"
    // e.g first -> irst-fay
    // 元音字母开头的单词在结尾增加 "hay" apple -> apple-hay
    // 第一个不是元音就是辅音
    let str = String::from("apple");
    let pig_str = pig_latin(&str);
    println!("{} pig_latin is {}", str, pig_str);
    let str2 = String::from("first");
    println!("{} pig_latin is {}", str2, pig_latin(&str2));
}

fn pig_latin(str: &str) -> String {
    let vowel_letter = vec!["a", "e", "i", "o", "u"];
    let first_char = &str[0..1];
    let other = &str[1..];
    match vowel_letter.contains(&first_char) {
        true => format!("{}-{}", str, "hay"),
        false => format!("{}-{}{}", other, first_char, "ay"),
    }
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

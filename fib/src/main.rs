use std::io;

fn main() {
    println!("Please input an integer number.");
    
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line.");
    
    let number: u32 = number.trim().parse().expect("Please intput a number!");
    let res = echo_fib(number);
    println!("{} fib is {}", number, res);
    let res2 = echo_fib_2(number);
    println!("{} fib_v2 is {}", number, res2);
}

fn echo_fib_2(mut n: u32) -> u32 {
    let mut p1 = 1;
    let mut p2 = 1;
    let mut cur = 2;
    while n > 2 {
        cur = p1 + p2;
        p1 = p2;
        p2 = cur;
        n -= 1;
    }
    return cur;
}

// recursion version
fn echo_fib(n: u32) -> u32 {
    if n <= 0 {
        return 0
    }
    if n <= 2 {
        return 1
    } else {
        return echo_fib(n - 1) + echo_fib(n - 2)
    }
}
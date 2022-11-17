use std::io;

fn main() {
    const F_TYPE: u32 = 1;
    const C_TYPE: u32 = 2;

    println!("Please input a number fo temp:");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");

    let number: f64 = number.trim().parse().expect("Please type a number!");

    println!("Please input your tmep type: 1) °F 2) °C");
    let mut temp_type = String::new();
    io::stdin()
        .read_line(&mut temp_type)
        .expect("Faield to read line");
    let temp_type: u32 = temp_type.trim().parse().expect("Please type a number in (1,2)");
    
    let result: f64;
    if temp_type == F_TYPE {
        result = convert_f_to_c(number);
        println!("{} °F -> {} °C", number, result);
    } else if temp_type == C_TYPE {
        result = convert_c_to_f(number);
        println!("{} °C -> {} °F", number, result);
    }
}

fn convert_c_to_f(c: f64) -> f64 {
    32.0 + c * 1.8
}

fn convert_f_to_c(f: f64) -> f64 {
    (f - 32.0) / 1.8
}

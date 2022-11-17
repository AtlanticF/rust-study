fn main() {
    // if expression
    let number = 3;
    // if condition must bool
    // panic: if number
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // else if
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is no divisible by 4, 3 or 2");
    }

    // use if in let
    let condition = true;
    let num = if condition { 5 } else { 6 };
    println!("The value of num is {num}");
    // if expresion return value must same
    // if condition { 1 } else { "six" } will panic
    
    // loop while for
    let mut counter = 0;
    loop {
        counter += 1;
        if counter > 5 {
            break;
        }
        println!("again!");
    };
    let mut count = 0;
    let result_count = loop {
        count += 1;
        if count == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result_count}");

    // loop label
    let mut label_count = 0;
    'counting_up: loop {
        println!("label_count = {label_count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if label_count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        label_count += 1;
    }
    println!("End count = {label_count}");


    // while
    let mut w = 3;
    while w != 0 {
        println!("{number}");
        w -= 1;
    }
    println!("LIFTOFF!!!");

    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("The value in arr index {index} is {}", arr[index]);
        index += 1;
    }

    // for
    for element in arr {
        println!("For loop arr {element}");
    }

    for e in (1..4).rev() {
        println!("Game count down: {e}!!!");
    }
}

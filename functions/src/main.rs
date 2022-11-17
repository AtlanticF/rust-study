fn main() {
    // statement has no return value.
    // statement contains expression: 6
    let y = 6;
    // we can not use like this: let x = (let y = 6);
    // because let y = 6; is a statement.
    println!("Hello, function! {y}");
    another_function(y, 'H');

    let x = {
        let m = 3;
        // default return last expression's value
        // return 4 then x = 4
        m + 1
    };
    println!("The value of x is {x}");

    let five = five();
    println!("The value of five is {five}");
    let plus = plus_one(five);
    println!("The value of five plus on is {plus}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

// define a function five() return data type is i32
// 5 is a expression, defualt returned.
fn five() -> i32 {
    5
}

fn another_function(x: i32, c: char) {
    println!("Another function.");
    println!("argument x is {x}, argument c is {c}");
}

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // shadowing
    let y = 5;
    let y = y + 1;
    println!("1. The value of y is: {y}");
    {
        let y = y * 2;
        println!("2. The value of y is: {y}");
    }
    println!("3. The value of y is: {y}");

    let spaces = "    ";
    let spaces: usize = spaces.len();
    println!("spaces size: {spaces}");
}

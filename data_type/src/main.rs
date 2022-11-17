fn main() {
    // scaler and compounds
    // scaler - int

    // 8,16,32,64,128... u or i
    // for two's complement
    // i32 is the default int rust
    let a: i32 = 1;
    let b: u32 = 2;
    // when "integer overflow", in debug mode, panic happen, in release mode, warpping
    println!("Rust integer data type: i32 {a} and u32 {b}");

    // scaler - float - f32 and f64
    // default is f64, f64 in modern CPU almost no difference with f32
    // use IEEE-754: test is 0.1 + 0.2
    let c = 0.1;
    let d = 0.2;
    let e = c + d;
    println!("Test 0.1 + 0.2 = {e}");
    println!("Test float 0.1 + 0.2 = {}", 0.1_f32 + 0.2_f32);

    // scaler - float
    println!("Float data {}, {}", true, false);

    // scaler - char
    // char default use 4 bytes = 4*8 = 32bit
    // Unicode Scalar Value
    let r_char = '😊';
    println!("Emoji character: {}", r_char);

    // compounds - tuple
    // fix with length
    // can use diffrent data type for every element
    let tup = (1, 0.2, true);
    // destructure
    let (_x, y, _z) = tup;
    println!("The value of y is {y}");
    // unit: let tup = ();
    let tup2: (i32, f32, bool) = (1, 0.2, false);
    // visit of index
    println!("Index 0 in tup2 {}", tup2.0);

    // compounds - array
    // memory allocat on stack not heap
    // element data type must same
    let arr = [1, 2, 3];
    // visit of index
    println!("Index 0 in arr {}", arr[0]);
    // out of bounds is a runtim panic: arr[5]
    // let arr: [i32, 5] = [1, 2, 3, 4, 5];
    // let arr = [3; 5]; => [3, 3, 3, 3 ,3];
}

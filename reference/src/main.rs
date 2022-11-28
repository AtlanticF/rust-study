fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    // if we want use s1 again, we should not to `move` ownship to function calculate_length
    // we can use reference to borrowing s1's value but not ownship
    // then we can use s1 again.
    println!("The length of '{}' is {}.", s1, len);

    // try to change the value from a reference
    // this can not to do, because s1 is immutable
    // change(&s1);
    
    // try to change mut variable from reference
    let mut s = String::from("hello");
    change(&mut s);
    println!("s change success: {}", s);

    // same time only has one mutable reference, but can has more immutable reference
    // let mut s2 = String::from("hello");
    //let r1 = &mut s2;
    // here we can not create mut/immut reference again.
    // let r2 = &mut s2; // complier error.
    // we can create multi immut reference
    
    let mut s2 = String::from("hello");
    let r2 = &s2;
    let r3 = &s2;
    println!("r2: {}, r3: {}", r2, r3);

    // here we can create mut reference, because r2 and r3 is moved.
    let r4 = &mut s2;
    println!("NLL r4: {}", r4);

    // dangling references
    // let reference_to_nothing = dangle();
}

//fn dangle() -> &String {
//    // try to return a reference
//    let s = String::from("hello");
//
//    &s
//} // here s leave scope, the reference is dangling. 


fn change(s: &mut String) {
    s.push_str(", world");
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // here  s leave scope but not has ownship so drop not called.

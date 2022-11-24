fn main() {
    // onwership: management heap memory when complier
    // 1. every value has one owner.
    // 2. any times value only has one owner.
    // 3. when variable leave scope, value will be drop.
    {
        let s = "hello"; // s is effective.
                         // s in memory on heap static area.
        // you can use s hear.
        println!("{} in it's scope, we can use is.", s);
    } // s leave scope, come to invalid.

    // String memeory allocate in runtime.
    let mut s1 = String::from("hello");
    s1.push_str(", world!");
    println!("{}", s1);
    // s1 life over
    // s1 use over, back memory, drop

    // Drop and Copy
    let x = 5;
    let y = x;
    // int type is implements Copy
    // hear you can use x too.
    println!("x + y = {}", x + y);

    let s2 = String::from("hello");
    // s2 in meomory: stack -> {ptr, len, capacity}
    // heap -> ['h','e','l','l','o']
    let s3 = s2;
    // hear s2 life over.
    // s2 move to s3
    println!("we can only use s3: {}", s3);

    // if we need deep copy, we can use clone
    let s4 = String::from("hello");
    let s5 = s4.clone();
    println!("use clone for deep copy, s4={}, s5={}", s4, s5);

    // function move ownership.
    let s6 = String::from("hello");
    // we move s6 ownership and value to the fucntion
    // s6 = some_string
    takes_ownership(s6);
    // hear s6 over
    let x1 = 5;
    makes_copy(x1);
    // hear x1 is valid
    println!("x1 reuse {}", x1);

    let s7 = gives_ownership();
    // some_string move to s7 we can use s7
    println!("some_string move s7: {}", s7);
    let s8 = String::from("hello");
    // move s8 to function and return, value and ownership move to s9
    let s9 = takes_and_gives_back(s8);
    println!("s8 move function and return move back to s9: {}", s9);
}

fn takes_ownership(some_string: String) { // some_string into scope
    println!("takes string ownership: {}", some_string);
} // some_string drop

fn makes_copy(some_integer: i32) {
    println!("copy i32: {}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string // return some_string and ownership
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

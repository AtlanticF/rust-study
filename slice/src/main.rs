fn main() {
    let s = String::from("中");
    // slice range can not get valid char.
    // complier success. runtime error.
    // let c = &s[0..1];
    let c = &s[0..];
    println!("中 slice[0..] is {}", c);

    let a = String::from("hello world");
    let hello = first_world(&a);
    println!("first world in '{}', is {}", a, hello);
}

fn first_world(s: &str) -> &str {
    let b = s.as_bytes();

    for (i, &item) in b.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

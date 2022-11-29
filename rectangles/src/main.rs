#[derive(Debug)]
struct Rectangle {
    width: u32,
    heigth: u32,
}
fn main() {
    let rectangle = Rectangle {
        width: 30,
        heigth: 50,
    };
    // 与 let r2 = rectangle 不同，= 已经发生 move
    // 创建新的 r2 使用 rectangle，没有发生 move，因为 Rectangle 字段都实现 Copy trait
    let r2 = Rectangle {
        ..rectangle
    };

    let area = area(&rectangle);
    println!("The rectangle area is: {}", area);
    println!("Rectangle: {:?}, New r2 build from rectangle: {:?}", rectangle, r2);

    // use dbg!
    // dbg! return expression's ownship
    // dbg! output to stderr
    // dbg! get parameter's ownship
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        heigth: 50,
    };
    dbg!(&rect1);
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.heigth
}

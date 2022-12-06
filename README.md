Study rust from [trpl](https://kaisery.github.io/trpl-zh-cn/)

- 编译：rustc
- 包管理：cargo build / run / new / update
    - Cargo.xml: dependences (SemVer) 语义化版本号
    - Cargo.lock: 构建可重现

## 一、一些概念

### 1.1 变量

- 使用 let 申明
- 变量默认是 immutable（不可变）的
- 使用 let mut 申明可变变量

### 1.2 常量

- 使用 const 申明
- 常量总是不能变
- 可以在任何作用域中申明，包括全局
- 约定是在单词之间使用全大写加下划线
- 编译时计算
- 在申明的作用域中，常量在整个程序生命周期中都有效

### 1.3 隐藏（shadowing）

- 定义一个与之前变量同名的新变量的行为，称为 shadowing
- mut 与 shadowing 有区别
    - 不使用 let 对 **不可变** 变量进行重新赋值时会报错
    - shadowing 实际创建一个新的变量，可以改变 **值** 类型，并且复用这个名字

### 1.4 数据类型

#### 1.4.1 标量 scaler: 单独的值

- 整型
    - 默认 i32
    - 8，16，32，64，128 bit 有无符号
    - arch 有无符号: isize usize，依赖 CPU 架构
    - 允许 `_` 分隔符: 1_000 = 1000
    - integer overflow
        - 在 debug 构建中产生 panic
        - 在 release 构建中发生 warpping 操作
- 浮点型
    - f32, f64 使用 IEEE-754 标准表示
    - 默认 f64
- 布尔
    - true, false
- 字符
    - 单引号
    - 占4字节
    - 代表一个 Unicode 标量值

#### 1.4.2 复合 compound: 多个值组合

- 元组 tuple
    - let tup = (100, 2.0, 1) / let tup: (i32, u32, f64) = (-2, 1, 2.3)
    - 声明后长度不变
    - 各元素类型可以不同
    - let (x, y, z) = tup 称之为 destructure
    - 通过 `.` 访问元素
- 数组 array
    - [] 申明
    - 数组内各元素类型必须相同
    - 不可变长度
    - let a: [i32; 5] = [1, 2, 3, 4, 5]
    - let a = [3; 5] 长度为 5 的数组，元素都为 3
    - a[index] 访问
    - index out of bounds：运行时错误，panic

### 1.5 函数

`snake case` 规范风格，小写加下划线分隔单词

#### 1.5.1 参数

- parameter 和 argument

#### 1.5.2 语句和表达式

- 函数体 = 语句 + 可选的结束表达式
- Rust 是 expression-based 语言
- Statements 执行操作，不返回值的 指令
- Expression 计算并产生一个值 
- 表达式可以是语句的一部分: let x = 6
- 函数隐式返回最后的 **表达式**

#### 1.5.3 注释

- `//`

#### 1.5.4 控制流

- 判断 **表达式**: if
- if (expression) expression 必须为 bool
- 因为是表达式，所以具有返回值: let number = if condition {1} else {2};
- 分支表达式返回值类型必须相同

- loop
- 循环标签 'counting_up: loop {} -- 消除多个循环之间的歧义
- while 条件循环
- for 遍历集合 for element in a {}
- for number in (1..4).rev() {}

## 二、所有权（Ownership）

变量的所有权总是遵循相同的模式：将值赋给另一个变量时移动它。当持有堆中数据值的变量离开作用域时，其值将通过 drop 被清理掉，除非数据被移动为另一个变量所有。

官方的说法：没有 GC，无需手动分配和释放。依赖 ownership 系统管理，`wonership 任何功能都不会减慢程序`。跟踪代码正在使用的 `堆` 上的数据，最大限度减少堆上 `重复` 数据的量，并且清理堆上不再使用的数据。

ownership 是 rust 内存管理的一种实现。区别于 GC 和手动管理内存，ownership 为每块被分配的内存定义了作用域和所有者，`(堆上)内存在拥有它的变量离开作用域后就被自动释放`

理解：`heap 上分配的内存`指向的变量，rust 会使用 `ownership` 管理起来，当变量`离开作用域`时，调用 `drop` 自动清理变量的堆内存。

关于二次释放（double free）：
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    // stack 上 s1 标记为无效，此时无法使用 s1
    // 也可以理解为 s1 的 ownership move 到 s2 了

    println!("{}, world!", s1);
}
```
 
### 2.1 三个规则

- Rust 中每个值都有一个 **所有者（Owner）**
- 值在任何时刻都只有 **一个** 所有者
- 当所有者（变量）离开作用域，这个值被抛弃

### 2.2 变量作用域

当变量离开作用域，Rust 为我们调用一个特殊的函数。这个函数叫做 drop

### 2.3 移动

```rust
let x = 5;
let y = x;

let s1 = String::from("hello");
let s2 = s1;
```
![](https://kaisery.github.io/trpl-zh-cn/img/trpl04-01.svg)
![](https://kaisery.github.io/trpl-zh-cn/img/trpl04-02.svg)
![move](https://kaisery.github.io/trpl-zh-cn/img/trpl04-04.svg)

### 2.4 克隆

需求：需要深度复制某变量堆上数据

使用：
```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    // 此时 s1，s2 均有效，s1 ownership 并没有 move
    // 堆上申请了两份内存，存储 "hello"
    println!("s1 = {}, s2 = {}", s1, s2);
}
```

### 2.5 Copy

像整型这样的在编译时已知大小的类型被整个存储在栈上，所以拷贝其实际的值是快速的。

实现：`Copy trait` 注解。任何一组简单标量值的组合都可以实现 Copy。

### 2.6 所有权和函数

```rust
fn main() {
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的 值 移动到函数里，s 失效，some_string 有效
                                    // some_string = s
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，
                                    // 所以在后面可继续使用 x

} // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 没有特殊之处

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。
  // 占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。没有特殊之处
```

### 2.6 所有权和作用域

返回值也可以转移所有权。

```rust
fn main() {
    let s1 = gives_ownership();         // gives_ownership 将返回值
                                        // 转移给 s1

    let s2 = String::from("hello");     // s2 进入作用域

    let s3 = takes_and_gives_back(s2);  // s2 被移动到
                                        // takes_and_gives_back 中,
                                        // 它也将返回值移给 s3
} // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 离开作用域并被丢弃

fn gives_ownership() -> String {             // gives_ownership 会将
                                             // 返回值移动给
                                             // 调用它的函数

    let some_string = String::from("yours"); // some_string 进入作用域.

    some_string                              // 返回 some_string 
                                             // 并移出给调用的函数
}

// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用的函数
}
```
问题：继续使用所有权转移前的参数是常见的需求，每次转移再返回略显啰嗦，如果使用使用 tuple 也是复杂，如何更优雅地解决这个问题？
Rust 提出 **borrowing** 和 **references**

## 三、引用和借用

### 3.1 概念

引用（reference）像一个指针，它是一个地址，可以由此访问储存于该地址的属于其他变量的数据。允许你使用值但不获取其所有权。

理解：猜测实现，rust 建立引用，但是并不会将引用与值做绑定（赋予引用值的所有权），所以引用在离开其作用域的时候也不会进行 drop

创建一个引用的行为：**借用（borrowing）**
```rust
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
// 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
// 所以什么也不会发生（引用离开作用域不会 drop，因为不具有所有权）
```
![](https://kaisery.github.io/trpl-zh-cn/img/trpl04-05.svg)

`正如变量默认是不可变的，引用也一样。（默认）不允许修改引用的值。借的东西不能随意改变，这里理所当然的。`

🤔 思考：通过传递可变的引用是否可以修改不可辨的变量？No: 不可变变量就是不可变

### 3.2 可变引用

可变引用的限制：如果你有一个对该变量的**可变引用**，**在同一作用域**，你就不能再创建对该变量的**引用**。

设计目的：避免 data race 数据竞争导致未定义行为。防止同一时间对同一数据存在多个可变引用。

造成 data race 原因：
- 两个或多个指针`同时`访问同一数据
- 至少有一个指针被用来写数据
- 没有同步数据访问的机制

多个不可变引用是可以的，因为没有哪个只能读取数据的人有能力影响其他人读取到的数据。

引用的作用域从声明的地方开始一直持续到最后一次使用为止。编译器在作用域结束之前判断不再使用的引用的能力被称为 **非词法作用域生命周期**（Non-Lexical Lifetimes，简称 NLL）。

```rust
fn main() {
    // 可变变量
    let mut s = String::from("hello");
    // 增加 mut -> 创建可变引用 s
    change(&mut s);
}

fn change(some_string: &mut String) {
    // 改变引用 somg_string 指向的内存的值
    some_string.push_str(", world");
}
```

```rust
fn main() {
    let mut s = String::from("hello");

    let r1 = &s; // 没问题
    let r2 = &s; // 没问题
    println!("{} and {}", r1, r2);
    // 此位置之后 r1 和 r2 不再使用（后续代码中没有 r1 和 r2？）

    let r3 = &mut s; // 没问题
    println!("{}", r3);
}
```

### 3.3 垂悬引用

悬垂指针：其指向的内存可能已经被分配给其它持有者。
在 Rust 中编译器确保引用永远也不会变成悬垂状态：当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。

```rust
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    // s 创建
    let s = String::from("hello");
    // 尝试返回 s 的引用
    &s
} // s 离开作用域，drop 调用，内存释放，此时 &s 垂悬
```

### 3.4 引用规则

- 在任意给定时间（作用域），要么 只能有一个可变引用，要么 只能有多个不可变引用。
- 引用必须总是有效的。（编译器保证没有垂悬引用）。

## 四、Slice 类型

Slice 允许引用集合中一段`连续的`元素序列。slice 是一类引用，`没有所有权`。

问题：编写一个函数，该函数接收一个用空格分隔单词的字符串，并返回在该字符串中找到的第一个单词。如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。

```rust
fn first_word_idx(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
fn main() {
    let mut s = String::from("helo world");
    let word = first_word_index(&s);
    s.clear();
    // word 值仍然是 5
    // 因为 s 无效了，此处 5 也没有意义了
}
```

### 4.1 字符串 slice

```rust
let s = String::from("hello world");
// [0,5) [6,11)
// len = end - start
let hello = &s[0..5]; // hello -> reference -> 'hello'
let world = &s[6..11]; // world -> reference -> 'world'
```

    注意：字符串 slice range 的索引必须位于有效的 UTF-8 字符边界内，如果尝试从字接字符的中间位置创建字符串 slice，则程序将会因错误而退出。（理解：slice range 是单字节的）

```rust
// string slice range 单字节

let s = String::from("中"); // hex:E4B8AD -> 3bytes
let c = &s[0..1];
println!("中 slice[0..1] is {}", c);
```

String variable, String reference, String reference 对比：

- String variable: {ptr, len, capacity}
- String reference: {ptr}
- String slice: {ptr, len}

```rust
// &str 代表 string slice
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
fn main() {
    let mut s = String::from("hello world");
    // 获取了 s 的不可变引用 -> world
    let world = first_word(&s);
    s.clear(); // 清空会尝试获取一个可变引用，编译错误
    // 还要使用 world，根据 NLL 此时 world 不可变引用依然有效
    println!("the first world is: {}", world);
}
```

### 4.2 字符串的字面值就是 slice

```rust
let s = "Hello world";
```

为什么字符串字面值不可变？因为是 &str 类型，它是一个不可变引用。


### 4.3 字符串 slice 作参数

```rust
// deref coercions
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // `first_word` 适用于 `String`（的 slice），整体或全部
    let word = first_word(&my_string[0..6]);
    let word = first_word(&my_string[..]);
    // `first_word` 也适用于 `String` 的引用，
    // 这等价于整个 `String` 的 slice
    let word = first_word(&my_string);

    let my_string_literal = "hello world";

    // `first_word` 适用于字符串字面值，整体或全部
    let word = first_word(&my_string_literal[0..6]);
    let word = first_word(&my_string_literal[..]);

    // 因为字符串字面值已经 **是** 字符串 slice 了，
    // 这也是适用的，无需 slice 语法！
    let word = first_word(my_string_literal);
}
```

### 4.4 其他类型的 slice

```rust
fn main() {
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}
```

## 五、结构体 Struct

- structure 是什么？
- 如何定义一个 struct?
- 如何实例化 struct?
- struct 字段的简化写法
- 结构体更新语法
- 元组结构体
- 类单元结构体
- **结构体数据的所有权**

结构体是自定义数据类型。

```rust
// 定义结构体
struct User {
    username: String,
    email: String,
    active: bool,
    sign_in_account: u64,
}
// 实例化
let user = User {
    username: String::from("simon"),
    email: String::from("simon@gmail.com"),
    active: true,
    sign_in_account: 1,
}
// 简化：与 field 相同的名字
fn build_user(username: String, email: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_account: 1,
    }
}
// 更新语法创建实例
let user2 = User {
    active: false,
    ..user // 必须放在最后
};
// 创建 user2 后不能再使用 user，因为 user 的 username 和 email 被移动到 user2 中。所有权被 move 了
// 元祖结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
// 类单元结构体: 实现 trait
struct AlwaysEqual;

// dbg! 接收一个表达式的所有权（println!是引用）打印调用时的文件和行号，以及表达式结果只，并返回该值的所有权
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}
```
Rust 不允许将结构体某个字段标记为可变。

### 5.1 方法语句

方法与函数：
- method 和 function
- 都使用 fn + 名字 声明，可以拥有参数和返回值
- 方法定义在 `结构体` 上下文中（struct,enum,trait)
- 方法的第一个参数总是 `self`，代表调用方法的结构体实例

方法定义：

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// impl 中所有内容都与 Rectangle 关联
impl Rectangle {
    // &self -> rectangle: &Rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // associated functions
    // 我们可以定义不以 self 为第一参数的关联函数（因此不是方法）
    fn say_hello() {
        println!("hello");
    }
    // 关联函数 经常 被用作返回一个结构体新实例的构造函数
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}
// 每个结构体都允许拥有多个 impl 块
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        // automatic referencing and dereferencing
        // (&rect1).area()
        rect1.area()
    );
}
```

## 六、枚举和模式匹配（enumerations and match patterns）

### 6.1 枚举的定义

枚举提供把值成为一个集合之一的方法。

```rust
enum IpAddrKind {
    IPv4,
    IPv6,
}
let ipv4 = IpAddrKind::IPv4;
let ipv6 = IpAddrKink::IPv6;
fn route(ip_kind: IpAddrKind) {}
route(IpAddrKind::IPv4);
route(IpAddrKind::IPv6);

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}
let localhost = IpAddr {
    kind: IpAddrKind::IPv4,
    address: String::from("127.0.0.1"),
};
let loopback = IpAddr {
    kind: IpAddrKind::IPv6,
    address: String::from("::1"),
},

enum IpAddr {
    V4(String), // V4(u8, u8, u8, u8)
    V6(String),
}
let localhost = IpAddr(String::from("127.0.0.1"));
let loopback = IpAddr(String::from("::1"));

// Quit 没有关联任何数据
// Move 类似结构体包含命名字段
// Write 包含单独一个 String
// ChangeColor 包含三个 i32
enum Message {
    Quit,
    Move: {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32),
}
// enum 依然可以使用 impl 来定义方法
impl Message {
    fn call(&self) {

    }
}
let m = Message::Wirte(String::from("hello"));
m.call();
```

Rust没有空值，存在None概念枚举 Option<T>

```rust
enum Option<T> {
    None,
    Some(T),
}
let some_number = Some(5);
let some_char = Some('a');
// 允许使用 Option<T> 来定义 None 值
// 希望 absent_number 是 i32 类型，但是现在它的值是枚举 None
let absent_number: Option<i32> = None;
let x: i8 = 5;
let y: Option<i8> = Some(5);
// no implementation for `i8 + Option<i8>`
// 类型不同，无法相加
// 在对 Option<T> 进行 T 的运算之前必须将其转换为 T。通常这能帮助我们捕获到空值最常见的问题之一：假设某值不为空但实际上为空的情况。
// 只有当使用 Option<i8>（或者任何用到的类型）的时候需要担心可能没有值，而编译器会确保我们在使用值之前处理了为空的情况。
let sum = x + y;
```

### 6.2 match 模式匹配控制流结构

match 控制流运算符允许我们将一个值与一系列的模式相比较，并根据匹配的模式执行相应的代码（有点像 switch case）

```rust
enum Mac {
    BookPro,
    BookAir,
    IMac,
}
fn value_in_mac(mac: Mac) -> u32 {
    match mac {
        Mac::BookPro => 9999,
        Mac::BookAir => 7999,
        Mac::IMac => 12999,
    }
}
// 通常分支代码比较短不使用大括号
// 多行代码可以使用大括号，分支后的逗号是可选的


// 通过枚举成员绑定值
#[derive(Debug)]
enum Country {
    China,
    America,
    India,
    Japanese,
    Korean,
}
// macbookpro 在不同国家的版本不同
enum Mac {
    BookPro(Country),
    BookAir,
    IMac,
}
fn value_in_mac(mac: Mac) -> u32 {
    match mac {
        Mac::BookPro(country) => {
            println!("Country from {:?} version.", country);
            9999
        },
        Mac::BoorAir => 7999,
        Mac::IMac => 12999,
    }
}
// call
value_in_mac(Mac::BookPro(Country::China));
```

#### Option<T> 匹配

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

match 匹配是穷尽的（有限的）：必须穷举到最后的可能性来使代码有效

#### 通配模式和 _ 占位符

如果希望对一些特定的值采用特殊操作，对其他的值采用默认操作。

```rust
let dice_roll = 9;
match dice_roll {
    3 => have_a_drink(),
    6 => take_a_photo(),
    // other 通配
    // 即使我们没有列出 u8 所有可能的值，这段代码依然能够编译，因为最后一个模式将匹配所有未被特殊列出的值
    // 我们必须将通配分支放在最后
    // 如果我们在通配分支后添加其他分支，Rust 将会警告我们，因为此后的分支永远不会被匹配到
    other => move_player(other),
}
fn have_a_drink() {}
fn take_a_photo() {}
fn move_player(other: u8) {}

fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => assassinate(),
        7 => guard(),
        // 无事发生，不做任何操作
        _ => (),
    }
    fn assassinate() {}
    fn guard() {}
}
```

### 6.3 if let 简洁控制

简化冗长的 match 模式匹配。
这样会失去 match 强制要求的穷尽性检查。
match 和 if let 之间的选择依赖特定的环境以及增加简洁度和失去穷尽性检查的权衡取舍。

可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。

```rust
let config_max = Some(3u8);
match config_max {
    Some(max) => println!("max: " + max),
    _ => (),
}

let config_max_2 = Some(3u8);
if let Some(max) => config_max_2 {
    println!("max: " + max);
}
// 可以在 if let 中包含一个 else。else 块中的代码与 match 表达式中的 _ 分支块中的代码相同

let mut count = 0;
if let Coin::Quarter(state) => coin {
    println!("count: " + count);
} else {
    count += 1;
}
```

## 七、包、Crate和模块

### 7.1 包和 crate

`crate 是 Rust 在编译时最小的代码单位，即便是用 rustc 而不是 cargo 来编译一个文件，编译器还是会将那个文件认作一个 crate`

crate 两种形式：
- 二进制项
- 库

package 组织一个或多个 crate，package 中最多包涵一个 library crate，可以包含任意多个 binary crate。至少包含一个 crate。

Cargo 遵循的一个约定：src/main.rs 就是一个与包同名的二进制 crate 的 crate root。
同样的，如果包目录中包含 src/lib.rs，则包带有与其同名的库 crate，且 src/lib.rs 是 crate root。

crate 根文件将由 Cargo 传递给 rustc 来实际构建库或者二进制项目。
crate 和模块结构组成 module tree。

### 7.2 module 控制 scope 和私有性

编译器编译模块的工作过程：

- **开始：**从 crate root 开始寻找要被编译的代码
- **申明模块：**在 crate root 中申明的模块会在以下路径寻找：
    - 内联，在大括号中，当 mod ${module_name} 后方不是一个分号而是一个大括号
    - 文件 src/${module_name}.rc
    - 文件 src/${module_name}/mod.rc
- **申明子模块：**在 crate root 外的其他文件中，可以定义：
    - 内联，在大括号中，当 mod ${module_name} 后方不是一个分号而是一个大括号
    - 文件 src/${parent_module}/${module_name}.rc
    - 文件 src/${parent_module}/${module_name}/mod.rc
- **模块的代码路径：**在隐私规则允许的前提下，同一 crate 内任意地方可以通过代码路径引用该模块的代码（🤔不同 crate 内要引入别的 crate 的代码呢？)
- **pub 关键字：**一个模块里的代码默认对父模块私有（默认父模块无法引入模块）。为了使一个模块公用，应当在声明时使用pub mod替代mod。为了使一个公用模块内部的成员公用，应当在声明前使用pub（函数/方法也有有隐私规则）
- **use 关键字：**use关键字创建了一个成员的快捷方式，用来减少长路径的重复（类似 PHP use）

### 7.3 引用模块

- 绝对路径：从 crate 根开始，以 crate 名字或者字面值 crate 开头。
- 相对路径：从当前模块开始，以 self、super 或当前模块的标识符开头。

绝对路径和相对路径都后跟一个或多个双冒号`::`分割。

我们更倾向于使用绝对路径，因为把代码定义和项调用各自独立地移动是更常见的。

Rust 中默认所有项（函数、方法、结构体、枚举、模块和常量）都是私有的。父模块中的项不能使用子模块中的私有项，但是子模块中的项可以使用他们父模块中的项。这是因为子模块封装并隐藏了他们的实现详情，但是子模块可以看到他们定义的上下文。

私有性规则不但应用于模块，还应用于结构体、枚举、函数和方法。

#### pub 共有结构和枚举

如果我们在一个结构体定义的前面使用了 pub ，这个结构体会变成公有的，但是这个结构体的字段仍然是私有的。
如果我们将枚举设为公有，则它的所有成员都将变为公有。

### 7.4 use 关键字引入路径到作用域

目的：解决全路径的冗长和重复

在作用域中增加 use 和路径类似于在文件系统中创建软连接（符号连接，symbolic link）。

要想使用 use 将函数的父模块引入作用域，我们必须在调用函数时指定父模块，这样可以清晰地表明函数不是在本地定义的，同时使完整路径的重复度最小化。
另一方面，使用 use 引入结构体、枚举和其他项时，习惯是指定它们的完整路径。

    🤔 也就是说，通常 use 模块唯独即可，可以清晰地表明使用的方法是属于哪个模块的。但是在引入结构题和枚举时，习惯是用完整路径，这有点像其他语言中引入一个常量。只是习惯。

例外:

```rust
use std::fmt;
use std::io;

fn function1() -> fmt::Result {
    // --snip--
    Ok(())
}
fn function2() -> io::Result<()> {
    // --snip--
    Ok(())
}
```

使用 as 提供别名：

```rust
use std::fmt::Result;
use std::io::Result as IoResult;

fn function1() -> Result {
    // --snip--
    Ok(())
}

fn function2() -> IoResult<()> {
    // --snip--
    Ok(())
}
```

使用 pub use 重导出
不仅将一个域，还允许别人把它导入他们自己的作用域。

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
// 通过 pub use 重导出，外部代码现在可以通过新路径 restaurant::hosting::add_to_waitlist 来调用 add_to_waitlist 函数。
// 相当于 restaurant 有了 hosting pub mod
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

使用外部包 package:
- 在 Cargo.toml [dependencies] 中引入
- use 使用
- 标准库无需引入，直接使用 std::（对于 String 其实是 prelude)

🤔 如果有大量的 use，是否有优化语法？使用嵌套

```rust
use std::cmp::Ordering;
use std::io;

use std::{cmp::Ordering, io};

use std::io;
use std::io::Write;
use std::io{self, Write};
```

🤔 有没有引入所有公有定义的方式？glob 运算符 *

```rust
use std::collections::*;

glob 运算符经常用于测试模块 tests 中，这时会将所有内容引入作用域。
```

### 7.5 模块拆分

问题：单个模块文件中多个模块持续膨胀，需要简化代码增加可读性。

```rust
// 申明引入 front_of_house 模块
// front_of_house.rc
mod front_of_house;

pub use crate::front_of_house:hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

// 子模块
pub mod hosting;
// src/front_of_house/hosting.rs
```

## 八、常见集合

- Vector: 有序存储一系列可变值
- String: 字符的集合
- HashMap: 哈希 map

### 8.1 Vector

- 内存中数据相邻排列
- 只能存储相同类型的值

#### 新建 Vector

```rust
let v: Vec<i32> = Vec::new();
let v = vec![1,2,3];
```

#### 更新 Vector
```rust
let mut v = Vec::new();

v.push(1);
v.push(2);
v.push(3);
```

#### 销毁 Vector
```rust
{
    let v = vec![1,2,3];
} // leave scope here, drop
```

#### 读取 Vector 元素
```rust
let v = vec![1,2,3,4,5];
let third: &i32 = &v[2];
println!("The third vector is {}", third);
match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
// index is begin from 0
// use & and [] get element
// [] 方式可以处理 out of bound 的问题，超过索引最大值时，返回 None

let mut v = vec![1,2,3,4,5];
let first = &v[0]; // 不可变引用
v.push(6); // 可变引用（扩容）
// 如果 v 没有足够的空间存储新元素
// rust 会进行自动扩容，此时需要拷贝原来的值到新的空间中
println!("The first element is {}", first);
```

#### 遍历 Vector

```rust
let v = vec![1,2,3,4,5];
for i in &v {
    println!("{}", i);
}

let mut v = vec![100,200,300];
for i in &mut v {
    // 解引用获取 i 的值
    *i += 50;
}
```

#### 使用 enum 存储不同类型的值到 Vector

枚举成员被认为是同一个枚举类型。

```rust
enum DataTypes {
    Int(i32),
    Float(f64),
    Text(String),
    Ch(char),
}
let datas = vec![
    DataTypes::Int(100),
    DataTypes::Float(10.1),
    DataTypes::Text(String::from("china")),
    DataTypes::Ch('a'),
]
```
Rust 在编译时就必须准确的知道 vector 中类型的原因在于它需要知道储存每个元素到底需要多少内存。

### 8.2 String

### 8.3 HashMap
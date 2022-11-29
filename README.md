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
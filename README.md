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
    - stack 上分配空间
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

官方的说法：没有 GC，无需手动分配和释放。依赖 ownership 系统管理，`wonership 任何功能都不会减慢程序`。跟踪代码正在使用的 `堆` 上的数据，最大限度减少堆上 `重复` 数据的量，并且清理堆上不再使用的数据。

ownership 是 rust 内存管理的一种实现。区别于 GC 和手动管理内存，ownership 为每块被分配的内存定义了作用域和所有者，`(堆上)内存在拥有它的变量离开作用域后就被自动释放`

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
Study rust from [trpl](https://kaisery.github.io/trpl-zh-cn/)

- 编译：rustc
- 包管理：cargo build / run / new / update
    - Cargo.xml: dependences (SemVer) 语义化版本号
    - Cargo.lock: 构建可重现

## 一些概念

### 变量

- 使用 let 申明
- 变量默认是 immutable（不可变）的
- 使用 let mut 申明可变变量

### 常量

- 使用 const 申明
- 常量总是不能变
- 可以在任何作用域中申明，包括全局
- 约定是在单词之间使用全大写加下划线
- 编译时计算
- 在申明的作用域中，常量在整个程序生命周期中都有效

### 隐藏（shadowing）

- 定义一个与之前变量同名的新变量的行为，称为 shadowing
- mut 与 shadowing 有区别
    - 不使用 let 对 **不可变** 变量进行重新赋值时会报错
    - shadowing 实际创建一个新的变量，可以改变 **值** 类型，并且复用这个名字
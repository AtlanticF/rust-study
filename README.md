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

### 数据类型

#### 标量 scaler: 单独的值

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

#### 复合 compound: 多个值组合

- 元祖 tuple
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
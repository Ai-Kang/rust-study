# <center> rust学习
# 下载和安装
```shell
# rust官网下载
# 新增环境变量
CARGO_HOME   D:\develop_tool\rust\.cargobin
RUSTUP_HOME  D:\develop_tool\rust\.rustup
# 执行下载的文件开始安装
```
# 环境错误处理
```shell
# 执行代码错误 error: linker `link.exe` not found
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```
# 基础命令
```shell
# 查看版本
rustc --version
# 安装rust时会安装Cargo，使用如下命令检查版本
cargo --version
# 本地查看文档（浏览器）
rustup doc
# 编译代码生成二进制可执行文件
rustc main.rs
rustc -o 编译后名称 编译前名称.rs
# 编译代码生成库文件
rustc --crate-type lib 编译前名称.rs
```
# cargo的使用
```shell
# 创建项目
cargo new 项目名称
cargo new --lib 项目名称
# 构建项目(项目目录下)
cargo build --release
# 检测代码
cargo check
# 运行测试
cargo run
cargo test
```
# 项目结构
```shell
# 库
project_name
|__Cargo.toml
|__src
   |__lib.rs
# 常规项目
project_name
|__Cargo.toml
|__src
   |main.rs
```
# Cargo.toml文件
```toml
# 项目设置
[package]
# 项目名称
name = "rust-study"
# 项目版本
version = "0.1.0"
# 使用的rust版本
edition = "2021"
# 作者
authors = ["ak"]
# 项目依赖项，代码的包或者库叫做crate
[dependencies]
# 在此处引入使用的库
rand = "0.3.23"
# 只在开发中使用到的库
[dev-dependencies]
# 在此处引入使用的库
rand = "0.3.23"
# 只在正式环境中生效的库-一般不使用
[build-dependencies]
# 在此处引入使用的库
```
# 变量
```text
1：在rust中，使用let关键字来声明变量
2：rust支持类型推导，可以显示指定变量类型
    let x: i32 = 5;
    let x = 5;
3：变量名蛇形命名法（Snake Case），而枚举和结构命名使用帕斯卡命名法（Pascal Case），如果变量没有用到可以前置下划线，消除警告
4：强制类型转换 Casting aValue to a Different Type
    let a = 3.1;
    let b = a as i32;
5：打印变量（{}与{:?}）
    println!("val: {}",x);
    println!("val: {x}");
6：rust的变量默认是不可变的，如果要定义可变类型
    let mut a = 10;
7：rust可以隐藏变量
    let a = 5;
    let a = "abc";
```
# 常量
```text
1：常量的值必须是在编译时已知的常量表达式，必须指定类型和值
    const SECOND_HOUR: usize = 3600;
2：与C语言的宏定义不同，rust的const常量的值被直接嵌入到生成的底层机器代码中，而不是进行简单的字符替换
3：常量名与静态变量名全部大写，单词之间下划线分隔
4：常量的作用域是块级作用域，他们只在声明他们的作用域内可见
```
# 静态变量
```text
1：与const常量不同，static变量是在运行时分配内存的
2：时可变的，可以使用unsafe修改
    unsafe {
        // 不安全的代码
    }
3：静态变量的生命周期为整个程序的运行时间
    static MY_STATIC: I32 = 42;
```

# 数据类型
## 整数类型
```text
无符号整数类型：u开头，数字代表占的空间
有符号整数类型：i开头，数字代表占的空间
length  |  Signed  |  Unsigned
8-bit       i8          u8
16-bit      i16         u16
32-bit      i32         u32
64-bit      i64         u64
128-bit     i128        u128
arch        isize       usize     由程序运行的计算机决定，如果是64位计算机则是64位，32位计算机则是32位
i32::MAX 这两个方法可以查看大小支持
i32::MIN
```
## 整数字面值
```text
Number literals  |  Example  |  中文
Decimal             98_222       十进制
Hex                 0xff         十六进制
Octal               0o77         八进制
Binary              0b1111_0000  二进制
Byte(u8 only)       b'A'         byte类型
```
## 浮点数
```text
f32：32位，单精度
f64：64位，双精度
```
## 布尔值
```text
true :1位
false：1位
```
## 字符类型
```text
// 4:字符类型
char：4字节，单个字符，使用单引号
let x: char = 'z';
let y = '😊';
```
## 元组和数组
```text
元组和数组的相同点：
    1：元组和数组都是Compound types，而Vec和Map都是Collection types
    2：元组和数组长度都是固定的
```
### 元组
```text
tuples 可存储不同类型的数据类型
fn main() {
    // 创建元组
    let t1: (char, i32, i32) = ('a', 2, 3);
    // 获取元素
    println!("{}", t1.1);
}
```
### 数组
```rust
// 只能存储统一的数据类型
fn main() {
    let arr1: [char; 3] = ['a', 'b', 'c'];
    // 创建一个数组，内容是10个3
    let arr2 = [3; 10];
    // 获取元素
    arr1[1];
    // 获取长度
    arr1.len();
    for item in arr1 {
        println!("{}", item)
    }
}
```
# ownership(所有权)
```rust
fn main() {
    let arr = [1, 2, 3];
    let tup = (1, '2', 'a');
    // copy
    let arr_ownership = arr;
    let tup_ownership = tup;
    println!("arr {:?}", arr);
    println!("tup {:?}", tup);
    println!("arr_ownership {:?}", arr_ownership);
    println!("tup_ownership {:?}", tup_ownership);
    /*
        copy：拷贝
        move：移动
            string
     */
    let str1 = String::from("abc");
    let str2 = str1;
    // 此时会报错，因为str1不存在了
    println!("str1: {}", str1);
}
```
## 数据拷贝与移动
### copy
```rust
fn main(){
    let c1 = 1;
    // 此时执行了拷贝
    let c2 = c1;
}
```
### move
```rust
fn main(){
    let s1 = String::from("hello");
    // 此处执行了所有权转移
    let s2 = s1;
    // 在此处开始s1已经不存在了
    // 此处在执行了深拷贝，s2依然存在
    let s3 = s2.clone();
}
// =========================================================
fn main() {
    let s1: String = String::from("str1");
    // 此处也进行了所有权的转移，在函数执行结束后s1也被销毁了
    get_len(s1);
    // println!("{}",s1)
}

fn get_len(s: String) ->usize {
    return s.len();
}
```
## String和&str
```text
String是一个堆分配的可变字符串类型
pub struct String {
    vec: Vec<u8>,
}

&str是指字符串切片引用，是在栈上分配的
    不可变引用，指向存储在其他地方的UTF-8编码的字符串数据
    由指针和长度构成
    
String是具有所有权的，而&str并没有所有权
Struct中属性使用String
    如果不适用显示声明生命周期无法使用&str
    不只是麻烦，还有很多隐患
函数参数推荐使用&str（如果不想交出所有权）
    &str为参数，可以传递&str和String
    &String为参数，只能传&String，不能传&str
```
### &str转String
```text
fn main() {
    let s1: String = String::from("str1");
    let s2 = "你好".to_owned();
    let s3 = "你好".to_string();
}
```


# 匹配模式 match
```text
1: match关键字实现
2：必须覆盖所有的变体
3：可以用_、..=、三元(if)等来进行匹配
```

```rust
fn main() {
    let  number = 0;
    match number {
        0=> println!("Zero"),
        1|2 => println!("One or Two"),
        3..=9 => println!("From Three to Nine"),
        n if n % 2 == 0 => println!("Even number"),
        _=> println!("Other")
    }
}
// =========================================================
enum Color {
    Red,
    Blue,
    Null,
}

impl Color {
    fn print_color(&self) {
        match self {
            Color::Red => { println!("my_red") }
            Color::Blue => { print!("my_blue"); }
            Color::Null => { print!("is null"); }
        }
    }
}

fn main() {
    let a: Color = Color::Red;
    a.print_color()
}

```
# 结构体


## Ownership与结构体
```text
1: 所有值在rust中都应该有一个owner
2：在同一时刻只能有一个所有者
3：值在超出作用域时会自动销毁
每当将值从一个位置传递到另外一个位置时，borrow checker都会重新评估所有权
    1：immutable Borrow 使用不可变的借用，值的所有仍归发送方所有，接收方直接接收对该值的引用，而不是该值的副本。但是，他们不能使用该引用来修改它指向的值，编译器不允许这样做。释放资源的责任仍由发送方承担。仅当发送者本身超出范围时，才会删除值
    2：mutable Borrow 使用可变的借用所有权，删除值的责任也有发送者承担。但是接收方能够通过他们接收的引用来修改值。
    3：Move 这是所有权转移，所有权从一个地点转移到另外一个地点。borrow checker关于释放该值的决定将由该值的接收方通知。由于所有权已经送发送方转移到接收方，因此发送方在将引用移动到另一个上下文后不能再使用该引用，发送方再移动后对value的任何使用都会导致错误
    
结构体的关联函数的参数：
    &self: (self: &Self) // 不可变引用
    &mut self: (self: &mut Self) // 可变引用
    self：(self: Self) // Move
```
## 堆和栈 copy和move
```text
stack:
    1: 栈将按照获取值的顺序徐存储值，并以相反的顺序删除值
    2：操作高效，函数作用域就是再栈上
    3：栈上存储的所有数据都必须具有已知固定大小数据
stack上的数据类型：
    1：基础数据类型
    2：tuple和array
    3：struct与枚举等也是存储在栈上，如果属性有String等在堆上的数据类型会有指向堆的情况
heap：
    1：堆的规律性比较差，当把一些东西放到请求的堆上时，请求空间并返回一个指针，这是该位置的地址
    2：长度不确定
heap上的数据类型：
    Box、Rc、String、Vec等
一般来说在栈上的数据类型都是默认copy但是struct等默认为move，需要copy只需要设置数据类型实现Copy特质即可，或是调用Clone函数（需要实现Clone特质）
```
## Box指针
```text
Box是一个智能指针，它提供对堆分配内存的所有权。它允许你将数据存储到堆上而不是栈上，并且在复制或移动时保持堆数据的唯一拥有权。
    使用Box可以避免一些内存管理的问题，如悬垂指针和重复释放
    1：所有权转移
    2：释放内存
    3：解引用
    4：构建递归数据结构
```

## copy与clone
```text
Move：所有权转移
Clone：深拷贝
Copy：Copy是在Clone的基础建立的marker trait(Rust中最类似继承的关系)
1：trait：（特质）是一种定义共享行为的机制。Clone也是特质
2：marker trait 是一个没有任何方法的 trait，它主要用于向编译器传递某些信息，以改变类型的默认行为
```
## Box
```rust
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    // 使用box把数据放到堆上
    let p1 = Box::new(Point { x: 10.0, y: 20.0 });
    print!("x:{} y:{}", p1.x, p1.y);
}
```
# 流程控制
## if语句
```rust
fn main() {
    let a = 10;
    if a == 10 {
        println!("等于10");
    }else if a == 20{
        println!("等于20");
    } else {
        println!("等于其他");
    }
}
```
## if let
```rust
fn main() {
    let a: u8 = 3;
    if let 3 = a {
        print!("a is value 3");
    } else {
        print!("a is not 3");
    };
}
```
## match
```rust
fn main() {
    let number = 1;
    match number {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        4 => println!("4"),
        _ => println!("number")
    }
}
```

## loop 无限循环
```rust
fn main() {
    let mut counter: i32 = 0;
    loop {
        counter += 1;
        // 跳出循环
        if counter == 10 {
            break;
        };
        println!("{counter}")
    }
}
```
## while 条件循环
```rust
fn main() {
    let mut counter: i32 = 0;
    while counter <= 10 {
        counter += 1;
        // 跳过循环
        if counter == 5 {
            continue;
        };
        // 跳出循环
        if counter == 10 {
            break;
        };
        println!("{counter}")
    }
}
```
## for 条件循环
```rust
fn main() {
    let a: [i32; 5] = [10, 20, 30, 40, 50];
    for i in a {
        println!("{i}")
    }

    for i in a.iter() {
        println!("{i}")
    }

    // 完成倒计时
    for i in (1..4).rev() {
        println!("{i}")
    };
}
```

# 所有权
```text
所有权是rust最独特的特性，让rust无需GC就可以保证内存安全。
Rust的核心特性就是所有权
所有程序在运行时都必须管理它们，使用计算机内存的方式
    1：有些语言有垃圾收集机制，在程序运行时，他们会不断的寻找不在使用的内存进行释放
    2：有些语言没有自动垃圾回收机制，需要程序员显式的分配和释放内存
    3：rust通过所有权机制来管理内存，其中包含一组编译器在编译时的检查的规则
```
```text
Stack(栈)：按照接受值的顺序来存储，按照相反顺序来出栈，先进后出。
    存储在栈上的数据必须拥有已知大小。
heap(堆)：内存组织性相比于栈稍差一些，当把数据放入堆内存时会请求一定量的空间。
    操作系统在heap里找一块足够大小的空间，把它标记为在用，并且返回一个指针，指针指向内存头地址。
    这个过程叫做堆内存分配，也叫”分配“
```
```text
所有权规则：
    每个值都有一个变量，这个变量是该值的所有者
    每个值同时只能有一个所有者
    当所有者超出作用域时，该值会被删除
```
## String类型
```text
String比基础标量类型更复杂
字符串字面值：程序里手写的哪些字符串值，他们是不可变的
rust还有第二种字符串类型：String
    在heap上分配，能够存储在编译时位置大小的文本
```
```rust
fn main() {
    // 创建String
    let mut s1: String = String::from("Hello");
    // 追加值
    s1.push_str(",World");
    print!("{s1}");
}
```
# 函数
```rust
fn main() {
   // 定义字符串类型
    let s1 = String::from("Test move");
    // 这里执行了move操作，所有这行代码后s1变得不在可用
    take_ownership(s1);
    // 定义基础变量类型
    let a1: i32 = 10;
    // 基础类型实现了copy，所以这行代码后a1变量继续可用
    makes_copy(a1);

}

fn take_ownership(str1: String){
    println!("{str1}")
}

fn makes_copy(a1: i32){
    print!("{a1}");
}
```
```rust
fn main() {
    // 定义字符串类型
    let mut  s1 = String::from("Test move");
    // 这里发生了引用、借用，s1不会丢失对值的所有权
    let slen: usize = take_ownership(&mut s1);
    println!("{s1}的长度{slen}")
}

/**
  *&符号代表接收一个引用，也叫借用，调用者不会丢失堆传入参数的所有权,
  * 借用时默认不允许修改数据
  * 增加mut关键字时变为可变 (str1: &mut String)
  */
fn take_ownership(str1: &mut String) -> usize {
    str1.push_str("haha");
    return str1.len();
}
```
## 字符串切片 &str
```rust
fn main() {
    let str1 = String::from("aa bb");
    let len_str = first_world(&str1);
    println!("{len_str}");
    // 字符串切片
    let a = &str1[0..2];
    let b = &str1[3..5];
    println!("{str1}");
    println!("{a}");
    println!("{b}");
}

fn first_world(s: &str) -> &str {
    // 字符串转数组
    let str_arr: &[u8] = s.as_bytes();
    for (i, &item) in str_arr.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
```
# 结构体
## 定义struct
```rust
struct User {
    name: String,
    age: u32,
    active: bool,
}
```
## 实例化struct
```rust

```


## 使用结构体
```rust
struct User {
    name: String,
    age: u32,
    active: bool,
}

fn main() {
    let user1 = User {
        name: String::from("ak"),
        age: 27,
        active: true,
    };
    // 更新语法
    let user2 = User {
        name: String::from("ak1"),
        ..user1
    };
    // 使用字段
    println!("{}", user1.name)
}
```
## 元组结构体
```rust
fn main() {
    // 定义元组struct
    struct Color(i32, i32, i32);
    let color1 = Color(255, 255, 255);
}
```
## 结构体定义和属性
```text
结构体是一种用户定义的数据类型，用于创建自定义的数据结构
定义：
    struct Point{
        x: i32,
        y: i32,
    }
每条数据的(x和y)称为属性、字段(field)
通过点(对象.xx)来访问结构体中的属性
```
## 结构体的方法
```text
结构体的方法是指，通过实例调用(&self、&mut self、self)
impl Point{
    fn 方法名(&self,other: &Point) -> f64 {
        // 方法体
        return 返回值;
    }
}
```
## 结构体关联函数
```text
关联函数是与类型相关联的函数，调用时为结构体名称::函数名()
impl Point {
    fn new(x: u32,y: u32) -> Self {
        return Point {x,y}
    }
}
```
## 关联变量
```text
关联变量是指，和结构体类型相关联的变量，也可以在特质或是枚举中
impl Point {
    const PI: f64 = 3.14159
}
调用：Point::PI
```

# 枚举
```text
枚举(enums) 是用户自定义的数据类型，用于表示具有一组离散可能值的变量
    每种可能值都称为“variant”（变体）
    枚举名::变体名
枚举的好处
    可以使代码更加严谨、更易读
    More robust programs
```
```rust
// 创建枚举
enum Shpe {
    Circle(f64,f32),
    Rectangle(f64),
    Square(f32),
}
```
## 常用枚举
```rust
pub enum Option<T> {
    None,
    Some(T),
}
pub enum Result<T,E> {
    Ok(T),
    Err(E),
}
```
# rust代码组织
```text
代码组织：
    哪些细节可以暴漏，哪些细节需要私有
    哪些作用域内有效
模块系统：
    package(包): cargo的特性，让构建、测试、共享crate
    crate(单元包)：一个模块树，它可以产生一个library或可执行文件
    module(模块)：use、让程序员控制代码的组织、作用域、私有路径。
    path(路劲)：为struct、function、module等命名的方式
```
## package和crate
```text
crate的类型：
    binary
    library
crate root:
    是源代码文件
    rust编译器从这里开始，组成你的crate根module
一个package：
    包含一个Cargo.toml,它描述了怎么来构建这些Crates
    只能包含0~1个library Crate
    可以包含任意数量的binary Crate
    必须包含至少一个crate (library 或 binary)
```
## module
### 定义模块
```rust
pub mod A {
    pub mod B {
        pub mod C {
            pub fn cfn() {
                println!("cfn")
            }
        }
    }
}

fn main() {
    // 绝对路径调用
    crate::A::B::C::cfn();
    // 相对路径
    A::B::C::cfn();
}
```
## use关键字
```rust
pub mod A {
    pub mod B {
        pub mod C {
            pub fn cfn() {
                println!("cfn")
            }
        }
    }
}

use A::B::C as C;
// 对于外部暴露
// pub use A::B::C as C;
fn main() {
    // 使用use引用调用
    C::cfn();
}
```
## 批量引入
```rust
use std::{
    io,
    cmp::Ordering,
};
use std::clone::*;
fn main() {}
```
# 常用集合
## Vector
```text
Vec<T>,叫做vector
    是由标准库提供的
    可以存储多个值
    只能存储相同类型的值
    值在内存中连续存放
```
```rust
fn main() {
    // 创建Vec
    let v1: Vec<i32> = Vec::new();
    // 使用宏创建
    let mut v2: Vec<i32> = vec![1, 2, 3];
    // 添加元素
    v2.push(10);
    // 读取
    let vl1 = &v2[1];
    let vl2 = v2.get(2);
    println!("{}", {vl1});
}
```
## String

## HashMap



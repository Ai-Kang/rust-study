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
## 使用结构体
```rust
enum Flavor {
    Spicy,
    Sweet,
    Fruity,
}

struct Drink {
    flavor: Flavor,
    price: f64,
}

impl Drink {
    // 关联变量
    const MAX_PRICE: f64 = 20.0;
    // 方法，不可变参数
    fn print_drink(&self) {
        match self.flavor {
            Flavor::Sweet => println!("Sweet" ),
            _=>println!("null")
        }
    }
    // 关联函数
    fn new(price: f64) -> Self {
        return Drink {
            flavor: Flavor::Fruity,
            // price: price,
            price,
        }
    }
}

fn main() {
    let sweet = Drink {
        flavor: Flavor::Sweet,
        price: 8.8,
    };
    // 调用函数
    sweet.print_drink();
    // 调用关联函数
    let fruity = Drink::new(12.0);
    fruity.print_drink();
}

```
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
## switch


# 函数






### 	2:if let

```rust
处理只关心一种匹配而忽略其它匹配的情况
fn main() {
    if let 1 = 1{
        println!("hhhh");
    } else {
        println!("aaaaa");
    }
}

```

### 	3:loop(无限循环)

```rust
// 循环语句 ，使用continue跳过本次循环，使用break结束循环 
loop{
   println!("猜一个数字！");
   // 定义变量 可变的 变量名 = 类型::方法,rust中默认是不可变的immutable
   let mut guess = String::new();
   // io库中的stdin输入方法   read_line(放入哪个变量) 读取一行  expect(错误信息)
  io::stdin().read_line(&mut guess).expect("无法读取");
  // 输入转换为数字，其实是使用了shadow隐藏了之前的变量，后把变量类型转换了
  let guess: u32 = match guess.trim().parse() {
                   Ok(num) => num,
                   // 如果转换错误则重新输入
                  Err(_) => continue
                 };
  //  match：使用返回值来决定执行的语句
  match guess.cmp(&secret_number) {
        Ordering::Greater => println!("太大了"),
        Ordering::Less => println!("太小了"),
        Ordering::Equal => {
          println!("回答正确");
          break;
        }
  };
}
```

### 	4:while(条件循环)

```rust
fn main() {
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }
}

```

### 	5:for(条件循环)

```rust
fn main() {
    let a: [i32; 6] = [10,20,30,40,50,60];
    for i in a.iter() {
        println!("{i}");
    }
}

fn main() {
    let a: [i32; 6] = [10,20,30,40,50,60];
    for (i,v) in a.iter().enumerate() {
        println!("{v}");
    }
}
```

### 	6:match（控制流运算符）

````rust
允许一个值与一个系列模式进行匹配，并执行匹配的模式对应的代码
模式可以是字面值、变量名、通配符.....
match 变量 {
    对比值1 => 返回结果1,
    对比值2 => {
        返回结果2
    },
    _ =>{默认处理};
}
````

### 	7:unwrap

```rust
unwrap: match表达式的一个快捷方法

let fileResult = File::open("E:/a.txt").unwrap();
```

### 	8:expect

```rust
fn main() {
    let fileResult = File::open("E:/a.txt").expect("无法打开文件");
}
```



## 4:函数

### 	1:函数定义1

```rust
// 程序入口
fn main() {
    // 函数体
}
// 函数声明使用fn关键字，推荐标准为所有字母小写，多个单词下划线隔开
fn test_t1() {
     // 函数体
}
```

### 	2:函数参数

```rust
// 函数分为形参和实参两种，形参是定义，实参是调用时传入
fn main() {
    test1(10,20);
}

fn test1(x: i32,y:i32) {
    println!("{},{}", x,y);
}
```

### 	3:返回值

```rust
// 带返回值函数定义
fn test1() -> i32{
    // 返回5
    5
}
fn test2() -> i32{
    // 返回5
    return 5;
}

fn main() {
    let i = test1();
    println!("{}",i);
}



```

## 4:所有权

```rust
// rust的核心特性就是所有权
// 所有程序在运行时都必须管理他们使用计算机内存的方式
//    rust采用所有权系统来管理，其中包含一组编译器在编译时检查的规则
//    当程序运行时，所有权特性不会减慢程序的运行速度

Stack  vs  Heap
栈内存  vs  堆内存

Stack----
在rust的系统级编程语言里，一个值在stack上还是在heap上对语言的行为和程序员为什么要做某些决定是有很大影响的
在代码运行时，stack和heap都是程序员可以用的内存，但是他们的结构很不相同

stack会按照接收顺序存储，按照相反顺序移除，先进后出
	添加数据--压栈
	移除数据--弹栈
存储在stack上的数据必须是拥有已知并且是固定的大小，编译时大小未知的数据或运行时大小可能发生变化的数据必须存放在heap上

Heap----
当程序员吧数据放入heap时，就会请求一定数量的空间
操作系统在heap里找到一块足够大的空间把他标记为在用，并且返回一个指针，也就是一个空间地址
这个过程叫做在heap上进行分配，又是仅仅称为分配

所有权存在的原因----
	跟踪代码的哪些部分正在使用heap的哪些数据
	最小化heap上的重复数据量
	清理heap上未使用的数据以避免空间不足
```

### 		1:所有权的规则

```rust
每个值都有一个变量，这个变量是该值的所有者
每个值同时只能有一个所有者
当所有者超出作用域（scope）时，该值将被删除
```

### 		2:所有权与函数

```rust
在语义上，将传递给函数和把值复制给变量是类似的：
	将值传递给函数将发生移动或复制（实现Copy的是复制）
```

### 	3:引用和借用

```rust
&符号表示引用
	&String
借用的数据不允许修改，可以加mut来表示可变，&mut String，传参也加mut
fn main() {
    let mut s1 = String::from("abcd");
    // get_len(s1);
    // get_len(&s1);
    let len: usize = get_len(&mut s1);
    println!("{len}");
}

fn get_len(str: &mut String) -> usize {
    return str.len();
}
```

### 	4:悬空引用(Dangling References)

```rust
一个指针引用了一个内存中的某个地址，而这块内存地址可能已经释放并分配给其它人使用。
在rust里编译器可以保证引用永远都不是悬空引用
	如果你引用了某些数据，编译器将保证在引用离开作用域之前数据不会离开作用域
```

## 5:切片

### 	1:字符串切片

```rust
字符串切片
[开始索引..结束索引] 包左不包右
fn main() {
    let mut s1 = String::from("abcd aikang");
    let x = first_world(&s1);
    println!("{x}");
}

fn first_world(s: &String) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
```

### 	2:字符串字面值

```rust
字符串字面值实际就是切片
let s:&str = "hello world!";

fn main() {
    let mut s1 = String::from("abcd aikang");
    let x = first_world(&s1);
    println!("{x}");
}

fn first_world(s: &str) -> &str {
    let bytes: &[u8] = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    return &s[..];
}
```

### 	3:其他类型切片

```rust
fn main() {
    let number: [i32; 6] = [1, 2, 3, 4, 5, 6];
    let lite: [i32] = number[..];
}
```


### 	1:函数实现

```rust
// 创建结构体
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

// 实现结构体方法 self就是本身相当于this
impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.length;
    }
}
 
fn main() {
    let r1 = Rectangle {
        width: 10,
        length: 20,
    };
    let area: u32 = r1.area();
    println!("{area}");
}
```

### 2:关联函数(静态函数)

```rust
// 创建结构体
#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}

// 实现结构体方法 self就是本身相当于this
impl Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.length;
    }
    // 关联函数
    fn square(size: u32) -> Rectangle {
        return Rectangle {
            width: size,
            length: size,
        };
    }
}

fn main() {
    let r1 = Rectangle {
        width: 10,
        length: 20,
    };
    let area: u32 = r1.area();
    println!("{area}");
    // 调用关联函数
    Rectangle::square(20);
}
```



## 8:系统模块

```rust
Package（包）:Cargo的特性，让构建、测试、共享crate
Crate（单元包）：一个模块树，它可能产生一个library或可执行文件
Module（模块）、use：让你控制代码的组织、作用域、私有路径
Path（路径）：为struct、function或module等项命名的方式
```

### 	1:package（包）和Crate

```rust
Crate的类型：
	binary
	library
Crate Root：
	是源码文件
	Rust编译器从这里开始，组成你的Crate的根Module
一个Package
	包含1个Cargo.toml，它描述了如何构建这些Crates
	只能包含0-1个library crate
	可以包含任意数量的binary crate
	但必须至少包含一个crate（library 或 binary）
src/main.rs: 二进制可执行文件
src/lib.rs: 库文件 
```

### 	2:module

```rust
module：
	在一个crate内，将代码进行分组
	增加可读性，易于复用
	控制项目（item）的私有性，public、private
建立module：
	mod 关键字
	mod mod_name{   
	}
	可嵌套
	可以包含其他项的定义（struct、enum、常量、trait、函数等）；

```

### 	3:路径

```rust
为了Rust的模块中找到某个条目，需要使用路径。
路径分为两种：
	绝对路径：从crate root开始，使用crate名或字面值crate
	相对路径：从当前模块开始，使用self(本身)、super(上一级)或当前模块的标识符
路径至少由一个标识符组成，标识符之间使用::分割
```

```rust
// 默认是私有的
pub mod A1 {
    pub mod A11 {
        pub fn a_fn() {}
    }
}
fn main() {
    // 绝对路径调用
    crate::A1::A11::a_fn();
    // 相对路径：因为main和A1是平级所以可以直接调用
    A1::A11::a_fn();
}
```

```rust
// 默认是私有的
mod A1 {
   fn a_fn{
       // 通过父级调用
       super::serve();
   }
}
fn serve(){}
```

### 	4:将模块内容移动到其他文件(分文件写)

```rust
模块定义时，如果模块名后边是“;”，而不是代码块，rust会从与模块名相同的文件中加载内容，模块树的结构不会变化
```

## 9:常用集合

### 	1:Vector

```rust
由标准库提供
可存储多个值
只能存储相同类型的数据
值在内存中连续存放
Vec::new();

use std::vec;
fn main() {
    let v1: Vec<i32> = Vec::new();
    let mut v2: Vec<i32> = vec![1, 2, 3];
    // 添加元素
    v2.push(10);
    // 删除元素
    v2.remove(2);
    // 获取数据-下标
    let number1: &i32 = &v2[1];
    let number2 = v2.get(0);
    // 清空
    v2.clear();
}
```

### 	2:String

```rust
rust的核心语言层面,只有一个字符串类型：字符串切片str(或&str)
字符串切片：
	对存储在其它地方、utf-8编码的字符串的引用
	字符串字面值：存储在二进制文件中，也是字符串切片
String来自标准库，
可增长、可修改、可拥有
utf-8编码
```

```rust
 // 创建string字符串
let mut a:String = String::new();
let mut b:String = String::from("hello");
// 追加
b.push_str(",world");
// 获取字符串字符数组
let bytes = b.as_bytes();
```

```rust
use std::fmt::format;

fn main() {
    // 创建
    let s1: String = String::new();
    let s2: String = "abc".to_string();
    let mut s3: String = String::from("abc");
    // 更新String
    s3.push_str("abc");
    s3.push('a');
    // 字符串拼接
    s3 = s3 + &s2;
    let mut s4: String = format!("{}-{}-{}", s1, s2, s3);
    // 按照索引进行访问
    for b in s4.bytes() {
        println!("{b}");
    }

    for b in s4.chars() {
        println!("{b}");
    }

    println!("{s2}");
}
```

### 3:HashMap

```rust
键值对的形式存储数据，一个键(key)对应一个值(value)
Hash函数：决定如何在内存中存放k和v
适用场景：通过k(任何类型)来寻找数据，而不是通过索引
```

````rust
use std::collections::HashMap;

fn main() {
    // 创建
    let mut scores: HashMap<&str, i32> = HashMap::new();
    // 添加数据
    scores.insert("a", 10);
    scores.insert("b", 20);

    // 第二种创建方法
    // vec-k
    let teams = vec!["a", "b"];
    // vec-v
    let vs = vec![10, 20];
    let scores1: HashMap<_, _> =
        // 合并
        teams.iter().zip(vs.iter())
            // 创建
            .collect();
    // 访问
    let v1 = scores1.get("a");
    let v1v = match v1 {
        None => {}
        Some(s) => s
    };
    // 更新-替换
    scores.insert("b", 30);
    // 更新-忽略
    scores.entry("b").or_insert(30);
}
````

## 9:错误处理

### 	1:可恢复错误

```rust
Result<T,E>

fn main() {
    let fileResult: Result<File, Error> = File::open("E:/a.txt");
    // 文件打开返回的结果分支，最终需要file文件，所以用File接收
    let open_file: File = match fileResult {
        // 成功直接返回File
        Ok(oFile) => oFile,
        // 失败时判断是什么错误， 获取到kind
        Err(oErr) => {
            let errorKind: ErrorKind = oErr.kind();
            // 判断是否未找到文件
            match errorKind {
                ErrorKind::No tFound => {
                    // 创建文件
                    let createFileResult = File::create("E:/a.txt");
                    // 判断是否添加成功
                    match createFileResult {
                        Ok(createFile) => createFile,
                        Err(_) => panic!("文件创建失败"),
                    }
                }
                _ => { panic!("文件打开失败") }
            }
        }
    };
    // 使用打开文件
    println!("{:?}", open_file);
}
```

### 	2:不可恢复错误

```rust
panic! 宏
# 系统遇到错误时直接退出，稍后有系统来清理内存
[profile.release]
panic = 'abort'
```

### 	3:返回给调用者错误处理

```rust
fn main() {
    let open = open_file();
}

fn open_file() -> Result<File, Error> {
    let open_file = File::open("./A.txt");
    let mut open_file = match open_file {
        Ok(f) => Ok(f),
        Err(e) => Err(e),
    };
    return open_file;
}
```

```rust
fn main() {
    let open = open_file();
}

fn open_file() -> Result<File, Error> {
    let open_file = File::open("./A.txt")?;
    return Ok(open_file);
}
```



## 10:关键字

## use(引用)

```rust
// 引用标准库中的io，程序默认导入的库是prelude
use std::io;
use std::{
    io;
    cmp;
}
// 指定别名
as use std::io::Result as IoResult;
pub use std::io::Result as IoResult;
// 公开的
pub
```

# 3:标准库

## 1:Prelude(预导入模块)

### 	1:Option枚举

````rust
描述了某个值可能存在（某种类型）或不存在的情况
Rust没有NULL
enum Option<T> {
    Some(T),
    None,
}
````



## 2:std

### 		1:io

```rust
use std::io;

// 定义变量 可变的 变量名 = 类型::方法,rust中默认是不可变的immutable
let mut guess = String::new();
// 标准输入 io库中的stdin输入方法   read_line(放入哪个变量) 读取一行  expect(错误信息)
io::stdin().read_line(&mut guess).expect("无法读取");
```

### 		2:cpm(比较)

```rust
// 枚举
use std::cmp::Ordering;

match guess.cmp(&secret_number) {
            Ordering::Greater => println!("太大了"),
            Ordering::Less => println!("太小了"),
            Ordering::Equal => {
                println!("回答正确");
                break;
            }
        };
```

### 	3:Range

```rust
// 生成1-3的数， rev反转数
for i in (1..4).rev() {
	println!("{i}");
}

let range = (1..4);
for i in range.rev() {
	println!("{i}");
}
```

## 3:宏

```rust
// 打印
println!("{s2}");
// 字符串拼接
let mut s4: String = format!("{}-{}-{}", s1, s2, s3);
// 错误处理
panic!("系统错误");
```



# 4:第三方库

## rand(随机数)

```rust
// 打开Cargo.toml 在[dependencies] 下加入 库名 = "版本" 如果在版本前加^则向上兼容的版本都可以
rand = "^0.3.23"

use rand::Rng; // trait 是一个接口
// 生成随机数,包前不包后
let secret_number = rand::thread_rng().gen_range(1,101);
```




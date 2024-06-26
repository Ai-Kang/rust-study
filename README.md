# 1:下载安装及编译

## 1:下载安装

```sh
# 增加环境变量(创建好对应的文件夹)
CARGO_HOME   D:\develop_tool\rust\.cargobin
RUSTUP_HOME  D:\develop_tool\rust\.rustup
# 查看版本
rustc --version
# 本地查看文档（浏览器）
rustup doc

# error: linker `link.exe` not found
rustup toolchain install stable-x86_64-pc-windows-gnu
rustup default stable-x86_64-pc-windows-gnu
```

## 2:编译

```sh
rustc main.rs
```

## 3:包管理器和系统构建工具(Cargo)

```sh
# Cargo是rust的构建系统和包管理工具，构建代码、下载依赖库、构建库等
# 安装rust时会安装Cargo，使用如下命令检查版本
cargo --version
```

## 4:项目创建及文件说明

```sh
cargo new 项目名称
# 生成的文件
Cargo.toml -- peizhi 配置文件
··········
    [package]  # 是一个区域标题，表示下方内容是用来配置包的
    name = "day-01"  # 项目名称
    version = "0.1.0" # 项目版本
    edition = "2021"  # 使用的rust版本
    authors = ["ak"] # 作者
    # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
    [dependencies]  # 项目依赖项，代码的包或者库叫做crate
    # 打开Cargo.toml 在[dependencies] 下加入 库名 = "版本" 如果在版本前加^则向上兼容的版本都可以
    rand = "0.3.23"
··········

Cargo.lock # 负责追踪项目依赖的精确版本，不需要手动修改该文件
src # 源代码都应该放在src下
	main.rs # 源代码 main函数是入口函数
```

## 5:项目构建

```sh
cargo build
cargo build --release # 为发布构建
# 创建可执行文件：target/debug/项目名称  或者target/debug/项目名称.exe
# 会生成Cargo.lock
```

## 6: 代码检查

```sh
cargo check
# cargo check 检查代码，确保能通过编译，但是不产生任何可执行文件
# cargo check 要比cargo build快很多，编译代码的时候可以反复使用cargo check检查代码提高效率
```

## 7:项目运行

```sh
cargo run
# 构建并运行项目，如果源代码没有更改则不会编译会直接生成二进制
```

# 2:语言基础

## 1:数据类型

### 		1:标量类型

```rust
// 1:整数类型
无符号整数类型：u开头，数字代表占的空间
有符号整数类型：i开头，数字代表占的空间

length  |  Signed  |  Unsigned
8-bit       i8          u8
16-bit      i16         u16
32-bit      i32         u32
64-bit      i64         u64
128-bit     i128        u128
arch        isize       usize     由程序运行的计算机决定，如果是64位计算机则是64位，32位计算机则是32位
// 1.1:整数字面值
Number literals  |  Example  |  中文
Decimal             98_222       十进制
Hex                 0xff         十六进制
Octal               0o77         八进制
Binary              0b1111_0000  二进制
Byte(u8 only)       b'A'         byte类型

// 2:浮点类型
f32：32位，单精度
f64：64位，双精度

// 3:布尔类型
true :1位
false：1位

// 4:字符类型
char：4字节，单个字符，使用单引号
let x = 'z';
let y = '😊';
```



### 		2:复合类型

```rust
// 1：元组（Tuple）
//  Tuple可以将多个数据类型的多个值放在一个类型中。
// Tuple的长度是固定的，一旦声明就无法改变
// 创建：在小括号内用逗号隔开
// Tiple中的每个位置都对应一个类型，Tuple中各个元素的类型不必相同
let tup: (i32, f64, u8) = (500, 6.4, 1);
// 取值1,解构
let (x, y, z) = tup;
// 取值2,下表
println!("{},{},{}", tup.0, tup.1, tup.2);

// 2：数组
// 数组可以将相同类型的多个值放入一个类型中
// 数组的长度是固定的
// 创建：在中括号中用逗号隔开
// 数组的好处：如果要让数据存放在stack(栈)而不是heap(堆)中，或者想保证有固定数量的元素。
let arr: [&str; 3] = ["周一", "周二","星期三"];
// 访问
let first = arr[0];
```

### 	3:字符串切片(&str)

````rust
let mut s1 = String::from("abcd aikang");
let str1:&str = &s1[0..3];
println!("{str1}");
````





## 2:变量与常量

```rust
// 声明变量使用let关键字，默认情况下变量是不可变的(lmmutable)，如果需要变量可变则在变量声明是加mut修饰

// let 变量名:变量类型 = 变量值;
let a:u32 = 10;
let mut b:u32 = 20;

// 常量：使用const关键字，不可以使用mut关键字，不可改变
// const 常量名:类型 = 常量值;
const C_NUMBER:u32 = 30;
```

## 3:隐藏(Shadowing)

```rust
// 隐藏是会复用之前的变量名并且会顶替之前的同名变量
let a:u32 = 10;
let a:String = String::new();
```



## 3:控制语句

### 		1:if

```rust
// if表达式允许根据条件执行不同分支，条件类型时bool类型
// if表达式中与条件相关的代码叫做分支(arm)
// if后边可选加上else表达式

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
// if是一个表达式，可以放在let右边作为赋值
let b = if a == 10 { a } else { 20 };
```

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
    // getLen(s1);
    // getLen(&s1);
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

## 6:struct

```rust
fn main() {
    // 初始化结构体
    let mut p1 = Person {
        name: String::from("ak"),
        age: 20,
        sex: false,
    };
    // 访问结构体
    println!("username: {},age: {},sex: {}",p1.name,p1.age,p1.sex);
}

/*
 定义结构体
*/
struct Person {
    name: String,
    age: u32,
    sex: bool,
}
```

```rust
/*
 定义结构体
 #[derive(Debug)] 默认打印    {:?} or {:#?}
*/
#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    sex: bool,
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

## 7:枚举

```rust
不可变的类型可以定义枚举
可以使用impl来定义方法
/*
 创建枚举值
*/
enum ApAddrKind {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    // 使用枚举
    let four: ApAddrKind = ApAddrKind::V4(127, 0, 0, 1);
    route(four);
}

// 方法中使用枚举
fn route(ip_kind: ApAddrKind) {}
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




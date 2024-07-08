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

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

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
    // 使用字段
    println!("{}",user1.name)
}

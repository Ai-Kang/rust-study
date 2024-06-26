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

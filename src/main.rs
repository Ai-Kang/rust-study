struct Point {
    x: f64,
    y: f64,
}


fn main() {
    // 使用box把数据放到堆上
    let p1 = Box::new(Point { x: 10.0, y: 20.0 });
    print!("x:{} y:{}", p1.x, p1.y);
}

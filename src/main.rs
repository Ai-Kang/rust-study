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

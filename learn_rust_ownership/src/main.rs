//rust 所有权学习

fn main() {
    let a: f32 = 3.1416;
    print(a);
    println!("{a}");
    let b = a;
    println!("{a}");

    println!("Hello, world!");
}

fn print(x: f32) {
    println!("{x}");
}

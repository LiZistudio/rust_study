fn main() {
    let name = String::from("zaohua");
    let name2 = &name[0..3];
    println!("Hello, world!{}", name2);
}

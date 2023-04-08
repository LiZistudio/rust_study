use ferris_says::say;
use std::io::{stdout, BufWriter};

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn wider(&self, rect: &Rectangle) -> bool {
        self.width > rect.width
    }
}

impl Test for Rectangle{
    fn print(&self) {
        println!("width:height={:?}:{:?}",self.height,self.width);
    }
}
pub trait Test {
    fn print(&self) {}
}

fn main() {
    let stdout = stdout();
    let message = String::from("你好,rust!");
    let width = message.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message, width, &mut writer).unwrap();


    
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 40, height: 20 };

    println!("{}", rect1.wider(&rect2));
    println!("{}", rect1.area());

    rect1.print();
    rect2.print();    
}

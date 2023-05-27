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
    let message1 = String::from("江南游子，把吴钩看了，栏杆拍遍。无人会，登临意。");
    let message2 = String::from("    醉里挑灯看剑，梦回吹角连营。八百里分麾下炙，五十弦翻塞外声。沙场秋点兵。");
    let width1 = message1.chars().count();
    let width2 = message2.chars().count();

    let mut writer = BufWriter::new(stdout.lock());
    say(&message1, width1, &mut writer).unwrap();
    say(&message2, width2, &mut writer).unwrap();



    
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 40, height: 20 };

    println!("{}", rect1.wider(&rect2));
    println!("{}", rect1.area());

    rect1.print();
    rect2.print();    
}

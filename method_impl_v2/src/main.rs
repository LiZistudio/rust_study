/*************************带有更多参数的方法*****************************/

fn main() {
    let rec1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rec2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rec3 = Rectangle {
        width: 60,
        height: 20,
    };

    println!("rec1能包含rec2吗?answer is {};rec1's area is {},rec2's area is {}.", rec1.can_hold(&rec2),rec1.area(),rec2.area());
    println!("rec2能包含rec3吗?answer is {};rec2's area is {},rec3's area is {}.", rec2.can_hold(&rec3),rec2.area(),rec3.area());
}


struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> u32 {
        self.width
    }

    fn height(&self) -> u32 {
        self.height
    }

    fn can_hold(&self,rec:&Rectangle) -> bool {
        self.width > rec.width && self.height > rec.height
    }
}
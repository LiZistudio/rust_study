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

    let rec4 = Rectangle::new(10,20);
    println!("rec4's area is {:?}", rec4.area());

    //创建一个正方形
    let rec5 = Rectangle::creat_square(5);
    let str1 = if rec5.height == rec5.width {
                            "正方形".to_string()
                        } else {
                            "长方形".to_string()
                        };
    println!("rec5 is a {:?} that edge is {:?}",str1,rec5.area());

    println!("rec1能包含rec2吗?answer is {};rec1's area is {},rec2's area is {}.", rec1.can_hold(&rec2),rec1.area(),rec2.area());
    println!("rec2能包含rec3吗?answer is {};rec2's area is {},rec3's area is {}.", rec2.can_hold(&rec3),rec2.area(),rec3.area());
}


struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn creat_square (edge:u32) -> Rectangle {
        Rectangle::new(edge, edge)
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }

    fn can_hold(&self,rec:&Rectangle) -> bool {
        self.width > rec.width && self.height > rec.height
    }
}
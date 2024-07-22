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
    let rec5 = Rectangle::creat_square(5);      //关联函数

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

    fn _get_width(&self) -> u32 {
        self.width
    }

    fn _get_height(&self) -> u32 {
        self.height
    }

    fn can_hold(&self,rec:&Rectangle) -> bool {
        self.width > rec.width && self.height > rec.height
    }
}

//*********************************多哥impl块*************************************
//每个结构体都允许拥有多个 impl 块。
/*impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}*/
//这是将上面impl块中的方法进行了拆分，放在了不同的impl块中。但是这里没有理由将这些方法分散在多个 impl 块中，不过这是有效的语法。
//第 10 章讨论泛型和 trait 时会看到实用的多 impl 块用例。

//总结
//结构体让你可以创建出在你的领域中有意义的自定义类型。通过结构体，我们可以将相关联的数据片段联系起来并命名它们，这样可以使得代码更加清晰。
//在 impl 块中，你可以定义与你的类型相关联的函数，而方法是一种相关联的函数，让你指定结构体的实例所具有的行为。
//但结构体并不是创建自定义类型的唯一方法：让我们转向 Rust 的枚举功能，为你的工具箱再添一个工具。
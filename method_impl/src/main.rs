/******************方法语法*********************/

/***********方法与函数类似：它们使用 fn 关键字和名称声明，可以拥有参数和返回值，同时包含在某处调用该方法时会执行的代码。
 * 不过方法与函数是不同的，因为它们在结构体的上下文中被定义（或者是枚举或 trait 对象的上下文，将分别在第 6 章和第 17 章讲解），
 * 并且它们第一个参数总是 self，它代表调用该方法的结构体实例。*************/
 
//定义方法
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn new (width:u32,height:u32) -> Rectangle {
        Rectangle { width, height }
    }

    fn area (&self) -> u32 {
        self.width * self.height
    }

    fn width(&self) -> bool {
        self.width > 0
    }

    /*与字段同名的方法将被定义为只返回字段中的值，而不做其他事情。这样的方法被称为 getters，Rust 并不像其他一些语言那样为结构字段自动实现它们。
    Getters 很有用，因为你可以把字段变成私有的，但方法是公共的，这样就可以把对字段的只读访问作为该类型公共 API 的一部分。*/
    fn height (&self) -> u32 {
        self.height
    }
    
}
//Rust 并没有一个与 -> 等效的运算符；相反，Rust 有一个叫 自动引用和解引用（automatic referencing and dereferencing）的功能。
//方法调用是 Rust 中少数几个拥有这种行为的地方。

fn main() {
    println!("👉 method and impl👈");

    //let rect1 = Rectangle { width: 30, height: 50 };
    let rect1 = Rectangle::new(10,10);
    let rect2 = Rectangle::new(0,5);
    println!("矩形的长宽分别为 👉 {:?}👈 ,面积为 👉 {}👈 。",rect1, rect1.area());
    if rect2.width() {
        println!("该矩形宽边长度为{:?}",rect1.width);
    }else {
        println!("矩形宽边为零,Error!!!");
    }

    println!("该矩形高为：{:?}",rect1.height());
}
/*
枚举和模式匹配
本章介绍 枚举（enumerations），也被称作 enums。枚举允许你通过列举可能的 成员（variants） 来定义一个类型。
首先，我们会定义并使用一个枚举来展示它是如何连同数据一起编码信息的。
接下来，我们会探索一个特别有用的枚举，叫做 Option，它代表一个值要么是某个值要么什么都不是。
然后会讲到在 match 表达式中用模式匹配，针对不同的枚举值编写相应要执行的代码。
最后会介绍 if let，另一个简洁方便处理代码中枚举的结构。
枚举是一个很多语言都有的功能，不过不同语言中其功能各不相同。Rust 的枚举与 F#、OCaml 和 Haskell 这样的函数式编程语言中的 代数数据类型（algebraic data types）最为相似。
 */

//定义枚举
// enum IpAddrKind {
//     Ipv4,
//     Ipv6,
// }

//直接将数据附加到枚举的每个成员上，这样就不需要一个额外的结构体了。
enum IpAddrKind {
    Ipv4(String),
    Ipv6(String),
}

//枚举值

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

fn main () {
    //创建IpAddrKind的两个成员的不同实例
    let four = IpAddrKind::Ipv4;
    let six = IpAddrKind::Ipv6;
    
    // let home = IpAddr {
    //     kind: IpAddrKind::Ipv4,
    //     address: String::from("127.0.0.1"),
    // };
    let home = IpAddrKind::Ipv4(String::from("127.0.0.1"));
    // let loopback = IpAddr {
    //     kind: IpAddrKind::Ipv6,
    //     address: String::from("::1"),
    // };
    let loopback = IpAddrKind::Ipv6(String::from("::1"));

    let m1 = Message::ChangeColor(255, 255, 255);
    let m = Message::Write(String::from("醉里挑灯看剑，梦回吹角连营"));
    m.call();

    //Option<T>中Some(T)的一些示例
    let some_number = Option::Some(5);
    let some_string = Option::Some("a string");
    let absent_number: Option<i32> = Option::None;
}

//定义一个函数来获取任何的IpAddrKind
// fn route (ip_type:IpAddrKind) {

// }

//成员中内嵌了多种多样的类型的枚举：
enum Message {
    Quit,
    //Move(Position),
    Move{x:i32,y:i32},//Move内嵌一个匿名结构体
    Write(String),
    ChangeColor(i32,i32,i32),
}
// struct Position {
//     x:i32,
//     y:i32,
// }
struct QuitMessage; // 类单元结构体
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // 元组结构体
struct ChangeColorMessage(i32, i32, i32); // 元组结构体

//枚举和结构体一样，都可以实现impl块
impl Message {
    fn call (&self) {
    }
}

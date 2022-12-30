use std::net::Ipv4Addr;

enum IpAdrKind {
      V4(String),
      V6(String)
}


fn main() {
    let m1 = Message::More { x: 1, y: 2 };
    Message::some_func();
    
    let ipv4 = IpAdrKind::V4(String::from("192.168.1.1"));
    let ipv6 = IpAdrKind::V6(String::from("192.168.2.1"));    

}

enum Message {
    Quit,                       //不存放数据
    More{x:i32,y:i32},          //存放一个匿名结构体
    Write(String),              //存放一个字符串
    ChangeColor(i32,i32,i32)    //存放三个整数    

}

impl Message {
    //关联函数
    fn some_func(){
        println!("welcome to rust");
    }
    
}


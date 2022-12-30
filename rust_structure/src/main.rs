//rust structure 使用结构体组织相关联的数据

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// fn main() {
//     println!("Hello, world!");
//     let mut u1: User = User {
//         username: String::from("LiShendu"),
//         email: String::from("1185345172@qq.com"),
//         sign_in_count: 1,
//         active: true,
//     };
//     let name = u1.username;
//     u1.username = String::from("kongkong");
//     println!("{name}");
//     println!("{}", u1.username);

//     let u2 = build_user(
//         String::from("18215005659@163.com"),
//         String::from("kongkong"),
//     );

//     let u3 = User {
//         username: String::from("xiongxiong"),
//         email: String::from("1185345172@qq.com"),
//         ..u2     //使u3直接复制u2中剩下的两个成员
//     };
// }
//使用函数初始化一个结构体
// fn build_user(email: String, username: String) -> User {
//     User {
//         username,
//         email,
//         sign_in_count: 1,
//         active: true,
//     }
// }

//元组结构体

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);
// fn main() {
//     let color1 = Color(123, 234, 456);
//     let point1 = Point(1, 2, 3);
//     let tup: (i32, u32) = (12, 12);
//     // let (x, y) = tup;
//     // println!("{x}{y}");
//     println!("first ={};second = {}", tup.0, tup.1);
// }

use colored::*;
//use std::{io, cmp::Ordering};

struct Flower {
    age:String,
    name:String
}

impl Flower {
    fn build_flower (a:String,b:String) ->Flower{
        Flower{
            age:a,
            name:b
        }
    }
}



fn main() {
    
    //let mut num1 = String::new();
    //io::stdin().read_line(&mut num1).expect("Filed!");
    //println!("{}",num1.yellow());
    
    // let num2 = String::from("10");
    // match num1.cmp(&num2) {
    //     Ordering::Less => println!("{}",1),
    //     Ordering::Greater => println!("{}",2),
    //     Ordering::Equal => println!("{}",3)        
    // }

    let f1 = Flower::build_flower(String::from("10"), String::from("makabaka"));
    let f2 = Flower::build_flower(String::from("8"), String::from("wuxidixi"));
    
    println!("The flower1's name is {},The flower's age is {}.",f1.name.red(),f1.age.green());
    println!("The flower2's name is {},The flower's age is {}.",f2.name.red(),f2.age.green());
    
}

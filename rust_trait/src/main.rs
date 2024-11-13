//trait:定义共享的行为
//trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共享的行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。

//注意：trait 类似于其他语言中常被称为 接口（interfaces）的功能，虽然有一些不同。

use core::panic;

//定义trait
//一个类型的行为由其可供调用的方法构成。如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。
//trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsAtricle {
    pub headline : String,
    pub location : String,
    pub author : String,
    pub content : String,
}
impl Summary for NewsAtricle {
    fn summarize(&self) -> String {
        format!("{},by {} ({})",self.headline,self.author,self.location)
    }
}

pub struct Tweet {
    pub username : String,
    pub content : String,
    pub reply : bool,
    pub retweet : bool,
}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{} : {}",self.username,self.content)
    }
}


impl Summary for Vec<i32> {
    fn summarize(&self) -> String {
        let num = self.get(0);
        let str = match num {
            Some(num) => num,
            None => panic!("none"),
        };
        str.to_string()
    }
}

fn main() {
    //定义trait
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    let vec = vec![1,2,3,4,5,6];
    let str1 = vec.summarize();
    println!("{:?}",str1);
}

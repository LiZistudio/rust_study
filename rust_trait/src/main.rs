//trait:定义共享的行为
//trait 告诉 Rust 编译器某个特定类型拥有可能与其他类型共享的功能。可以通过 trait 以一种抽象的方式定义共享的行为。可以使用 trait bounds 指定泛型是任何拥有特定行为的类型。

//注意：trait 类似于其他语言中常被称为 接口（interfaces）的功能，虽然有一些不同。

use core::panic;
use std::fmt::{Debug, Display};

//定义trait
//一个类型的行为由其可供调用的方法构成。如果可以对不同类型调用相同的方法的话，这些类型就可以共享相同的行为了。
//trait 定义是一种将方法签名组合起来的方法，目的是定义一个实现某些目的所必需的行为的集合。
pub trait Summary {
    //fn summarize(&self) -> String;
    //trait的默认实现
    fn summarize (&self) -> String {
        String::from("(read more...)")
    }
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

//默认实现
impl Summary for Vec<&str> {}
// impl Summary for Vec<&str> {
//     fn summarize (&self) -> String {
//         let item1 = self.get(2);
//         let str1 = match item1 {
//             Some(str) => str,
//             None => panic!("Error"),
//         };
//         str1.to_string()
//     }
// }

//trait作为参数
// pub fn notify (item: impl Summary) {     //trait bound 的语法糖
//     println!("Breaking news {}",item.summarize());
// }

//trait buond 语法
//impl Trait 语法适用于直观的例子，它实际上是一种较长形式语法的语法糖。我们称为 trait bound，它看起来像：
pub fn notify<T:Summary> (item:T) {
    println!("Breaking news {}",item.summarize());
}

pub fn function1 (_item1:impl Summary, _item2:impl Summary) {}  //这允许_item1和_item2的具体类型不一致，只要都实现了Summary这个trait.
pub fn function2<T:Summary> (_item1:T,_item2:T) {}  //这样就能强制使_item1和_item2的具体类型是一致的.

//通过"+"指定多个trait bound
//如果 notify 需要显示 item 的格式化形式，同时也要使用 summarize 方法，那么 item 就需要同时实现两个不同的 trait：Display 和 Summary。这可以通过 + 语法实现：
pub fn _notify<T:Summary+Display> (item:T) {
    println!("Breaking news {}",item.summarize());
}
//or
//pub fn _notify(item:impl Summary+Display) {}

//通过where简化trait bound 
//fn some_function (item1:impl Display+Debug,item2:impl Clone+Debug) ->i32 {0}  //impl trait写法
//fn some_function <T:Clone+Debug,U:Display+Clone> (item1:T,item2:U)->i32 {0}
 fn _some_function<T,U> (_t:T,_u:U) ->i32 
 where T:Display+Debug,U:Clone+Debug {0}    //where引导的从句,trait bound语法糖

 //返回实现了trait的类型
fn _function3 () -> impl Summary {
    let tweet = Tweet {
        username:String::from("value"),
        content:String::from("value"),
        reply:true,
        retweet:false,
    };
    tweet
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

    let str1 = vec!["《卜算子》","《清平乐》","《念奴娇》","《菩萨蛮》","《如梦令》"];
    let str = str1.summarize();
    println!("{}",str);

    //trait作为参数
    notify(tweet);
    notify(vec);
    notify(str1);

}

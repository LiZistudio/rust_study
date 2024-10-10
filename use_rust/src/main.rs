pub use std::collections::HashMap;
//use std::{fmt,io};
use rand::Rng;
use std::io::{self,Write};
use rand::*;

fn main() {
    eat_at_restaurant("扣三丝", "李华", 0001);
    let mut map:HashMap<i32,i32> = HashMap::new();
    map.insert(1,2);
    println!("map:{:?}",map);

    //fn function1 () -> fmt::Result {}
    //fn function2 () -> io::Result<()> {}

    let result = my_function(3,4);
    println!("map2:{:?}",result);

    //引入外部包（如rand）
    let my_rand = rand::thread_rng().gen_range(0..100);
    println!("my_rand is {}",my_rand);

}


mod front_of_house {
    pub mod hosting {
        pub struct Custom {
            name:String,
            num:u8,
        }
        impl Custom {
            pub fn add_to_waitlist (name:&str,num:u8) -> Custom {
                let custom = Custom {
                    name:String::from(name),
                    num,
                };
                println!("已创建顾客{:?}的信息并且添加到了队列当中,序号为{:?}。",custom.name,custom.num);
                return custom;
            }
        }
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant (food_name:&str,name:&str,num:u8) {
    // //绝对路径
    // crate::front_of_house::hosting::add_to_waitlist();
    // //相对路径
    // front_of_house::hosting::add_to_waitlist();

    hosting::Custom::add_to_waitlist(name,num);

    println!("您的客户点的菜是{:?}。",food_name);
}


//使用 as 关键字提供新的名称
pub use std::collections::HashMap as MyHashMap;

fn my_function (a:i32,b:i32) -> MyHashMap<i32,i32> {
    let mut map2:MyHashMap<i32,i32> = MyHashMap::new();
    map2.insert(a, b);
    map2
}
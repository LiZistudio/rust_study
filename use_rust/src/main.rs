use std::collections::HashMap;
//use std::{fmt,io};

fn main() {
    eat_at_restaurant("扣三丝", "李华", 0001);
    let mut map:HashMap<i32,i32> = HashMap::new();
    map.insert(1,2);
    println!("map:{:?}",map);

    //fn function1 () -> fmt::Result {}
    //fn function2 () -> io::Result<()> {}

    let result = my_function(3,4);
    println!("map2:{:?}",result);

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

use crate::front_of_house::hosting;

pub fn eat_at_restaurant (order_name:&str,name:&str,num:u8) {
    // //绝对路径
    // crate::front_of_house::hosting::add_to_waitlist();
    // //相对路径
    // front_of_house::hosting::add_to_waitlist();

    hosting::Custom::add_to_waitlist(name,num);

    println!("您的客户点的菜是{:?}。",order_name);
}


//使用 as 关键字提供新的名称
use std::collections::HashMap as MyHashMap;

fn my_function (a:i32,b:i32) -> MyHashMap<i32,i32> {
    let mut map2:MyHashMap<i32,i32> = MyHashMap::new();
    map2.insert(a, b);
    map2
}
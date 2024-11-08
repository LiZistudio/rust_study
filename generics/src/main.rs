//use std::collections::HashMap;
//use std::fs::File;

use core::panic;
use std::fs::File;

//如下面两个函数分别用来找出最大的i32和char
fn largest_i32(list:&Vec<i32>) -> i32 {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

fn largest_char(list:&Vec<char>) -> char {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max
}
//使用泛型
// fn largest_item<T:std::cmp::PartialOrd>(slice:&[T]) -> T {
//     let mut max = &slice[0];
//     for item in slice.iter() {
//         if item > max {
//             max = item;
//         }
//     }
//     max
// }

//结构体定义中的泛型
//同样也可以用 <> 语法来定义结构体，它包含一个或多个泛型参数类型字段。

#[derive(Debug)]
struct Point<T> {
    x:T,
    y:T
}
//方法定义中的泛型
//注意必须在 impl 后面声明 T，这样就可以在 Point<T> 上实现的方法中使用它了。
//在 impl 之后声明泛型 T ，这样 Rust 就知道 Point 的尖括号中的类型是泛型而不是具体类型。
impl<T> Point<T> {
    fn new(x:T,y:T) -> Point<T> {
        let point = Point {
            x,
            y
        };
        point
    }
}
//构建一个只用于拥有泛型参数 T 的结构体的具体类型的 impl 块
impl Point<f32> {
    fn get_x (&self) -> &f32 {
        &self.x
    }

    //计算该点到原点的距离
    fn get_distance (&self) -> f32 {
        (&self.x.powi(2)+&self.y.powi(2)).sqrt()
    }
}

//使用多个泛型类型参数
struct Color<S,T> {
    str:S,
    num1:T,
    num2:T,
    num3:T
}

//枚举定义中的泛型
//首先复习一下Option泛型枚举类型
pub fn option_test() {
    let vec1 = vec![1,2,3,4,5,6];
    let vec2 = vec1.get(2);
    let vec3 = match vec2 {
        Some(value) => value,
        None => panic!("值不存在"),
    };
    println!("Option value is {}",vec3);
}
//Result泛型枚举类型拥有多个泛型形参
pub fn result_test() {
    File::create("水龙吟.txt").expect("创建文件失败");
    let file1 = File::open("水龙吟.txt");
    let file = match file1 {
        Ok(file) => file,
        Err(error) => panic!("文件打开失败{}",error),
    };
    println!("{:?}",file);
}

fn main() {
    println!("泛型数据类型");
    //在函数定义中使用泛型
    let number_list = vec![1,23,4,5,6,23,6,7,34,433];
    println!("{:?}",largest_i32(&number_list));
    let char_list = vec!['a','f','e','h','z'];
    println!("{:?}",largest_char(&char_list));

    let point_a:Point<f32> = Point::new(3.33, 4.44);
    println!("{:?}",point_a);
    println!("{:?}",point_a.get_x());
    println!("{:?}",point_a.get_distance());

    let point_b = Point::new(1, 2);
    println!("{:?}",point_b);

    let point_c = Point {
        x:"a",
        y:"b"
    };
    println!("{:?}",point_c);

    let point_d = Point::new("江水何年初见月","江月何年初照人");
    println!("{:?}",point_d);

    // let mut hash_a = HashMap::new();
    // hash_a.insert("number",2);
    let str = vec!["银瓶乍破水浆喷","铁骑突出刀枪鸣"];
    println!("{:?}",str.get(1));
    match str.get(0) {
        Some(str) => println!("{:?}",str),
        None => panic!()
    }

    // let f1 = File::open("春江花月夜.txt");
    // let f1 = match f1 {
    //     Ok(file) => file,
    //     Err(err) => panic!("未能打开文件{:?}",err)
    // };

    //let f2 = File::open("蜀道难.txt").expect("打开错误");
    //let f3 = File::open("论持久战.txt")?;

    let _colo1 = Color {
        str:String::from("Blue"),
        num1:0,
        num2:0,
        num3:255
    };

    //泛型枚举类型Option复习
    option_test();
    //泛型枚举类型Result复习
    result_test();

}

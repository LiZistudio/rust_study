/*掌握闭包和迭代器是编写符合语言风格、高性能 Rust 代码重要的一环*/


//闭包：可以捕获环境的匿名函数
//Rust 的 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数。

use std::{collections::HashMap, thread, time::Duration};

use rand::Rng;

fn main() {
    let simulated_user_specified_value = 30;
    let simulated_random_number = rand::thread_rng().gen_range(1..10);

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );

    //示例 13-8：尝试调用一个被推断为两个不同类型的闭包
    let example_closure = |x| x;
    let result = example_closure(String::from("杨柳岸，晓风残月。多情应笑我，早生华发。"));
    println!("examplr_closure's value is :{}",result);
    //example_closure(3);
    //第一次使用 String 值调用 example_closure 时，编译器推断 x 和此闭包返回值的类型为 String。
    //接着这些类型被锁定进闭包 example_closure 中，如果尝试对同一闭包使用不同类型则会得到类型错误。

}


// /使用闭包创建行为的抽象
//示例 13-1：一个用来代替假定计算的函数，它大约会执行两秒钟
fn _simulated_expensive_calculation(intensity:u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

//示例 13-3：程序的业务逻辑，它根据输入并调用 simulated_expensive_calculation 函数来打印出健身计划
// fn generate_workout(intensity:u32,random_number:u32) {
//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             simulated_expensive_calculation(intensity)
//         );
//         println!(
//             "Next, do {} situps!",
//             simulated_expensive_calculation(intensity)
//         );
//     } else if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 simulated_expensive_calculation(intensity)
//             );
//             println!("random:{}",random_number);
//         }
// }


//使用函数重构
//示例 13-4：将 simulated_expensive_calculation 调用提取到一个位置，并将结果储存在变量 expensive_result 中
// fn generate_workout(intensity:u32,random_number:u32) {
//     let expensive_result = simulated_expensive_calculation(intensity);
//     if intensity < 25 {
//         println!(
//             "Today, do {} pushups!",
//             expensive_result
//         );
//         println!(
//             "Next, do {} situps!",
//             expensive_result
//         );
//     } else if random_number == 3 {
//             println!("Take a break today! Remember to stay hydrated!");
//         } else {
//             println!(
//                 "Today, run for {} minutes!",
//                 expensive_result
//             );
//             println!("random:{}",random_number);
//         }
// }


//重构使用闭包储存代码
fn generate_workout(intensity:u32,random_number:u32) {

    // //示例 13-5：定义一个闭包并储存到变量 expensive_closure 中
    // let expensive_closures = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    //示例 13-11：在 generate_workout 函数中利用 Cacher 结构体来抽象出缓存逻辑
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });


    //示例 13-7：为闭包的参数和返回值增加可选的类型标注
    // let expensive_closures = |num:u32| -> u32 {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_result.value(intensity)

        );
    } else if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)

            );
            println!("random:{}",random_number);
        }
}
//示例 13-6：调用定义的 expensive_closure

//************使用带有泛型和 Fn trait 的闭包**********************
//示例 13-9：定义一个 Cacher 结构体来在 calculation 中存放闭包并在 value 中存放 Option 值
struct Cacher<T> 
    where T:Fn(u32) -> u32 
{
    calculation:T,
    value:HashMap<u32,u32>,
}

impl<T> Cacher<T> 
    where T:Fn(u32) -> u32
{
    fn new(calculation:T) -> Cacher<T> {
        Cacher {
            calculation,
            value:HashMap::new(),
        }
    }

    fn value(&mut self,arg:u32) -> u32 {
            match self.value.get(&arg) {
                Some(&v) => v,
                None => {
                    let v = (self.calculation) (arg);
                    self.value.entry(arg).or_insert(v);
                    v
                },
            }
        }
}
//示例 13-10：Cacher 的缓存逻辑


//Cacher实现的限制


#[cfg(test)]
mod tests {
    use crate::Cacher;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);
    
        let _v1 = c.value(1);
        let v2 = c.value(2);
    
        assert_eq!(v2, 2);
    }
}

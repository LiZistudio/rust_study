/*掌握闭包和迭代器是编写符合语言风格、高性能 Rust 代码重要的一环*/


//闭包：可以捕获环境的匿名函数
//Rust 的 闭包（closures）是可以保存进变量或作为参数传递给其他函数的匿名函数。

use std::{thread, time::Duration};

use rand::Rng;

fn main() {
    let simulated_user_specified_value = 30;
    let simulated_random_number = rand::thread_rng().gen_range(1..10);

    generate_workout(
        simulated_user_specified_value,
        simulated_random_number
    );
}


// /使用闭包创建行为的抽象
//示例 13-1：一个用来代替假定计算的函数，它大约会执行两秒钟
fn simulated_expensive_calculation(intensity:u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

//示例 13-3：程序的业务逻辑，它根据输入并调用 simulated_expensive_calculation 函数来打印出健身计划
fn generate_workout(intensity:u32,random_number:u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
            println!("random:{}",random_number);
        }
    }

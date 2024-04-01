use colored::*;
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("欢迎来到猜数字游戏!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    //println!("秘密数字是：{secret_number}");

    loop {
        println!("请输入一个数字：");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("读取用户输入出错！");
        println!("你输入的数字是：{guess}");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "太小！".red()),
            Ordering::Greater => println!("{}", "太大！".red()),
            Ordering::Equal => {
                println!("{}", "you are winner!".green());
                break;
            }
        }
    }
}

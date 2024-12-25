// /作为最简单例子，Rust 中的测试就是一个带有 test 属性标注的函数。

use core::panic;
use std::fs::File;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}


//使用 assert! 宏来检查结果
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
//-----------------------snip----------------------------------



//使用assert_eq!和assert_ne!宏来测试相等
pub fn add_two (a:i32) -> i32 {
    a + 2
}
//-------------------------snip----------------------------------


//自定义失败信息
pub fn greeting(name:&str) -> String {
    format!("Hello {}!",name)
    //String::from("Hello")
}
//------------------------snip----------------------------------


//使用should_panic检查pianc
pub struct Guess {
    value:i32,
}
// impl Guess {
//     pub fn new (value:i32) -> Guess {
//         if value<1 || value>100 {
//             panic!("猜测数必须在1~100之间,现值为{}。",value);
//         }
//         Guess {
//             value
//         }
//     }
// }
//示例 11-9：一个会带有特定错误信息的 panic! 条件的测试
impl Guess {
    pub fn new(value:i32) -> Guess {
        if value < 1 {
            panic!("猜测值应该大于或等于1,但是当前的猜测值为{}",value);
        }else if value > 100 {
            panic!("猜测值应该小于或等于100,但是当前的猜测值为{}",value);
        }
        Guess {
            value
        }
    }
}
//----------------snip-------------------



//将Result<T,E>用于测试
fn create_file (a:&str) -> File{
    let file = File::create(a).expect("文件创建失败");
    file
    // let file = match file {
    //     Ok(file) => file,
    //     Err(error) => panic!("文件创建失败"),
    // };
}

#[cfg(test)]
mod tests {
    use core::time;
    use std::{io::Write, os::windows::fs::FileExt, thread::sleep};

    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        let result = add(3, 3);
        assert_eq!(result,6);
    }

    // #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(!smaller.can_hold(&larger));
    }


    #[test]
    fn it_adds_two() {
        let a = 10;
        assert_eq!(12,add_two(a));
    }

    #[test] 
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"),"函数greeting的结果没有包含名字,结果为{}",result);
        
    }

    //--------------should_panic----------------
    #[test]
    #[should_panic(expected = "猜测值应该大于或等于1")]
    fn smaller_than_1() {
        Guess::new(0);
    }
    #[test]
    #[should_panic(expected = "猜测值应该小于或等于100")]
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn create_file_success() {
        let one_millis = time::Duration::from_millis(1);
        let mut my_file = create_file("readme.md");
        for i in 1..11 {
            if i%2==0 {
                my_file.write(b"\n# To be or not to be , this is a question.")
                .expect("写入失败");
            }else {
                //my_file.write(b"\n## To be or not to be , this is a question.").expect("写入失败");
                my_file.seek_write(b" ",0)
                .expect("写入失败");
            }
        }
    }

    #[test]
    fn it_work() -> Result<(), String> {
        if 4 == 2+2 {
            Ok(())
        }else {
            Err(String::from("得到2+2不等于4"))
        }
    }
}

//---------------snip----------------由 cargo new 自动生成的测试模块和函数

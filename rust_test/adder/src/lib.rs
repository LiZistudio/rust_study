// /作为最简单例子，Rust 中的测试就是一个带有 test 属性标注的函数。

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
    //format!("Hello {}!",name)
    String::from("Hello")
}
//------------------------snip----------------------------------

//使用should_panic检查pianc


#[cfg(test)]
mod tests {
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
        assert!(result.contains("Carol"),"函数greeting的结果没有包含名字，结果为{}",result);
        
    }

}

//---------------snip----------------由 cargo new 自动生成的测试模块和函数


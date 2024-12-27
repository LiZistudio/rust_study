//*--------------------控制测试如何运行------------------------


//*******************************并行或连续的运行测试****************************/
pub fn hello_1() {
    println!("1");
}
pub fn hello_2() {
    println!("2");
}

//压强转换为单位面积承载的重量
pub fn p_to_kg(p:f32) -> f32 {
    p*10.204
}
//******************************************snip******************************/



//****************************显式函数输出*******************/
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);      //直接测试时不会有println!的输出，此输出被截获。
    10                                      //如果想要显示函数输出，可以使用“cargo test -- --show-output”
}
//转示例 11-10：一个调用了 println! 的函数的测试
//**************************snip********************/


//*通过指定名字来运行部分测试*/
pub fn add_two(a: i32) -> i32 {
    a + 2
}
//******snip**************/


//**************************忽略某些测试*****************************/
//可以添加#[ignore]属性来忽略某些测试函数


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work_1() {
        hello_1();
    }

    #[test]
    fn it_work_2() {
        hello_2();
    }

    #[test]
    fn p_to_kg_success() {
        let result = p_to_kg(0.45);
        assert_eq!(4.592,result,"得到的实际测试结果为{},与4.592kg不等。",result);
    }


//************示例 11-10：一个调用了 println! 的函数的测试***************
    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value);                      //会通过的测试
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value);                       //会失败的测试
    }
    //如果你希望也能看到通过的测试中打印的值，可以通过在末尾增加 --show-output 参数来告知 Rust 显示
//********************************snip******************************



//*通过指定名字来运行部分测试 */
//示例 11-11：不同名称的三个测试
    #[test]
    fn add_two_and_two() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn add_three_and_two() {
        assert_eq!(5, add_two(3));
    }

    #[test]
    fn one_hundred() {
        assert_eq!(102, add_two(100));
    }
/*运行单个测试--指定测试名称，如：cargo test one_hundred(测试函数名)*/
/*过滤运行多个测试--名字中共同的字段，或者测试模块名*/
//*示例 11-11：不同名称的三个测试*/

//***************忽略某些测试****************/
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        for i in 1..1000000 {
            println!("沉舟侧畔千帆过，病树前头万木春。({})",i);
        }
    }
//****************snip*********************/

}

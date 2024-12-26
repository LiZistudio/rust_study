//--------------------控制测试如何运行------------------------


/*******************************并行或连续的运行测试****************************/
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
/******************************************snip******************************/



/****************************显式函数输出*******************/
fn prints_and_returns_10(a: i32) -> i32 {
    println!("I got the value {}", a);      //直接测试时不会有println!的输出，此输出被截获。
    10
}
//转示例 11-10：一个调用了 println! 的函数的测试



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


    //示例 11-10：一个调用了 println! 的函数的测试
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


}

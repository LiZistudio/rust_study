/*函数*/
//函数名命名用snake case法，且定义在main函数前后均可

// fn another_function() {
//     println!("Another function.");
// }

fn another_function(x:i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: &str) {
    println!("The measurement is: {} {}", value, unit_label);
}

fn back_five() -> i32 {
    //return 5;
    5
}
fn plus_one(input:i32) -> i32 {
    input+1
}
/*语句和表达式*/
//Rust 是一门基于表达式（expression-based）的语言。
//语句（statement）是执行一些操作但不返回值的指令。
//表达式（expression）计算并产生一个值。

fn main() {
    println!("Hello, world!");

    //函数的调用就是表达式
    another_function(3);
    print_labeled_measurement(6,"min");

    //语句
    let _value = 666;
    //表达式
    let y = {
        let x = 3;
        x + 1           //注意！此处没有分号，如果写分号就变成了语句。
    };                  //返回值4绑定到y上
    println!("The value of y is: {}", y);

    let result:i32 = back_five();
    println!("The function feedback {result}");

    let result2:i32 = plus_one(10);
    println!("The plus_one's value is: {result2}");

}
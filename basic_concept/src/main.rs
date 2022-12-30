//通用编程概念

/*fn main() {
    let mut x = 5;
    println!("{x}");
    x = 10;
    println!("{x}");

    const SUBSCRIBER_COUNT: u32 = 100;
    println!("{SUBSCRIBER_COUNT}");

  let x: &str = "二十";
    println!("{x}");

    let x1 = 'd';
    let x2 = 'e';
    let x3 = '😊';
    println!("{x1},{x2},{x3}");

    //复合类型~元组tuple

    let tup: (f32, i32) = (32.2, 100);
    let (a1, a2) = tup;
    let a3: f32 = tup.0;

    println!("{a1},{a2},{a3}");

    //复合类型~数组
    let arr1: [i32; 5] = [100, 200, 300, 400, 500];

    let y1: i32 = arr1[1];

    //let _y2: i32 = arr1[5]; //越界访问
    let _y3: i32 = arr1[2];

    let arr2 = [1; 10];

    println!("y1 = {}, arr2[5] = {}", y1, arr2[5]);
    //Function
    println!("The sum is {}", my_function(3, 5));
}

//函数

fn my_function(x: i32, y: i32) -> i32 {
    println!("The value of x is {}", x);
    println!("The value of y is {}", y);
    x + y
}
*/

//控制流
fn main() {
    // //控制流if/else in let
    // let condition: bool = false;

    // let number = if condition { 1 } else { 0 };
    // println!("{}", number);

    //控制流 loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter >= 10 {
            break counter;
        }
    };
    println!("{}", counter);
    println!("{}", result);
}

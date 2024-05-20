fn main() {
    println!("Hello, world!");

    //变量声明
    let mut x = 5;
    println!("the value of x is :{x}");
    x = 6;
    println!("the value of x is :{x}");

    //常量声明
    //注意！rust常量声明不用let关键字，而是直接用const，并且必须注明常量类型
    const THREE_HOURS_IN_SECONDS:u32 = 3*60*60;
    println!("3个小时是 {THREE_HOURS_IN_SECONDS} 秒");

    let val = 1;
    let val = val + 1;
    {
        let val = val * 2;
        println!("the value of val is :{val}");
    }
    println!("the value of val is :{val}");

    let str1:String = String::from("    ");
    let str1 = str1.len();
    println!("{str1}");

    let _number1:i32 = "36".parse().expect("The input is not a number");

    let _number2 = 64.0;    //rust中浮点型赋值在不指定具体类型时，默认为双精度浮点型，即64位。
    let _number3:f32 = 32.0;

    let c = 'h';
    let c2 = '😊';
    println!("{c}ello {c2}");
    //**********rust中的数学运算操作符*****************
    // addition
    let sum = 5 + 10;
    println!("计算和:{sum}");
    // subtraction
    let difference = 95.5 - 4.3;
    println!("计算差:{difference}");
    // multiplication
    let product = 4 * 30;
    println!("计算乘:{product}");
    // division
    let quotient = 56.7 / 32.2;
    let floored = 2 / 3; // Results in 0
    println!("f64除:{quotient}");
    println!("i32除:{floored}");
    // remainder
    let remainder = 43 % 5;
    println!("取余:{remainder}");

    //**********bool*************
    let t = true;
    let f: bool = false; // with explicit type annotation
    println!("{t}");
    println!("{f}");

    //**********复合类型*************
    //Rust 有两种基本的复合类型：元组（tuple）和数组（array）
    let tup: (i32, f64, u8) = (500, 6.4, 1);    //将变量tup绑定到整个元组
    let (x, y, z) = tup;          //通过模式匹配来destructure这个tup元组，并将其值绑定到x,y,z三个变量上
    println!("{x}");
    println!("{y}");
    println!("{z}");

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("{five_hundred}");
    println!("{six_point_four}");
    println!("{one}");

    //数组类型
    //数组的每个元素必须具有相同的类型。Rust 中的数组具有固定长度。
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let first = a[0];
    let second = a[1];
    println!("{first}");
    println!("{second}");
    let _arr2= [1;5];

    let _months: [&str; 12] = ["January", "February", "March", "April", "May", "June", "July","August", "September", "October", "November", "December"];

    
}

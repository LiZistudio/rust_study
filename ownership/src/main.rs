//所有权规则*******************Rust的核心功能之一***********************重要指数：💖💖💖💖💖

//首先，让我们看一下所有权的规则。当我们通过举例说明时，请谨记这些规则：

//Rust 中的每一个值都有一个被称为其 所有者（owner）的变量。
//值在任一时刻有且只有一个所有者。
//当所有者（变量）离开作用域，这个值将被丢弃。

use std::io;

//变量作用域
fn main() {
    let important_counter = "💖 💖 💖 💖 💖:";
    println!("\n注:本章课程为Rust所有权功能(ownership),重要指数:{important_counter}");
    
    //这里有两个重要的时间点：
    //当 s 进入作用域 时，它就是有效的。
    //这一直持续到它 离开作用域 为止。
    {//混沌
    let mut _s = "敦煌"; //出生
    }//死亡

    //ownership以String类型为例
    println!("请输入:>");
    let mut str = String::new();
    io::stdin().read_line(&mut str).unwrap();
    println!("你输入的字符串为:{}",str);

    let mut str2:String = String::from("空不异色，色不异空");
    str2.push_str("。空即是色，色即是空。");
    println!("{}",str2);

    //字符串字面量和String中String可变而字面量却不行,区别在于两个类型对内存的处理上。

    //对于 String 类型，为了支持一个可变，可增长的文本片段，需要在堆上分配一块在编译时未知大小的内存来存放内容。这意味着：
    //必须在运行时向内存分配器请求内存。
    //需要一个当我们处理完 String 时将内存返回给分配器的方法。
    
    //一、第一部分由我们完成：当调用 String::from 时，它的实现（implementation）请求其所需的内存。这在编程语言中是非常通用的。
    //二、内存回收。方法一、GC;方法二、allocreate&free;而Rust采用了第三种方法，就是‘内存在拥有它的变量离开作用域后就被自动释放’。
    //如：
    {
        let my_str = String::from("采菊东篱下，悠然见南山");    //my_str开始有效
        println!("{}", my_str);     //使用
        //drop(my_str);
    }       //此作用域结束
            //my_str不再有效

    //这是一个将 String 需要的内存返回给分配器的很自然的位置：
    // 当 s 离开作用域的时候。当变量离开作用域，Rust 为我们调用一个特殊的函数。
    // 这个函数叫做 drop，在这里 String 的作者可以放置释放内存的代码。
    // Rust 在结尾的 } 处自动调用 drop。

    //变量与数据交互的方式（一）：移动
    let x = 7;
    let y = x;
    println!("{:?}",x);
    println!("{:?}",y);

    let s1 = String::from("乱花渐欲迷人眼，浅草才能没马蹄");
    let s2 = s1;
    //println!("{}",s1);
    println!("{}",s2);
    //如果你在其他语言中听说过术语 浅拷贝（shallow copy）和 深拷贝（deep copy），那么拷贝指针、长度和容量而不拷贝数据可能听起来像浅拷贝。
    //不过因为 Rust 同时使第一个变量无效了，这个操作被称为 移动（move），而不是浅拷贝。

    //另外，这里还隐含了一个设计选择：Rust 永远也不会自动创建数据的 “深拷贝”。因此，任何 自动 的复制可以被认为对运行时性能影响较小。

    //变量与数据交互的方式（二）：克隆
    //如果我们 确实 需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone 的通用函数。
    let s3 = String::from("最爱湖东行不足，绿杨荫里白沙堤");
    let s4 = s3.clone();
    println!("s3:{:?} AND s4:{:?}",s3,s4);

    //Copy trait:
        //任何一组简单标量值的组合都可以实现 Copy，任何不需要分配内存或某种形式资源的类型都可以实现 Copy 。如下是一些 Copy 的类型.

    //如下是一些 Copy 的类型：
    // 所有整数类型，比如 u32。
    // 布尔类型，bool，它的值是 true 和 false。
    // 所有浮点数类型，比如 f64。
    // 字符类型，char。
    // 元组，当且仅当其包含的类型也都实现 Copy 的时候。比如，(i32, i32) 实现了 Copy，但 (i32, String) 就没有。


    // *所有权与函数******************************
    //将值传递给函数在语义上与给变量赋值相似。向函数传递值可能会移动或者复制，就像赋值语句一样。

    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                  // ... 所以到这里不再有效
    //println!("{}", s);    XXX

    let x = 5;                      // x 进入作用域

     makes_copy(x);                  // x 应该移动函数里，
                                  // 但 i32 是 Copy 的，所以在后面可继续使用 x
    println!("{}",x);

    //************************返回值与作用域*****************************************************************/
    let _s1 = gives_ownership();         // gives_ownership 将返回值
                                                // 移给 s1
    let s2 = String::from("hello");     // s2 进入作用域                    
    
    let _s3 = takes_and_gives_back(s2);  // s2 被移动到
                                      // takes_and_gives_back 中,
                                      // 它也将返回值移给 s3

    //*************************转移返回值的所有权*************************************************************/
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

}// 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
  // 所以不会有特殊操作

  // 这里, s3 移出作用域并被丢弃。s2 也移出作用域，但已被移走，
  // 所以什么也不会发生。s1 移出作用域并被丢弃

//所有权与函数
fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放
  
fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作

//返回值与作用域
fn gives_ownership() -> String {           // gives_ownership 将返回值移动给
    // 调用它的函数

let some_string = String::from("yours"); // some_string 进入作用域

some_string                              // 返回 some_string 并移出给调用的函数
}
// takes_and_gives_back 将传入字符串并返回该值
fn takes_and_gives_back(a_string: String) -> String { // a_string 进入作用域

    a_string  // 返回 a_string 并移出给调用的函数
}

//转移返回值的所有权
fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() 返回字符串的长度

    (s, length)
}
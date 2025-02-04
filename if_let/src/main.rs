/*if_let简单控制流*/

fn main() {
    //匹配一个 Option<u8> 值并只希望当值为 3 时执行代码：
    let value:Option<u8> = Option::Some(3);
    match value {
        Some(i) => value_is_three(i),
        _ => (),
        
        None => println!("None"),    //无法达到的匹配！！！
    }
    /*我们想要对 Some(3) 匹配进行操作但是不想处理任何其他 Some<u8> 值或 None 值。
    为了满足 match 表达式（穷尽性）的要求，必须在处理完这唯一的成员后加上 _ => ()，
    这样也要增加很多样板代码。！！！*/

    //不过我们可以使用 if let 这种更短的方式编写。
    // if value == Some(3) {
    //     value_is_three(3);
    // }

    //换句话说，可以认为 if let 是 match 的一个语法糖，它当值匹配某一模式时执行代码而忽略所有其他值。
    if let Some(3) = value {
        value_is_three(3);
    }
}


//匹配一个 Option<u8> 值并只希望当值为 3 时执行代码：
fn value_is_three (i:u8) {
    println!("当前值为{}",i);
}


/*总结
现在我们涉及到了如何使用枚举来创建有一系列可列举值的自定义类型。我们也展示了标准库的 Option<T> 类型是如何帮助你利用类型系统来避免出错的。
当枚举值包含数据时，你可以根据需要处理多少情况来选择使用 match 或 if let 来获取并使用这些值。
你的 Rust 程序现在能够使用结构体和枚举在自己的作用域内表现其内容了。在你的 API 中使用自定义类型保证了类型安全：编译器会确保你的函数只会得到它期望的类型的值。
为了向你的用户提供一个组织良好的 API，它使用起来很直观并且只向用户暴露他们确实需要的部分，那么现在就让我们转向 Rust 的模块系统吧。*/
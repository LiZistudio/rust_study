fn main() {
    println!("vector 用来储存一系列的值");
    //我们要讲到的第一个类型是 Vec<T>，也被称为 vector。vector 允许我们在一个单独的数据结构中储存多个值，所有值在内存中彼此相邻排列。
    //vector 只能储存相同类型的值。它们在拥有一系列项的场景下非常实用，例如文件中的文本行或购物车中商品的价格。

    //新建一个空的vector
    let mut v1:Vec<f32> = Vec::new();
    v1.push(3.33);
    
    //更新vector
    v1.push(9.99);          //通过push()方法更新vector
    println!("{:?}",v1);

    //在更实际的代码中，一旦插入值 Rust 就可以推断出想要存放的类型，所以你很少会需要这些类型标注。
    //更常见的做法是使用初始值来创建一个 Vec，而且为了方便 Rust 提供了 vec! 宏。这个宏会根据我们提供的值来创建一个新的 Vec。
    //示例 8-2 新建一个拥有值 1、2 和 3 的 Vec<i32>：
    /*宏 vec!*/
    let v2 = vec![3.33,5.55,7.77];
    println!("{:?}",v2);

    //丢弃 vector 时也会丢弃其所有元素
    //类似于任何其他的 struct，vector 在其离开作用域时会被释放
    {
        {
            let v3 = vec![1,2,3,4,5];
            println!("有效的vector v3 :{:?}",v3);
            //drop(v3);
            //处理v3
        } //<-离开时v3被丢弃
        println!("离开作用域vector v3 被丢弃");
    }

    //读取vector的元素
    let v4 = vec![3,1,4,1,5,9,2,6];
    let v4_third = &v4[3];
    println!("The third element of v4 is {}",v4_third);
    //v4[3] = 6;

    match v4.get(3) {
        Some(v4_third) => println!("The third element of v4 is {}",v4_third),
        None => println!("v4 don't have third element."),
    }

    //vector超出索引边界
    let v5 = vec![1,2,3,4,5];
    //let v5_01 = &v5[100];
    let mut v5_02 = v5.get(100);
    //println!("{:?}",v5_02);     //None
    let zero = 0;
    match v5_02 {
        Some(result) => println!("v5_02 的值为：{}",result),
        None => v5_02 = Option::Some(&zero),
    }
    println!("{:?}",v5_02);

    //遍历vector中的元素
    let v6 = vec![1,2,3,4,5,6,7];
    //遍历不可变引用
    for i in &v6 {
        print!("{}  ",i);
    }
    //遍历可变引用
    println!();
    let mut v7 = vec![1,2,3,4,5,6,7,8,9];
    for i in &mut v7 {
        *i+=50;
        print!("{}  ",i);
    }

    //使用枚举来存储多种类型
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(1),
        SpreadsheetCell::Float(3.33),
        SpreadsheetCell::Text(String::from("book")),
    ];
}

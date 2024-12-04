//Rust需要我们使用泛型生命周期参数来注明他们的关系，这样就能确保运行时实际使用的引用绝对是有效的。
//生命周期的概念从某种程度上说不同于其他语言中类似的工具，毫无疑问这是 Rust 最与众不同的功能。

//生命周期避免了悬垂引用
//生命周期的主要目标是避免悬垂引用，它会导致程序引用了非预期引用的数据。
//--snip--示例10-17尝试使用离开作用域的值的引用

//-----------------snip---------------------借用检查器
//这里将 r 的生命周期标记为 'a 并将 x 的生命周期标记为 'b。如你所见，内部的 'b 块要比外部的生命周期 'a 小得多。
//在编译时，Rust 比较这两个生命周期的大小，并发现 r 拥有生命周期 'a，不过它引用了一个拥有生命周期 'b 的对象。
//程序被拒绝编译，因为生命周期 'b 比生命周期 'a 要小：被引用的对象比它的引用者存在的时间更短。



//-----------------snip---------------------函数中的泛型生命周期
// fn longest (string1:&str,string2:&str) ->&str {
//     if string1.len() > string2.len() {
//         string1
//     }else {
//         string2
//     }
// }        //示例 10-21：一个 longest 函数的实现，它返回两个字符串 slice 中较长者，现在还不能编译

use std::fmt::Display;

fn longest (str1:&str, str2:&str) -> String {
    if str1.len() > str2.len() {
        str1.to_string()
    }else if str1.len() < str2.len() {
        str2.to_string()
    }else {
        println!("str1_len == str2_len");
        String::from("==")
    }
}
//-----snip-----因为 Rust 并不知道将要返回的引用是指向 x 或 y。事实上我们也不知道，因为函数体中 if 块返回一个 x 的引用而 else 块返回一个 y 的引用！


//-----------------snip-----------------生命周期标注语法
// 这里有一些例子：我们有一个没有生命周期参数的 i32 的引用，一个有叫做 'a 的生命周期参数的 i32 的引用，和一个生命周期也是 'a 的 i32 的可变引用：
// &i32      引用
// &'a i32       带有显式生命周期的引用
// &'a mut i32       带有显式生命周期的可变引用
/* 单个生命周期标注本身没有多少意义，因为生命周期标注告诉 Rust 多个引用的泛型生命周期参数如何相互联系的。
 例如如果函数有一个生命周期 'a 的 i32 的引用的参数 first。还有另一个同样是生命周期 'a 的 i32 的引用的参数 second。这两个生命周期标注意味着引用 first 和 second 必须与这泛型生命周期存在得一样久。*/

//-------------snip------------------函数签名中的生命周期标注
fn lonest<'a> (str1:&'a str,str2:&'a str) -> &'a str {
    if str1.len() > str2.len() { str1 }else { str2 }
}
//现在函数签名表明对于某些生命周期 'a，函数会获取两个参数，他们都是与生命周期 'a 存在的一样长的字符串 slice。
//函数会返回一个同样也与生命周期 'a 存在的一样长的字符串 slice。它的实际含义是 lonest 函数返回的引用的生命周期与传入该函数的引用的生命周期的较小者一致。
//示例 10-22：longest 函数定义指定了签名中所有的引用必须有相同的生命周期 'a---------------snip----------------


//-------------------snip--------------深入理解生命周期
/*指定生命周期参数的正确方式依赖函数实现的具体功能。*/
//例如，如果将 longest 函数的实现修改为总是返回第一个参数而不是最长的字符串 slice，就不需要为参数 y 指定一个生命周期。如下代码将能够编译:
fn longest2<'a> (x: &'a str,y: &str) -> &'a str {
    x
}
//综上，生命周期语法是用于将函数的多个参数与其返回值的生命周期进行关联的。
//一旦他们形成了某种关联，Rust 就有了足够的信息来允许内存安全的操作并阻止会产生悬垂指针亦或是违反内存安全的行为。


//--------------snip---------结构体定义中的生命周期标注
//示例 10-25 中有一个存放了一个字符串 slice 的结构体 ImportantExcerpt：
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

//生命周期省略（Lifetime Elision）
//示例 10-26：示例 4-9 定义了一个没有使用生命周期标注的函数，即便其参数和返回值都是引用
fn first_word (s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
//函数或方法的参数的生命周期被称为 输入生命周期（input lifetimes），而返回值的生命周期被称为 输出生命周期（output lifetimes）。


//方法定义中的生命周期标注
impl<'a> ImportantExcerpt<'a> {
    fn level (&self) -> i32 {
        0
    }

    fn announce_and_return_part (&self,announce:&str) -> &str { //第三条生命周期省略规则
        println!("Attention please:{:?}",announce);
        self.part
    }
}

//静态生命周期
//这里有一种特殊的生命周期值得讨论：'static，其生命周期能够存活于整个程序期间。所有的字符串字面量都拥有 'static 生命周期，我们也可以选择像下面这样标注出来：


//结合泛型类型参数、trait bounds和生命周期
fn longest_with_an_announcement<'a,T> (x:&'a str,y:&'a str,ann:T) -> &'a str
where T:Display
{
    println!("{}",ann);
    if x > y {
        x
    }else {
        y
    }
}


fn main() {
    println!("生命周期与引用有效性");

    
    //---------------snip--------------示例10-17尝试使用离开作用域的值的引用
    {
        let r;
    
        {
            let x = 5;
            //r = &x;     //x在将值借用给r之后就死亡了，所以该引用是个悬垂引用。
            r = x;
        }
    
        println!("r: {}", r);
    }
    //-------------------------snip----------------------------------------


    //------------------------snip---------------------------借用检查器(borrow checker)
    {
        let r;              //-------------------------+-- 'a
        {                        //                         |
            let x = 5;      //--------+-- 'b           |
            //r = &x;            //        |                |
            r = x;               //        |                |
        }                        //--------+                |
        println!("r:{}",r);      //                         |
    }                            //-------------------------+
    //--------------------------snip--------------------------示例 10-18：r 和 x 的生命周期标注，分别叫做 'a 和 'b


    //------------------------------snip------------------------没有悬垂引用
    {
        let x = 5;              //------------'b
        let r = &x;            //------------'a
        println!("r:{}",r);
    }
    //-----------------------snip--------------------------示例 10-19：一个有效地引用，因为数据比引用有着更长的生命周期


    //----------------snip--------------------示例 10-20：main 函数调用 longest 函数来寻找两个字符串 slice 中较长的一个
    let string1 = "hello";
    let string2 = "Never ever ever give up.";
    let result = longest(string1, string2);
    println!("The longest is :{:?}",result);
    println!("{:?}",lonest("aaa","abcdefg"));
    //------------------------------------------snip---------------------------------------------------

    //通过传递拥有不同具体生命周期的引用来限制lonest函数的使用
    let s1 = String::from("大江东去，浪淘尽，千古风流人物。");
    {
        let s2 = String::from("千古风流人物，还看今朝。");
        let result = lonest(s1.as_str(),s2.as_str());
        println!("最长的一句话是:{:?}",result);
    }
    //------snip-------上面这段是可以正常编译的
    /*let s3 = String::from("秦皇汉武，略输文采。唐宗宋祖，稍逊风骚。");
    let result;
    {
        let s4 = String::from("一代天骄，成吉思汗，只识弯弓射大雕。");
        result = lonest(s3.as_str(),s4.as_str());       //borrowed value does not live long enough
    }
    println!("{:?}",result);*/
    //----------------snip-------------上面这段是无法编译的
    

    //-----------snip------------示例 10-25：一个存放引用的结构体，所以其定义需要生命周期标注
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.')
        .next()
        .expect("Could not find a '.'");
    let i = ImportantExcerpt{ part:first_sentence, };
    println!("{:?}",i);
    //这里的 main 函数创建了一个 ImportantExcerpt 的实例，它存放了变量 novel 所拥有的 String 的第一个句子的引用。
    // novel 的数据在 ImportantExcerpt 实例创建之前就存在。
    // 另外，直到 ImportantExcerpt 离开作用域之后 novel 都不会离开作用域，所以 ImportantExcerpt 实例中的引用是有效的。


    //静态生命周期
    //这里有一种特殊的生命周期值得讨论：'static，其生命周期能够存活于整个程序期间。所有的字符串字面量都拥有 'static 生命周期，我们也可以选择像下面这样标注出来：
    let my_string:&'static str = "静态生命周期";
    //这个字符串的文本被直接储存在程序的二进制文件中而这个文件总是可用的。因此所有的字符串字面量都是 'static 的。


}


/*现在你知道了泛型类型参数、trait 和 trait bounds 以及泛型生命周期类型，你已经准备好编写既不重复又能适用于多种场景的代码了。
泛型类型参数意味着代码可以适用于不同的类型。trait 和 trait bounds 保证了即使类型是泛型的，这些类型也会拥有所需要的行为。
由生命周期标注所指定的引用生命周期之间的关系保证了这些灵活多变的代码不会出现悬垂引用。*/


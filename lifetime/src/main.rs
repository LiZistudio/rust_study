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

fn longest (str1:&str,str2:&str) -> String {
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
    //------------------------------------------snip---------------------------------------------------

    //--------------snip-------------生命周期标注语法


}

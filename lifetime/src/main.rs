//Rust需要我们使用泛型生命周期参数来注明他们的关系，这样就能确保运行时实际使用的引用绝对是有效的。
//生命周期的概念从某种程度上说不同于其他语言中类似的工具，毫无疑问这是 Rust 最与众不同的功能。

//生命周期避免了悬垂引用
//生命周期的主要目标是避免悬垂引用，它会导致程序引用了非预期引用的数据。
//--snip--示例10-17尝试使用离开作用域的值的引用

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
    
}
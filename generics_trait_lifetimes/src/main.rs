fn main() {
    println!("Generics & Trait & LifeTimes;泛型、Trait和生命周期");
    //泛型是具体类型或其他属性的抽象替代。

    //trait是一个定义泛型行为的方法。trait可以与泛型结合来将泛型限制为拥有特定行为的类型，而不是任意类型。

    //生命周期（lifetimes），是一类允许我们向编译器提供引用如何相互关联的泛型。Rust 的生命周期功能允许在很多场景下借用值的同时仍然使编译器能够检查这些引用的有效性。

    /**********提取函数来减少重复**********/
    let number_list = vec![15,68,87,256,75,562,47,418,24,287,487,145,587,635];
    let max_number = get_max_number(&number_list);
    println!("{:?}",max_number);
}

//查找一组数中的最大值
fn get_max_number(number_list:&Vec<i32>) -> i32 {
    let mut max_number = number_list[0];
    for &number in number_list {
        if number > max_number {
            max_number = number
        }
    }
    max_number
}

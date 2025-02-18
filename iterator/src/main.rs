/*使用迭代器处理元素序列*/

fn main() {

    let v1 = vec![1,2,3];
    let mut v1_iter = v1.iter();
    //示例 13-13：创建一个迭代器
    // for i in v1_iter {
    //     println!("{i}");
    // }
    //示例 13-14：在一个 for 循环中使用迭代器

    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    //示例 13-15：在迭代器上（直接）调用 next 方法


}

//Iterator trait 和 next 方法
//迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait。这个 trait 的定义看起来像这样：
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;

    //此处省去方法默认实现
}


//消费迭代器的方法
//这些调用next方法的方法被称为消费适配器（consuming adaptors），因为调用他们会消费迭代器
//如sum方法:
#[test]
fn iterator_sum() {
    let vec1 = vec![1,2,3,4,5,6,7,8,9];
    let iterator_v1 = vec1.iter();
    let total:i32 = iterator_v1.sum();

    assert_eq!(total,45);
}
//示例 13-16：调用 sum 方法获取迭代器所有项的总和
//调用 sum 之后不再允许使用 v1_iter 因为调用 sum 时它会获取迭代器的所有权。

//产生其他迭代器的方法
//迭代适配器
#[test]
fn func() {
    let v2 = vec![1,2,3];
    let v3:Vec<_> = v2.iter().map(|x|{x+1}).collect();
    let v4 = vec![2,3,4];
    assert_eq!(v3,v4);
}
//示例 13-17：调用迭代器适配器 map 来创建一个新迭代器
//示例 13-18：调用 map 方法创建一个新迭代器，接着调用 collect 方法消费新迭代器并创建一个 vector


//使用闭包获取环境

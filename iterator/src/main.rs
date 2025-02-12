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

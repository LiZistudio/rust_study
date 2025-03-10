/*使用迭代器处理元素序列*/

fn main() {
}

//Iterator trait 和 next 方法
//迭代器都实现了一个叫做 Iterator 的定义于标准库的 trait。这个 trait 的定义看起来像这样：
// pub trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;

//     //此处省去方法默认实现
// }


#[cfg(test)]
mod tests {
    use crate::{shoes_in_my_size, Shoe};


    #[test]
    fn iterator_next() {
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
    #[test]
    fn filters_by_size() {
        let shoes_vec1 = vec![
            Shoe {size:42,style:String::from("sneaker")},
            Shoe {size:40,style:String::from("sandal")},
            Shoe {size:42,style:String::from("boot")},
        ];

        let my_shoes = shoes_in_my_size(shoes_vec1, 42);
        let suit_shoes = vec![Shoe {size:42,style:String::from("sneaker")},
                                        Shoe{size:42,style:String::from("boot")}];

        assert_eq!(my_shoes,suit_shoes);
    }
}

#[derive(PartialEq,Debug)]
pub struct Shoe {
    pub size:i32,
    pub style:String,
}

//示例 13-19：使用 filter 方法和一个捕获 shoe_size 的闭包
pub fn shoes_in_my_size(shoes:Vec<Shoe>,shoe_size:i32) -> Vec<Shoe> {
    shoes.into_iter()
        .filter(|s|{s.size == shoe_size})
        .collect()
}

//实现 Iterator trait 来创建自定义迭代器
struct Counter {
    count:u32,
}

impl Counter {
    fn new() -> Counter {
        Counter {
            count:0,
        }
    }
}
//示例 13-20：定义 Counter 结构体和一个创建 count 初值为 0 的 Counter 实例的 new 函数

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        // if self.count < 6 {
        //     Some(self.count)
        // }else {
        //     None
        // }

        //let cmp_number = 6;
        // match self.count.cmp(&cmp_number) {
        //     std::cmp::Ordering::Less => Some(self.count),
        //     _ => None,
        // }
        // match self.count {
        //     0..=5 => Some(self.count),
        //     _ => None,
        // }
        match self.count {
            n if n < 6 => Some(n),
            _ => None,
        }
    }
}
//示例 13-21：在 Counter 结构体上实现 Iterator trait

#[test]
fn ont_to_six_iterator() {
    let mut counter = Counter::new();

    assert_eq!(counter.next(),Some(1));
    assert_eq!(counter.next(),Some(2));
    assert_eq!(counter.next(),Some(3));
    assert_eq!(counter.next(),Some(4));
    assert_eq!(counter.next(),Some(5));
    assert_eq!(counter.next(),None);   
}
//示例 13-22：测试 next 方法实现的功能

//使用自定义迭代器中其他 Iterator trait 方法
#[test]
fn using_other_iterator_trait_methods() {
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
//示例 13-23：使用自定义的 Counter 迭代器的多种方法

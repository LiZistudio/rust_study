
fn main() {
    println!("泛型数据类型");
    //在函数定义中使用泛型
    let number_list = vec![1,23,4,5,6,23,6,7,34,433];
    println!("{:?}",largest_i32(&number_list));
    let char_list = vec!['a','f','e','h','z'];
    println!("{:?}",largest_char(&char_list));
}


//如下面两个函数分别用来找出最大的i32和char
fn largest_i32(list:&Vec<i32>) -> i32 {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

fn largest_char(list:&[char]) -> char {
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max
}
//使用泛型
// fn largest_item<T:std::cmp::PartialOrd>(slice:&[T]) -> T {
//     let mut max = &slice[0];
//     for item in slice.iter() {
//         if item > max {
//             max = item;
//         }
//     }
//     max
// }

//结构体定义中的泛型

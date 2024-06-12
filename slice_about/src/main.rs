fn main() {
    let poem1 = String::from("落霞与孤鹜齐飞，秋水共长天一色。");
    let poem2 = &poem1;
    let poem3 = &poem1[0..6];
    println!("{:?}中的{:?}",poem2, poem3);

    let _slice1 = &poem1[0..3];
    let _slice2 = &poem1[..3];
    //_slice1和_slice2是一样的，0可以省略。

    let len = poem1.len();
    let _slice3 = &poem1[3..len];
    let _slice4 = &poem1[3..];
    //slice3和slice4是一样的，要是引用到字符串结尾，可以省略区间尾部。

    //************************切片 Slice 类型************************
    //另一个没有所有权的数据类型是 slice。slice 允许你引用集合中一段连续的元素序列，而不用引用整个集合。

    //?小习题：编写一个函数，该函数接收一个字符串，并返回在该字符串中找到的第一个单词。如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。
    let str1 = String::from("Flower like sunshine");
    let str1 = "Flower like sunshine";
    let str1 = "垆边人似月 皓腕凝霜雪";
    fn first_word(s:&str) -> &str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' '{
                return &s[0..i];
            }
        }
        &s[..]
    }

    let res = first_word(&str1[..]);
    let res = first_word(str1);
    println!("{:?}",res);

    let arr:[i32;5] = [1,2,3,4,5];
    let slice_arr = &arr[1..3];
    println!("{:?}",slice_arr);
}

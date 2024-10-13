/*使用字符串储存UTF-8编码的文本*/

fn main() {
    println!("Hello String");
    let str1 = String::from("使用字符串储存UTF-8编码的文本");
    let str2 = "字符串切片";
    let str3 = &str1[0..6];
    println!("str1:{:?};str2:{:?};str3:{:?}.",str1,str2,str3);
}

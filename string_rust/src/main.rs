//使用字符串存储 UTF-8 编码的文本

//use std::fmt::format;

fn main() {
    println!("Hello String");
    let mut _str1 = String::new();
    _str1 = String::from("使用UTF-8");

    let deta = "UTF-8编码的字符串切片";
    let str2 = deta.to_string();

    let _str3 = "字符串".to_string();
    println!("{}",str2);

    let _str4 = String::from("value");

    let mut hello = vec![String::from("السلام عليكم"),
                                  String::from("Dobrý den"),
                                  String::from("Hello"),
                                  String::from("שָׁלוֹם"),
                                  String::from("नमस्ते"),
                                  String::from("こんにちは"),
                                  String::from("안녕하세요"),
                                  String::from("你好"),
                                  String::from("Olá"),
                                  String::from("Здравствуйте"),
                                  String::from("Hola"),
                                  String::from("ALOHA"),];
    for i in &hello {
        println!("{:?}",i);
    }

    //更新字符串
    //String 的大小可以增加，其内容也可以改变，就像可以放入更多数据来改变 Vec 的内容一样。另外，可以方便的使用 + 运算符或 format! 宏来拼接 String 值。
    hello.push(String::from("哈喽"));
    for i in &hello {
        println!("{:?}",i);
    }

    let _str5 = _str4+" is right";
    let _str6 = format!("value {}","is false");
    println!("{:?}",_str5);
    println!("{:?}",_str6);

    let mut _str7 = String::from("了了青山见，");
    _str7.push_str("纷纷宿雾空");
    println!("{:?}",_str7);

    let mut str9 = String::from("采菊东篱下，");
    let str10 = String::from("悠然见南山");
    str9.push_str(&str10);
    println!("{:?}",str10);
    println!("{:?}",str9);

    let mut s1 = String::from("醉里挑灯看剑，");
    let s2 = "梦回吹角连营。";
    s1.push_str(s2);
    println!("{:?}",s1);
    println!("{:?}",s2);

    let mut s3 = String::from("lo");
    s3.push('l');
    println!("{:?}",s3);

    let s4 = String::from("揭谛，揭谛，");
    let s5 = String::from("波若揭谛");
    let s6 = s4 + &s5;
    println!("{:?}",s6);

    let s7 = String::from("江南游子，");
    let s8 = String::from("把吴钩看了，栏杆拍遍。");
    let s9 = String::from("无人会，登临意。");
    let s10 = format!("{}{}{}",s7,s8,s9);
    println!("{:?}",s10);

    let mut ss = Vec::new();
    ss.push('h');
    ss.push('e');
    ss.push('l');
    ss.push('l');
    ss.push('o');
    println!("{:?}",ss);
    for i in &ss {
        print!("{}",i);
    }

    let ss1 = String::from("不生不灭，不垢不净，不增不减").len();
    println!("\n{}",ss1);
    let ss2 = String::from("yestoday is a gift").len();
    println!("{}",ss2);

    let ss3 = String::from("诸色妄妄");
    let ss4 = &ss3[0..6];
    println!("{:?}",ss4);

    for c in s8.chars() {
        print!("{}",c);
    }
    for b in s8.bytes() {
        print!("{}",b);
    }
}

// fn main() {
//     println!("💖💖💖💖💖引用与借用💖💖💖💖💖");

//     let s1 = String::from("hello");

//     let len = calculate_length(&s1);      //以一个对象的引用作为参数而不是获取值的所有权

//     println!("The length of '{}' is {}.", s1, len);

//     let s3 = String::from("老当益壮，宁移白首之心。");
//     let s4:&String = &s3;
//     let s5 = s3.clone();
//     println!("s4:{:?}", s4);
//     println!("s3:{:?}", s3);
//     println!("s5:{:?}", s5);

//     let s6 = "穷且益坚，不坠青云之志。";
//     let s7 = s6;
//     println!("s7:{:?}",s7);
//     println!("s6:{:?}",s6);

//     let _counter = print_length(&s3);
// }

// fn calculate_length(s: &String) -> usize {      //&String s 指向 String s1 // s 是对 String 的引用
//     s.len()
// }   // 这里，s 离开了作用域。但因为它并不拥有引用值的所有权，
//     // 所以什么也不会发生

// fn print_length(s:&String) -> i32 {
//     println!("The length of '{}' is {} byte.", s, s.len());
//     s.len() as i32
// }
// //注意：与使用 & 引用相反的操作是 解引用（dereferencing），它使用解引用运算符，*。

//**********************可变引用*****************/
//变量在默认情况下是不可变的
// fn main() {
//     let mut str1:String = String::from("浔阳江头夜送客，");
//     change(&mut str1);
//     println!("str1:{:?}",str1);

//     //不过可变引用有一个很大的限制：在同一时间，只能有一个对某一特定数据的可变引用。尝试创建两个可变引用的代码将会失败。
//     // let str2 = &mut str1;
//     // let str3 = &mut str1;
//     // println!("str2:{:?};str3:{:?}",str2,str3);

//     //这个限制的好处是 Rust 可以在编译时就避免数据竞争。数据竞争（data race）类似于竞态条件，它由这三个行为造成：
//     // 两个或更多指针同时访问同一数据。
//     // 至少有一个指针被用来写入数据。
//     // 没有同步数据访问的机制。
//     // 译注：以上三个行为同时发生才会造成数据竞争，而不是单一行为。
    
//     //下面是可行的
//     {
//         let _str2 = &mut str1;
//     }
//     let str3 = &mut str1;
//     println!("str3:{:?}",str3);

//     //类似的规则也存在于同时使用可变与不可变引用中。这些代码会导致一个错误：

//     //This code does not compile!
//     // let mut s = String::from("hello");

//     // let r1 = &s; // 没问题
//     // let r2 = &s; // 没问题
//     // let r3 = &mut s; // 大问题

//     // println!("{}, {}, and {}", r1, r2, r3);

//     //非词法作用域生命周期（Non-Lexical Lifetimes，简称 NLL）
//     let mut s = String::from("寻寻觅觅");

//     let r1 = &s; // 没问题
//     let r2 = &s; // 没问题
//     println!("{} and {}", r1, r2);
//     // 此位置之后 r1 和 r2 不再使用
//     //它们的作用域没有重叠
//     let r3 = &mut s; // 没问题
//     println!("{}", r3);


// }

// fn change (s: &mut String) {
//     s.push_str("枫叶荻花秋瑟瑟。");
// }


//*******************************悬垂引用（Dangling References）*******************************
//在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。

fn main () {
    let reference_to_nothing = dangle();
    println!("reference_to_nothing:{}", reference_to_nothing);
}

// fn dangle() -> &String {    // dangle 返回一个字符串的引用
//     let s = String::from("hello");
//     &s  // 返回字符串 s 的引用
// }//这里 s 离开作用域并被丢弃。其内存被释放。
// //危险！！！

fn dangle() -> String {
   let s = String::from("主人下马客在船，举酒欲饮无管弦。");
   s
}//这样就没有任何错误了。所有权被移动出去，所以没有值被释放。

//*****************************************************************/
//**引用的规则                                                     */
//**让我们概括一下之前对引用的讨论：                                 */
//**                                                              */
//**在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。*/
//**引用必须总是有效的。                                            */
//*****************************************************************/
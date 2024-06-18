struct Color(i32,i32,i32);
struct Point(i32,i32,i32);

struct AlwaysEqual;

fn main() {
    println!("👉👉👉使用没有命名字段的元组结构体来创建不同的类型👈👈👈");
    
    //元组结构体
    let black = Color(0,0,0);
    let zero = Point(0,0,0);
    println!("{}",black.1);

    //类单元结构体
    let subject = AlwaysEqual;
}

    //结构体数据的所有权
    //在示例 5-1 中的 User 结构体的定义中，我们使用了自身拥有所有权的 String 类型而不是 &str 字符串 slice 类型。
    //这是一个有意而为之的选择，因为我们想要这个结构体拥有它所有的数据，为此只要整个结构体是有效的话其数据也是有效的。
    //可以使结构体存储被其他对象拥有的数据的引用，不过这么做的话需要用上生命周期（lifetime），这是一个第 10 章会讨论的 Rust 功能。
    //生命周期确保结构体引用的数据有效性跟结构体本身保持一致。如果你尝试在结构体中存储一个引用而不指定生命周期将是无效的，比如这样：
    struct User {
       //username: &str,
       //email: &str,
       sign_in_count: u64,
       active: bool,
    }
    //这会导致编译错误，因为 Rust 不知道 &str 引用的生命周期是什么。
    //为了解决这个问题，我们需要指定生命周期，让 Rust 知道 &str 引用的生命周期是有效的。
    //在 Rust 中，生命周期是泛指某一块数据的有效期。
    //生命周期的语法是尖括号包裹的生命周期标注，比如 <'a> 或 <'static>。
    //生命周期标注放在结构体名之后，紧跟在结构体名后面的空格后面。
    struct User2<'a> {
        username: &'a str,
        email: &'a str,
        sign_in_count: u64,
        active: bool,
     }
use std::{fmt::Error, fs::File, io::{self, Read}};

fn main() {
    println!("Result and recoverable.");
    // let file_p = File::open("静夜思.txt");
    // let f = match file_p {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the 静夜思.txt {:?}",error),
    // };
    // println!("{:?}",f);

    let f2 = File::open("七律_长征.txt");
    let mut changzheng = match f2 {
        Ok(file) => file,
        Err(error) => panic!("未发现文件‘七律_长征.txt’{:?}",error),
    };
    println!("七律_长征:{:?}",changzheng);

    let mut qilv_changzheng = vec![];
    changzheng.read_to_end(& mut qilv_changzheng);
    //println!("{:?}",qilv_changzheng);

    for i in &qilv_changzheng {
        print!("{}",i);
    }

    let ff = qilv_changzheng.get(0..48);
    println!("\n{:?}",ff);

    //匹配不同的错误
    let f = File::open("hello.txt");

    let _f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("创建文件出错{:?}",e),
            },
            other_error => panic!("Not found this file.{:?}",other_error),
        },
    };

    //这里有好多 match！match 确实很强大，不过也非常的基础。第 13 章我们会介绍闭包（closure）。
    //Result<T, E> 有很多接受闭包的方法，并采用 match 表达式实现。
    //一个更老练的 Rustacean 可能会这么写：
    let _f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    //失败时panic！的简写:unwrap或者expect
    //match 能够胜任它的工作，不过它可能有点冗长并且不总是能很好地表明其意图。Result<T, E> 类型定义了很多辅助方法来处理各种情况。
    //其中之一叫做 unwrap，它的实现就类似于示例 9-4 中的 match 语句。如果 Result 值是成员 Ok，unwrap 会返回 Ok 中的值。
    //如果 Result 是成员 Err，unwrap 会为我们调用 panic!。这里是一个实践 unwrap 的例子：

    //let _ff1 = File::open("将进酒.txt").unwrap();
    //let _ff2 = File::open("卜算子_咏梅.txt").expect("未能正常打开文件，文件损坏或者不存在。");
    
    let res = read_username_from_file();
    println!("{:?}",res);
}

//传播错误
// fn read_username_from_file() -> Result<String,io::Error> {
//     let f = File::open("七律_长征.txt");
//     let mut f = match f {
//         Ok(file) => file,
//         Err(error) => return Err(error),
//     };
//     let mut s = String::new();
//     match f.read_to_string(&mut s) {
//         Ok(_) => Ok(s),
//         Err(e) => Err(e),
//     }
// }
//传播错误的简写：？运算符
// fn read_username_from_file() -> Result<String,io::Error> {
//     let mut f = File::open("七律_长征")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }

//"?"运算符消除了大量样板代码并使得函数的实现更简单。我们甚至可以在"?"之后直接使用链式方法调用来进一步缩短代码
fn read_username_from_file() -> Result<String,io::Error> {
    let mut s = String::new();
    File::open("七律_长征.txt")?.read_to_string(&mut s)?;
    Ok(s)
}

//更简短的写法，使用“fs::read_to_string”.
// fn read_username_from_file() -> Result<String,io::Error> {
//     std::fs::read_to_string("七律_长征.txt")
// }

// * "?"运算符可被用于返回 Result 的函数

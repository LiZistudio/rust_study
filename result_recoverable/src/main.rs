use std::{fmt::Error, fs::File, io::Read};



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
    let mut qilv_changzheng = Vec::new();
    changzheng.read_to_end(& mut qilv_changzheng);
    //println!("{:?}",qilv_changzheng);
    for i in &qilv_changzheng {
        print!("{}",i);
    }
    let ff = qilv_changzheng.get(0..48);
    println!("\n{:?}",ff);

    //匹配不同的错误
    let f = File::open("hello.txt");
    let f = match f {
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
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    //失败时panic！的简写:unwrap或者expect
    let ff1 = File::open("将进酒.txt").unwrap();
    let ff2 = File::open("卜算子_咏梅.txt").expect("未能正常打开文件，文件损坏或者不存在。");
    
}

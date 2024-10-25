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
}

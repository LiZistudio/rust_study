/*
如何编写测试
Rust 中的测试函数是用来验证非测试代码是否按照期望的方式运行的。测试函数体通常执行如下三种操作：
1.设置任何所需的数据或状态
2.运行需要测试的代码
3.断言其结果是我们所期望的
让我们看看 Rust 提供的专门用来编写测试的功能：test 属性、一些宏和 should_panic 属性。
*/

//测试函数剖析

use std::{
    fs::File,
    io::Write,
    os::windows::fs::FileExt,
};

    
fn main() {
    println!("Rust 编写自动化测试");
    create_file_success();    
}

fn create_file_success() {

    let mut my_file = create_file("poem.md");
    for i in 1..11 {
        if i%2==0 {
            my_file.write(b"\n# To be or not to be , this is a question.")
            .expect("写入失败");
        }else {
            //my_file.write(b"\n## To be or not to be , this is a question.").expect("写入失败");
            my_file.seek_write(b" ",0)
            .expect("写入失败");
        }
    }
}

pub fn create_file(name:&str) -> File {
    let file = File::create(name).expect("创建文件失败");
    file
}

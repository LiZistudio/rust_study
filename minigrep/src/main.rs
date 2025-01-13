

/***********接受命令行参数**************/

// use encoding_rs::{GB18030, WINDOWS_1252};
// use std::path::Path;
// use std::fs::File;
// use std::io::{BufRead, BufReader};
// use encoding_rs_io::DecodeReaderBytesBuilder;



// fn main() -> std::io::Result<()> {
//     let mut path_and_grep = String::new();
//     println!("请输入文件路径和要搜索的字符串，（中间用空格隔开）;-)");
//     std::io::stdin().read_line(&mut path_and_grep)?;

//     let args:Vec<&str> = path_and_grep.split_whitespace().collect();
//     if args.len() < 2 {
//         eprintln!("--------->>>参数不足<<<------------");
//         return Ok(());
//     }

//     let path = args[0].trim().trim_matches('"');
//     let grep = args[1].trim();

//     let file_path = Path::new(path);

//     if !file_path.exists() {
//         eprintln!("文件\"{}\"不存在",path);
//         return Ok(());
//     }

//     // 获取文件扩展名
//     let is_word_doc = match file_path.extension().and_then(|ext| ext.to_str()) {
//         Some(ext) => ext.eq_ignore_ascii_case("doc") || ext.eq_ignore_ascii_case("docx"),
//         None => false,
//     };

//     let file = match File::open(file_path) {
//         Ok(file) => file,
//         Err(e) => {
//             eprintln!("无法打开文件{}",e);
//             return Ok(());
//         }
//     };

//     let reader: Box<dyn BufRead> = if is_word_doc {
//         Box::new(BufReader::new(DecodeReaderBytesBuilder::new()
//             .encoding(Some(GB18030))
//             .build(file)))
//     } else {
//         Box::new(BufReader::new(file))
//     };

//     let mut counter = 0;
//     let mut found = false;

//     println!("\n搜索结果:");
//     println!("------------------snip-------------------------");

//     for line in reader.lines() {
//         counter += 1;
//         match line {
//             Ok(content) => {
//                 if content.contains(grep) {
//                     println!("行号 {}: {}", counter, content);
//                     found = true;
//                 }
//             },
//             Err(e) => {
//                 eprintln!("警告: 读取第 {} 行时出错: {}", counter, e);
//                 continue;
//             }
//         }
//     }

//     if !found {
//         println!("未找到包含\"{}\"的字符串", grep);
//     }

//     println!("------------------snip-------------------------");
//     println!("总共搜索了 {} 行", counter);

//     println!("\n>>>按Enter键退出<<<...");
//     let mut temp = String::new();
//     std::io::stdin().read_line(&mut temp)?;
//     Ok(())
// }


//args 函数和无效的 Unicode
//注意 std::env::args 在其任何参数包含无效 Unicode 字符时会 panic。
//如果你需要接受包含无效 Unicode 字符的参数，使用 std::env::args_os 代替。
//这个函数返回 OsString 值而不是 String 值。
//这里出于简单考虑使用了 std::env::args，因为 OsString 值每个平台都不一样而且比 String 值处理起来更为复杂。

use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>(); // 获取命令行参数

    let len = args.len(); // 获取参数个数
    println!("{}",len);
    //示例 12-2：创建变量来存放查询参数和文件名参数
    let _path = &args[0]; // 获取第一个参数  //注意 vector 的第一个值是 "target/debug/minigrep"，它是二进制文件的名称。
    let query = &args[1]; // 获取第二个参数
    let filename = &args[2..len].join(" "); // 获取第三个参数
    println!("query: {}", query);
    println!("filename: {}", filename);

    for arg in &args {
        let arg = &arg[..];
        println!("{}",&arg);
    }
}

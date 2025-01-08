

/***********接受命令行参数**************/

use std::{fs::File, io::BufRead, path::Path};



fn main() -> std::io::Result<()> {
    let mut path_and_grep = String::new();
    println!("请输入文件路径和要搜索的字符串，（中间用空格隔开）;-)");
    std::io::stdin().read_line(&mut path_and_grep)?;

    let args:Vec<&str> = path_and_grep.split_whitespace().collect();
    if args.len() < 2 {
        eprintln!("参数不足");
    }

    let path = args[0].trim().trim_matches('"');
    let grep = args[1].trim();

    let file_path = Path::new(path);
    let file = File::open(file_path)?;
    let reader = std::io::BufReader::new(file);

    let mut counter = 0;
    let mut found = false;

    println!("\n搜索结果:");
    println!("------------------snip-------------------------");

    for line in reader.lines() {
        counter += 1;
        let line = line?;
        match line.contains(&grep) {
            true => {
                println!("该字符串出现在第{}行，为{:?}", counter,line);
                found = true;
            },
            false => continue,
        }
    }
    println!("------------------snip-------------------------");

    if !found {
        println!("未找到包含\"{}\"的字符串", grep);
    }

    println!("\n>>>按Enter键退出<<<...");
    let mut temp = String::new();
    std::io::stdin().read_line(&mut temp)?;
    Ok(())
}
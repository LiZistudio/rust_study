
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};
use std::process::Command;
use colored::Colorize;

fn main() -> std::io::Result<()> {
    loop {
        Command::new("cmd.exe")
            .arg("/c")
            .arg("cls")
            .status()
            .expect("clear error!");

        println!("{}","欢迎使用打样材料库存查询系统;-)".bright_blue().red());
        println!("{}","*****************************************".bright_blue().red());
        println!("**      1.查询      0.退出(请输入序号) **");
        println!("**                                     **");
        println!("**                 (Copyright ©2025 Kk)**");
        println!("*****************************************");

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice)?;

        match choice.trim().parse::<i32>() {
            Ok(0) => {
                println!("感谢使用，再见。;-)");
                pause()?;
                break;
            },
            Ok(1) => {
                println!("请输入要搜索的线材或树脂或树脂槽类型;-)");
                let mut filament_kind = String::new();
                std::io::stdin().read_line(&mut filament_kind)?;

                let filament_kind = filament_kind.trim();
                let file_path = Path::new("filament_overview.txt");

                let file = match File::open(file_path) {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("无法打开文件{}", e);
                        pause()?;
                        return Ok(());
                    }
                };

                let reader = BufReader::new(file);

                let mut counter = 0;
                let mut found = false;
                let mut match_count = 0;

                println!("\n正在搜索 \"{}\" ...", filament_kind);
                println!("==================snip========================");

                for line in reader.lines() {
                    counter += 1;
                    match line {
                        Ok(content) => {
                            if content.contains(&filament_kind) {
                                found = true;
                                match_count += 1;
                                println!("\n查找结果 #{}", match_count);
                                println!("行号: {}", counter);
                                println!("原始内容: {}", content);
                                
                                let kind: Vec<&str> = content.split_whitespace().collect();

                                if !kind.is_empty() {
                                    let filament_name = &kind[..kind.len()-1].join(" ");
                                    let filament_number = kind[kind.len()-1];

                                    println!("库存信息: 当前库中有{},数量为{}", filament_name, filament_number);
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("警告: 读取第 {} 行时出错 - {}", counter, e);
                            continue;
                        }
                    }
                }

                println!("\n====================snip======================");
                if !found {
                    println!("未找到 材料\"{}\"", filament_kind);
                } else {
                    println!("共找到 {} 处匹配", match_count);
                }
                println!("总共搜索了 {} 行内容", counter);

                pause()?;
            },
            _ => {
                println!("无效输入，请重试!");
                std::thread::sleep(std::time::Duration::from_secs(1));
                continue;
            }
        }
    }
    Ok(())
}

fn pause() -> std::io::Result<()> {
    println!("\n按Enter键继续...");
    io::stdout().flush()?;
    let mut temp = String::new();
    std::io::stdin().read_line(&mut temp)?;
    Ok(())
}

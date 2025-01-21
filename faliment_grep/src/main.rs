
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

        println!("{}","欢迎使用打样材料库存查询系统;-)          ".on_bright_white().bright_red());
        println!("{}","*****************************************".on_bright_white().bright_red());
        println!("{}","**      1.🔍查询           0.⏏️ 退出    **".on_bright_white().bright_red());
        println!("{}","**          (请输入对应序号)           **".on_bright_white().bright_red());
        println!("{}","**                 (Copyright ©2025 Kk)**".on_bright_white().bright_red());
        println!("{}","*****************************************".on_bright_white().bright_red());

        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice)?;

        match choice.trim().parse::<i32>() {
            Ok(0) => {
                println!("{}","感谢使用，再见。😉".bright_red());
                pause()?;
                break;
            },
            Ok(1) => {
                println!("{}","请输入要搜索的线材或树脂或树脂槽类型🫱".bright_red());
                let mut filament_kind = String::new();
                std::io::stdin().read_line(&mut filament_kind)?;

                let filament_kind = filament_kind.trim();
                let file_path = Path::new("filament_overview.txt");

                let file = match File::open(file_path) {
                    Ok(file) => file,
                    Err(e) => {
                        eprintln!("{}{}","无法打开文件".red(), e);
                        pause()?;
                        return Ok(());
                    }
                };

                let reader = BufReader::new(file);

                let mut counter = 0;
                let mut found = false;
                let mut match_count = 0;

                println!("\n{} \"{}\" {}","正在搜索".blue(), filament_kind.red(),"...".blue());
                println!("{}","==================snip========================".bright_yellow());

                for line in reader.lines() {
                    counter += 1;
                    match line {
                        Ok(content) => {
                            if content.to_lowercase().contains(&filament_kind.to_lowercase()) {
                                found = true;
                                match_count += 1;
                                println!("{}\n{}","查找结果 #".bright_blue(), match_count);
                                println!("{} {}","行号:".yellow(), counter);
                                println!("{} {}","原始内容:".yellow(), content.bright_red());
                                
                                let kind: Vec<&str> = content.split_whitespace().collect();

                                if !kind.is_empty() {
                                    let filament_name = &kind[..kind.len()-1].join(" ");
                                    let filament_number = kind[kind.len()-1];

                                    println!("{}{},{}{}",
                                    "库存信息: 当前库中有".bright_blue(), 
                                    filament_name.bright_red(),
                                    "数量为".bright_blue(), 
                                    filament_number.bright_red());
                                }
                            }
                        },
                        Err(e) => {
                            eprintln!("{} {}{}{}","警告⚠️ : 读取第".red(), counter," 行时出错 - ".red(), e);
                            continue;
                        }
                    }
                }

                println!("\n{}","====================snip======================".bright_yellow());
                if !found {
                    println!("{}\"{}\"","未找到 材料".blue(), filament_kind.red());
                } else {
                    println!("{}{}{}","共找到 ".blue(), match_count," 处匹配".blue());
                }
                println!("{}{}{}","总共搜索了 ".blue(), counter," 行内容".blue());

                pause()?;
            },
            _ => {
                println!("{}","⚠️ 无效输入，请重试".bright_red());
                std::thread::sleep(std::time::Duration::from_secs(1));
                continue;
            }
        }
    }
    Ok(())
}

fn pause() -> std::io::Result<()> {
    println!("\n{}","按Enter键继续...".yellow());
    io::stdout().flush()?;
    let mut temp = String::new();
    std::io::stdin().read_line(&mut temp)?;
    Ok(())
}

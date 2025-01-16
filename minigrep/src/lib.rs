use std::error::Error;
use std::fs;
use std::io::BufRead;
use std::{io, path::Path};
use fs::File;

pub fn run(config:&Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(&config.filename)?;
    println!("{}",contents);
    Ok(())
}

pub struct Config {
    pub query:String,
    pub filename:String,
}
//解析参数
impl Config {
    pub fn new(args:&[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("参数不足");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok(Config{query,filename})
    
    }    
}

//search
pub fn search(query:&str,filename:&str) -> Vec<String> {
    let mut result = Vec::new();
    let path = Path::new(filename);
    let file = File::open(path);
    let file = match file {
        Ok(file) => file,
        Err(e) => panic!("打开文件失败{}",e),
    };
    let reader = io::BufReader::new(file);
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.contains(query) {
                result.push(line);
            }
        }
    }
    result
}


#[cfg(test)]
mod test {
    
    use super::*;

    #[test]
    fn test_run_print_poem() {
        let config = Config {
            query:String::from("us"),
            filename:String::from("for_grep.md"),
        };

        if let Err(e) = run(&config) {
            eprintln!("读取错误:{}",e);
        }
    }

    #[test]
    fn one_result() {
        let query = "us";
        let filename = "for_grep.md";
        let result_arr = search(query,filename);

        let expected_arr = vec![
            String::from("# Then there's a pair of us - don't tell!"),
            String::from("# They'd banish us, you know.")
        ];

        assert_eq!(result_arr,expected_arr);
    }
}


use std::error::Error;
use std::fs;

pub fn run(config:&Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(&config.filename)?;
    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
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
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
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
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}

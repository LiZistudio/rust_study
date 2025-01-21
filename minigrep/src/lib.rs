use std::error::Error;
use std::fs;
use std::env;

pub fn run(config:&Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    }else{
        search_case_insensitive(&config.query, &contents)
    };
    for line in results {
        println!("{}",line);
    }
    Ok(())
}

pub struct Config {
    pub query:String,
    pub filename:String,
    pub case_sensitive:bool,
}
//解析参数
impl Config {
    pub fn new(args:&[String]) -> Result<Config,&'static str> {
        if args.len() < 3 {
            return Err("参数不足");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        //示例 12-23：检查叫做 CASE_INSENSITIVE 的环境变量
        Ok(Config{query,filename,case_sensitive})
    
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

//实现 search_case_insensitive 函数
pub fn search_case_insensitive<'a>(query: &str,contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results:Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query[..]) {
            results.push(line);
        }
    }
    results
}
//示例 12-21：定义 search_case_insensitive 函数，它在比较查询和每一行之前将他们都转换为小写

#[cfg(test)]
mod test {
    
    use super::*;

    #[test]
    fn test_run_print_poem() {
        let config = Config {
            query:String::from("us"),
            filename:String::from("for_grep.md"),
            case_sensitive:true,
        };

        if let Err(e) = run(&config) {
            eprintln!("读取错误:{}",e);
        }
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    //为准备添加的大小写不敏感函数新增失败测试
    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

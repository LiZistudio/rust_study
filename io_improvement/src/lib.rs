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
    
    //示例 13-26：以迭代器作为参数更新 Config::new 的签名
    pub fn new(mut args:std::env::Args) -> Result<Config,&'static str> {
        args.next();

        //使用 Iterator trait 代替索引
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    
        //示例 12-23：检查叫做 CASE_INSENSITIVE 的环境变量
        Ok(Config{query,filename,case_sensitive})
    
    }
}//示例 13-27：修改 Config::new 的函数体来使用迭代器方法


//search
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut results = Vec::new();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line);
    //     }
    // }

    // results

    contents.lines()
    .filter(|line| line.contains(query))
    .collect()

}//示例 13-29：在 search 函数实现中使用迭代器适配器


//实现 search_case_insensitive 函数
pub fn search_case_insensitive<'a>(query: &str,contents: &'a str) -> Vec<&'a str> {
    //let query = query.to_lowercase();
    // let mut results:Vec<&str> = Vec::new();

    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query[..]) {
    //         results.push(line);
    //     }
    // }
    // results

    contents.lines()
    .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
    .collect()
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

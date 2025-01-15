use std::error::Error;
use std::fs;


pub fn run(config:Config) -> Result<(), Box<dyn Error>>{
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

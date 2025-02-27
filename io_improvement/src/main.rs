use std::env;
use std::process;
use io_improvement::Config;

fn main() /*-> std::io::Result<()>*/ {
    // let args:Vec<String> = env::args().collect();

    //直接使用 env::args 返回的迭代器
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    //println!("query: {},filename:{}", config.query,config.filename);

    if let Err(e) = io_improvement::run(&config) {
        eprintln!("Application error: {}",e);
        process::exit(1);
    }

    //Ok(())
}

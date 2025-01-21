use colored::*;
use std::process::Command;
use std::thread;
use std::time::Duration;

fn main() {
    shut_down();

    let mut i = 1;
    while i<=8 {
        println!("\t\t\t\t{}","您的电脑将在20秒后关机！！！".red().on_bright_yellow());

        println!("\n\n\n\n\n\n\t\t{}"," ▄█     █▄     ▄████████    ▄████████  ▄█  ███▄▄▄▄      ▄██████▄".red());  
        println!("\t\t{}"," ███    ███   ███    ███   ███    ███ ███  ███▀▀▀██▄   ███    ███".red());  
        println!("\t\t{}"," ███    ███   ███    ███   ███    ███ ███▌ ███   ███   ███    █▀".red());  
        println!("\t\t{}"," ███    ███   ███    ███  ▄███▄▄▄▄██▀ ███▌ ███   ███  ▄███".red());  
        println!("\t\t{}"," ███    ███ ▀███████████ ▀▀███▀▀▀▀▀   ███▌ ███   ███ ▀▀███ ████▄".red());  
        println!("\t\t{}"," ███    ███   ███    ███ ▀███████████ ███  ███   ███   ███    ███".red());  
        println!("\t\t{}"," ████▄█▄███   ███    ███   ███    ███ ███  ███   ███   ███    ███".red());  
        println!("\t\t{}"," ▀▀███▀███▀   ███    █▀    ███    ███ █▀    ▀█   █▀    ████████▀".red());  
        println!("\t\t{}","                           ███    ███".red());  

        thread::sleep(Duration::from_millis(1000));
        clear_screen();
        thread::sleep(Duration::from_millis(1000));

        i+=1;
       
    }
    
    
    println!("\n\n\n\n\n\n\t\t\t{}","██████╗".red().on_yellow()); 
    println!("\t\t\t{}","╚════██╗".red().on_yellow()); 
    println!("\t\t\t{}","█████╔╝".red().on_yellow()); 
    println!("\t\t\t{}"," ╚═══██╗".red().on_yellow()); 
    println!("\t\t\t{}","██████╔╝".red().on_yellow()); 
    println!("\t\t\t{}","╚═════╝".red().on_yellow());
    thread::sleep(Duration::from_millis(1000));
    clear_screen();
    //thread::sleep(Duration::from_millis(500));

    println!("\n\n\n\n\n\n\t\t\t{}","██████╗".red().on_yellow()); 
    println!("\t\t\t{}","╚════██╗".red().on_yellow()); 
    println!("\t\t\t{}"," █████╔╝".red().on_yellow()); 
    println!("\t\t\t{}","██╔═══╝ ".red().on_yellow()); 
    println!("\t\t\t{}","███████╗".red().on_yellow()); 
    println!("\t\t\t{}","╚══════╝".red().on_yellow());
    thread::sleep(Duration::from_millis(1000));
    clear_screen();
    //thread::sleep(Duration::from_millis(500));

    println!("\n\n\n\n\n\n\t\t\t{}","   ██╗".red().on_yellow()); 
    println!("\t\t\t{}","  ███║".red().on_yellow()); 
    println!("\t\t\t{}","  ╚██║".red().on_yellow()); 
    println!("\t\t\t{}","   ██║".red().on_yellow()); 
    println!("\t\t\t{}","   ██║".red().on_yellow()); 
    println!("\t\t\t{}","   ╚═╝".red().on_yellow());
    thread::sleep(Duration::from_millis(1000));
}


fn clear_screen(){
    Command::new("cmd")
            .args(["/C", "cls"])
            .status()
            .unwrap();
}

fn shut_down() {
    Command::new("cmd")
            .args(["/C", "shutdown", "/s", "/t", "20"])
            .status()
            .unwrap();
}

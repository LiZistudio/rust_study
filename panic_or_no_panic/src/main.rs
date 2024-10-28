use std::net::IpAddr;
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("panic_or_no_panic");

    //示例、代码原型和测试都非常适合 panic
    //在我们准备好决定如何处理错误之前，unwrap 和 expect 方法在原型设计时非常方便。当我们准备好让程序更加健壮时，它们会在代码中留下清晰的标记。


    //当我们比编译器知道更多的情况
    let home:IpAddr = "127.0.0.1".parse().expect("地址无效");
    println!("{:?}",home);

    //错误处理指导原则
    //当代码有可能以有害状态结束时，建议让代码 panic。在这里，有害状态是指一些假设、保证、协议或不可变性被打破的状态，比如无效的值、矛盾的值或缺失的值被传递给代码——加上以下一种或多种情况：
    //有害状态是指一些意外的事情，而不是预期可能偶尔发生的事情，比如用户输入错误格式的数据。
    //在此之后的代码需要摆脱这种有害状态，而不是在每一步都检查这个问题。
    //在使用的类型中，没有一个好的方式来编码这些信息。我们将在第 17 章的“将状态和行为编码为类型”一节中通过一个例子来说明我们阐述的意思。

    //创建自定义类型进行有效性验证
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        if guess<1 || guess>100 {
            println!("请在1到100数字之间输入");
            continue;
        }

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

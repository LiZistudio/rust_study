//**************循环****************
//Rust 有三种循环：loop、while 和 for。

//如果存在嵌套循环，break 和 continue 应用于此时最内层的循环。
//你可以选择在一个循环上指定一个循环标签（loop label），然后将标签与 break 或 continue 一起使用，
//使这些关键字应用于已标记的循环而不是最内层的循环。
fn main() {
    println!("Hello, world!");
    // loop {
    //     println!("free loop");
    // }
    //loop循环
    let mut count = 0;
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;

        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {}", count);

    let mut counter:i32 = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("result is {result}");

    //while循环
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    //for循环
    for item in a {
        println!("the value is: {}", item);
    }

    for number in (1..4).rev() {
        println!("the value is {number}");
    }
}

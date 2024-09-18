/*****************************************match 控制流运算符***************************************/

// fn main() {
//     println!(" 😘 😘 😘 😘 😘 match :) 😘 😘 😘 😘 😘 ");

//     let number = value_in_rmb(Rmb::RedMao);
//     println!("number is {:?}",number);

//     let number = value_in_rmb(Rmb::Budala(Place::LaSa));

// }

// #[derive(Debug)]
// enum Place {
//     LaSa,
// }

// enum Rmb {
//     GreenMao,
//     Budala(Place),
//     RedMao,
// }

// //绑定值的模式
// fn value_in_rmb (rmb:Rmb) -> u8 {
//     match rmb {
//         Rmb::RedMao => {
//             println!("恭喜你,拿到一张毛爷爷");
//             100
//         },
//         Rmb::GreenMao => 1,
//         Rmb::Budala(city) => {
//             println!("布达拉宫在{:?}",city);
//             50
//         },
//     }
// }

fn main () {
    let money_value = value_in_cents(Coin::Penny);
    println!("{:?}",money_value);

    let money_value2 = value_in_cents(Coin::Quarter(UsState::Alabama));
    println!("{:?}",money_value2);

    let five = Option::Some(5);
    let six = plus_one(five);
    println!("{:?}",six.unwrap_or_default());
    let none = plus_one(None);

    //通配符和_占位符
    let dice_roll = 10;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),            //Rust 中的匹配是穷举式的!!!    请注意，我们必须将通配分支放在最后，因为模式是按顺序匹配的。
        //通配模式，值绑定到other变量。
        //Rust 还提供了一个模式，当我们不想使用通配模式获取的值时，请使用 _ ，这是一个特殊的模式，
        //可以匹配任意值而不绑定到该值。这告诉 Rust 我们不会使用这个值，所以 Rust 也不会警告我们存在未使用的变量。
        //_ => println!("_占位符,使match匹配时穷尽的,并且部不将值绑定到_"),
        _ => reroll(),
    }
}

#[derive(Debug)]
enum UsState {
    NewYork,
    Alabama,
}
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents (coin:Coin) -> u8 {
    match coin {
        Coin::Penny => {
            print!("Luck Penny    ");
            1
        },
        Coin::Nickel =>5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            print!("From {:?}    ",state);
            25
        },
    }
}

//匹配Option<T>
//匹配Some<T>
fn plus_one (x:Option<i32>) -> Option<i32> {
    match x {
        Option::None => None,                               //Rust 中的匹配是穷举式的!!!
        Option::Some(i) => Some(i+1), 
    }
}

//通配模式和_占位符
//使用见main函数
fn add_fancy_hat() {
    println!("你拥有了一定好看的帽子");
}
fn remove_fancy_hat() {
    println!("摘掉你好看的帽子");
}
fn move_player(num_spaces: u8) {
    println!("向前移动{}步",num_spaces);
}
fn reroll () {
    println!("您掷出的骰子不是3或7,请重新投掷");
}
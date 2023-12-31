use rust_module::add;
use rust_module::pow::{add_1tox,print_1tox,pow};
fn main() {
    println!("*********Hello, world!********");
    let a:i32 = 2;
    let b:i32 = 10;
    let res1:i32 = add::add_int(a,b);
    let res2:i32 = pow::pow_int(a,b);
    let res4:i32 = pow::pow_int(2,8);
    println!("2的8次方为：{}",res4);
    println!("{}加{}为{}",a,b, res1);
    println!("{}的{}次方为{}",a,b, res2);
    let res3 = add_1tox::add_1tox(100);
    println!("1to100 sum is {}",res3);
    print_1tox::print_1tox(10);

}

mod add;
use add::add::add;
mod sub;
fn main() {
    println!("{}",add(3.33, 7.77));
    let result = sub::sub::sub(3.33, 7.77);
    println!("{}",result);
}

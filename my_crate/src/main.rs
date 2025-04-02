
//将 crate 发布到 Crates.io

use add_one::add;
use art::kinds::PrimaryColor;
use art::utils::mix;

fn main() {
    println!("将 crate 发布到 Crates.io");

    let result = add(0.3, 0.7);
    println!("{}", result);

    '_export_art:{
        let red = PrimaryColor::Red;
        let yellow = PrimaryColor::Yellow;

        let mix_color = mix(red, yellow);
        println!("{:?}", mix_color);
    }
    //示例 14-4：一个通过导出内部结构使用 art crate 中项的 crate

}

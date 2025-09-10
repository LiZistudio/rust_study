
//将 crate 发布到 Crates.io

use add_one::add;

use art::PrimaryColor;
//use art::kinds::PrimaryColor;
use art::mix;
//use art::utils::mix;
//示例 14-6：一个使用 art crate 中重导出项的程序

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


//使用 pub use 导出合适的公有 API
//见库art

//创建 Crates.io 账号
//在你可以发布任何 crate 之前，需要在 crates.io 上注册账号并获取一个 API token。



//发布新 crate 之前
//向新 crate 添加元信息
//修改 Cargo.toml 文件，添加 crate 的元信息。
// license 标识符值
//有了唯一的名称、版本号、由 cargo new 新建项目时增加的作者信息、描述和所选择的 license，
//已经准备好发布的项目的 Cargo.toml 文件可能看起来像这样：
// [package]
// name = "guessing_game"
// version = "0.1.0"
// edition = "2021"
// description = "A fun game where you guess what number the computer has chosen."
// license = "MIT OR Apache-2.0"

// [dependencies]


//发布到 Crates.io
//发布 crate 时请多加小心，因为发布是 永久性的（permanent）。对应版本不可能被覆盖，其代码也不可能被删除。
//crates.io 的一个主要目标是作为一个存储代码的永久文档服务器，这样所有依赖 crates.io 中的 crate 的项目都能一直正常工作。
//而允许删除版本没办法达成这个目标。然而，可以被发布的版本号却没有限制。
//cargo publish


//发布现存 crate 的新版本
/*当你修改了 crate 并准备好发布新版本时，改变 Cargo.toml 中 version 所指定的值。
请使用语义化版本规则来根据修改的类型决定下一个版本号。接着运行 cargo publish 来上传新版本。*/

//使用 cargo yank 从 Crates.io 撤回版本
/*$ cargo yank --vers 1.0.1
    Updating crates.io index
        Yank guessing_game@1.0.1*/

 /*cargo yank --vers 1.0.1 --undo
    Updating crates.io index
      Unyank guessing_game@1.0.1*/

fn main() {
    println!(" 🫓  🫓  🫓  一个使用结构体的示例程序 🫓  🫓  🫓 ");
    // let width1 = 30;
    // let height1 = 50;

    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(width1, height1)
    // );

    // //使用元组重构
    // let rect1:(u32,u32) = (30,50);
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     area(rect1)
    // );

    //使用结构体重构：赋予更多意义
    let scale = 3;
    let rect1 = Rect { 
        width: dbg!(30*scale), 
        height: 50
     };
     dbg!(&rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("The rect1's width and height:{:#?}", rect1);
}

//获取以像素为单位的长方形的宽度和高度，并计算出长方形的面积。
// fn area (length:u32, width:u32) -> u32 {
//     length * width
// }

// fn area (rect:(u32,u32)) -> u32 {
//     rect.0 * rect.1
// }

//使用结构体重构：赋予更多意义

#[derive(Debug)]
struct Rect {
    width:u32,
    height:u32,
}
fn area (rect:&Rect) -> u32 {
    rect.width * rect.height
}
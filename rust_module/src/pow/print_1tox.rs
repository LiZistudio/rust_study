// pub fn print_1tox(x:i32) {
//     let mut i = 1;
//     while i <= x {
//         println!("{}", i);
//         i += 1;
//     }
// }

/*******递归实现*******/

pub fn print_1tox(x:i32) {
    if 1==x {
        println!("{}", 1);
    }else {
        print_1tox(x-1);
        println!("{}", x);
    }
}
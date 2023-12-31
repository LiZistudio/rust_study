// pub fn add_1tox(x:i32) -> i32 {
//     let res = x*(x+1)/2;         //前n项和
//     return res;
// }

//add_1tox递归实现
pub fn add_1tox(x:i32) -> i32 {
    if 1==x {
        return 1;
    }
    else {
        let res = x + add_1tox(x-1);
        return res;
    }
}

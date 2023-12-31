// pub fn pow_int (a:i32, b:i32) -> i32{
//     let mut i = 0;
//     let mut res = 1;
//     while i < b {
//         i += 1;
//         res = res * a;
//     }
//     res
// }

/*********递归实现***********/
pub fn pow_int (a:i32, b:i32) -> i32{
    if 1 == b {
        return a;
    }else {
        let result = a*pow_int(a, b-1);
        return result;
    }
}
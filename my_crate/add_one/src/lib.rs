//注释包含项的结构
//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
// --snip--
//示例 14-2：my_crate crate 整体的文档


//编写有用的文档注释


///计算两个32位浮点数的和
/// 
/// #Examples
/// 
/// ```
/// let a = 1.0;
/// let b = 2.0;
/// let result = my_crate::add(a, b);
/// 
/// assert_eq!(result, 3.0);
///
pub fn add(left: f32, right: f32) -> f32 {
    left + right
}


/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
///
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2.0, 2.0);
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_add_one() {
        assert_eq!(3.0, add_one(2.0));
    }

    #[test]
    fn closures() {
        let plus_one = |x| x + 1;
        assert_eq!(3, plus_one(2));

        let a = 10;
        let plus_a_b = |x:i32| -> i32 {
            let result = x + a; result
        };

        assert_eq!(12, plus_a_b(2));
    }
}


//常用（文档注释）部分
//# Examples
//# Panics
//# Errors
//# Safety

//文档注释作为测试

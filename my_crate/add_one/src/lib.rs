//编写有用的文档注释

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

pub fn add(left: f32, right: f32) -> f32 {
    left + right
}

pub fn add_one(x: f32) -> f32 {
    x + 1.0
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
}

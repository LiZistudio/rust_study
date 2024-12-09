// /作为最简单例子，Rust 中的测试就是一个带有 test 属性标注的函数。

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn exploration() {
        let result = add(3, 3);
        assert_eq!(result,7);
    }
}

//---------------snip----------------由 cargo new 自动生成的测试模块和函数


//! # Art
//!
//! # 这是一个用于对艺术概念进行建模的库
//! # 该库包含两种颜色的枚举类型：
//! - `PrimaryColor`：表示三种原色（红色、黄色和蓝色）
//! - `SecondaryColor`：表示三种混合色（橙色、绿色和紫色）

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;
//示例 14-5：增加 pub use 语句重导出项


pub mod kinds {
    /// The primary colors according to the RYB color model.
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    #[derive(PartialEq)]
    #[derive(Debug)]
    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    /// Combines two primary colors in equal amounts to create
    /// a secondary color.
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        let tup:(PrimaryColor,PrimaryColor) = (c1, c2);
        match tup {
            (PrimaryColor::Red, PrimaryColor::Yellow) => SecondaryColor::Orange,
            (PrimaryColor::Yellow, PrimaryColor::Red) => SecondaryColor::Orange,
            (PrimaryColor::Red, PrimaryColor::Blue) => SecondaryColor::Purple,
            (PrimaryColor::Blue, PrimaryColor::Red) => SecondaryColor::Purple,
            (PrimaryColor::Yellow, PrimaryColor::Blue) => SecondaryColor::Green,
            (PrimaryColor::Blue, PrimaryColor::Yellow) => SecondaryColor::Green,
            _ => panic!("Don't need to mix the same color"),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_mix() {
        use crate::kinds::*;
        use crate::utils::*;
    
        assert_eq!(mix(PrimaryColor::Red, PrimaryColor::Yellow), SecondaryColor::Orange);
    }
}
//示例 14-3：一个库 art 其组织包含 kinds 和 utils 模块

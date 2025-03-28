pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        match (c1, c2) {
            (PrimaryColor::Red, PrimaryColor::Yellow) => SecondaryColor::Orange,
            (PrimaryColor::Yellow, PrimaryColor::Red) => SecondaryColor::Orange,
            (PrimaryColor::Red, PrimaryColor::Blue) => SecondaryColor::Purple,
            (PrimaryColor::Blue, PrimaryColor::Red) => SecondaryColor::Purple,
            (PrimaryColor::Yellow, PrimaryColor::Blue) => SecondaryColor::Green,
            (PrimaryColor::Blue, PrimaryColor::Yellow) => SecondaryColor::Green,
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

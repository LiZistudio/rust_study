pub mod kinds {
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    #[derive(PartialEq)]
    #[derive(Debug)]
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod utils {
    use crate::kinds::*;

    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
        let tup = (c1, c2);
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

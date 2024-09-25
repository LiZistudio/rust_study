// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }


mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist () {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant () {
    // //绝对路径
    // crate::front_of_house::hosting::add_to_waitlist();
    // //相对路径
    // front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
}
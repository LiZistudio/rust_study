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


pub mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant () {
    front_of_house::hosting::add_to_waitlist(); //相对路径
    //restaurant::front_of_house::hosting::add_to_waitlist(); //绝对路径
    crate::front_of_house::hosting::add_to_waitlist(); //绝对路径
}

fn main() {
    println!("absolute_path and relative path");
    println!("路径用于引用模块树中的项");

    //路径有两种形式：
    //绝对路径（absolute path）从 crate 根部开始，以 crate 名或者字面量 crate 开头。
    //相对路径（relative path）从当前模块开始，以 self、super 或当前模块的标识符开头。


    //使用pub关键字暴露路径
    pub mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {} //这表明使模块公有并不使其内容也是公有的。模块上的 pub 关键字只允许其父模块引用它。
    
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
        //crate::front_of_house::hosting::add_to_waitlist(); //绝对路径

        //在夏天点一份黑面包作为早餐
        let mut meal = back_of_restaurant::Breakfast::summer("Ray");
        meal.toast = String::from("mantou");
    }

    pub mod back_of_restaurant {
        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,  
        }

        impl Breakfast {
            pub fn summer (toast:&str) -> Breakfast {
                Breakfast {
                    toast:String::from(toast),
                    seasonal_fruit:String::from("apple"),
                }
            }
        }
    }
}
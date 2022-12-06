// 餐厅前台
pub mod front_of_house {
    // 客户服务
    pub mod hosting {
        // 点单等待队列
        pub fn add_to_waitlist() {}
        // 落座
        pub fn seat_at_table() {}
    }

    // 服务员服务
    pub mod serving {
        // 获取订单
        pub fn take_order() {}
        // 处理订单
        pub fn serve_order() {}
        // 处理支付
        pub fn take_payment() {}
    }
}
// 🤔 crate 类比目录结构
// hosting 和 serving 互为 siblings

fn super_serve_order() {}
// 后厨
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub fn fix_incorrect_order() {
        cook_order();
        super::super_serve_order();
    }

    fn cook_order() {}
}

pub fn eat_breakfast_at_restaurant() {
    // 订购黑麦吐司面包作为早餐
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // 改变主意想要换一个面包类型
    back_of_house::fix_incorrect_order();
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
    // 不允许访问早餐附带的季节水果
    // meal.seasonal_fruit = String::from("blueberries");
}

// 使用 use absolute path
use crate::front_of_house::hosting;
// relative path
//use self::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径
    front_of_house::hosting::add_to_waitlist();

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

    hosting::add_to_waitlist();
}

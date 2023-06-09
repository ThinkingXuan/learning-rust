mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist(){
            println!("add_to_waitlist");
        }
        fn seat_at_tables(){}
    }

    mod serving {
        fn take_order(){}
        fn serve_order(){}
        fn take_payment(){}
    }
}

pub fn eat_at_restaurant() {
    // 绝对路径
    crate::front_of_house::hosting::add_to_waitlist();
    // 相对路径
    front_of_house::hosting::add_to_waitlist();
}

use crate::front_of_house::hosting;
pub fn eat_at_restaurant2() {
    hosting::add_to_waitlist();
}
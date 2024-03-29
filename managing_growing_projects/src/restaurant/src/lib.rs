use crate::front_of_house::hosting;

mod front_of_house;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    let order1 = back_of_house::Appetizer::Salad;
    let order2 = back_of_house::Appetizer::Soup;
    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.taost = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

fn deliver_order() {}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order()
    }

    fn cook_order() {}
}

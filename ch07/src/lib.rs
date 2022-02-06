#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

fn serve_order() {}

mod front_of_house;

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
                seasonal_fruit: String::from("복숭아"),
            }
        }
    }

    fn cook_order() {}
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }
}

pub use crate::front_of_house::hosting;
// 상대 경로
// use self::front_of_house::hosting;

// 안티 패턴, 관용적이지 않음
// use self::front_of_house::hosting::add_to_waitlist;
// add_to_waitlist();

pub fn eat_at_restaurant() {
    // 절대 경로
    // crate::front_of_house::hosting::add_to_waitlist();
    // 상대 경로
    // front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("플랫브레드");
    meal.toast = String::from("화이트");
    println!("{} 토스트로 주세요", meal.toast);

    // // 에러 발생, 접근 불가능
    // meal.seasonal_fruit = String::from("블루베리");

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

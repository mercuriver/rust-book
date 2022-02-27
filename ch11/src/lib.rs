pub struct Guess {
    value: u32,
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 {
            panic!("값이 1보다 작음. 지정된 값: {}", value)
        } else if value > 100 {
            panic!("값이 100보다 큼. 지정된 값: {}", value)
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_then_100() {
        Guess::new(101);
    }

    #[test]
    #[should_panic(expected = "값이 100보다 큼.")]
    fn greater_then_101() {
        Guess::new(102);
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

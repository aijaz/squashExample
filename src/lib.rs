#![allow(dead_code)]

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

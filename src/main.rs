#[derive(Clone)]
struct BigInt {
    positive: bool,
    numbers: Vec<u8>,
}

impl Default for BigInt {
    fn default() -> Self {
        BigInt {
            positive: true,
            numbers: vec![0],
        }
    }
}

impl BigInt {
    fn new() -> BigInt {
        BigInt::default()
    }
}

#[cfg(test)]
mod tests {
    use crate::BigInt;

    #[test]
    fn default() {
        let def = BigInt::default();
        assert_eq!(def.positive, true);
        assert_eq!(def.numbers, [0].to_vec());
    }

    #[test]
    fn new() {
        let new = BigInt::new();
        assert_eq!(new.positive, true);
        assert_eq!(new.numbers, [0].to_vec());
    }

    #[test]
    fn from() {
        let x = BigInt::from(20);
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [2, 0].to_vec());
        let x = BigInt::from(-20);
        assert_eq!(x.positive, false);
        assert_eq!(x.numbers, [2, 0].to_vec());
        let x = BigInt::from(-320020000981234567890);
        assert_eq!(x.positive, false);
        assert_eq!(
            x.numbers,
            [3, 2, 0, 0, 2, 0, 0, 0, 0, 9, 8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0].to_vec()
        );
    }

    #[test]
    fn from_string_numbers() {
        let x = BigInt::from_str("20");
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [2, 0].to_vec());
        let x = BigInt::from_str("666");
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [6, 6, 6].to_vec());
        let x = BigInt::from_str("-20");
        assert_eq!(x.positive, false);
        assert_eq!(x.numbers, [2, 0].to_vec());
        let x = BigInt::from_str("-320020000981234567890");
        assert_eq!(x.positive, false);
        assert_eq!(
            x.numbers,
            [3, 2, 0, 0, 2, 0, 0, 0, 0, 9, 8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0].to_vec()
        );
        let x = BigInt::from_str("-0");
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [0].to_vec());
    }

    #[test]
    fn from_string_words_from_str() {
        let x = BigInt::from_str("twenty");
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [2, 0].to_vec());
        let x = BigInt::from_str("minus twenty four");
        assert_eq!(x.positive, false);
        assert_eq!(x.numbers, [2, 4].to_vec());
        let x = BigInt::from_str("two milion five hundred fifty thousand twenty one");
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [2, 5, 5, 0, 0, 2, 1].to_vec());
        let x = BigInt::from_str("minus two milion one");
        assert_eq!(x.positive, false);
        assert_eq!(x.numbers, [2, 0, 0, 0, 0, 0, 1].to_vec());
        let x = BigInt::from_str("zero");
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [0].to_vec());
        let x = BigInt::from_str("onse");
        assert!(x.is_err());
        let x = BigInt::from_str("twenty thousand thousand");
        assert!(x.is_err());
        let x = BigInt::from_str("twenty thousand hundred");
        assert!(x.is_err());
    }

    #[test]
    fn from_string_words_from_str_digits() {
        let x = BigInt::from_str("two zero");
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [2, 0].to_vec());
        let x = BigInt::from_str("minus two four");
        assert_eq!(x.positive, false);
        assert_eq!(x.numbers, [2, 4].to_vec());
        let x = BigInt::from_str("two five five zero zero two one");
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [2, 5, 5, 0, 0, 2, 1].to_vec());
        let x = BigInt::from_str("minus two zero zero zero zero zero one");
        assert_eq!(x.positive, false);
        assert_eq!(x.numbers, [2, 0, 0, 0, 0, 0, 1].to_vec());
        let x = BigInt::from_str("zero");
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [0].to_vec());
        let x = BigInt::from_str("onse");
        assert!(x.is_err());
        let x = BigInt::from_str("twenty thousand thousand");
        assert!(x.is_err());
        let x = BigInt::from_str("twenty thousand hundred");
        assert!(x.is_err());
    }

    #[test]
    fn from_string_words_try_from() {
        let x = BigInt::try_from("twenty");
        assert_eq!(x.unwrap().positive, true);
        assert_eq!(x.unwrap().numbers, [2, 0].to_vec());
        let x = BigInt::try_from("minus twenty four");
        assert_eq!(x.unwrap().positive, false);
        assert_eq!(x.unwrap().numbers, [2, 4].to_vec());
        let x = BigInt::try_from("two milion five hundred fifty thousand twenty one");
        assert_eq!(x.unwrap().positive, true);
        assert_eq!(x.unwrap().numbers, [2, 5, 5, 0, 0, 2, 1].to_vec());
        let x = BigInt::try_from("minus two milion one");
        assert_eq!(x.unwrap().positive, false);
        assert_eq!(x.unwrap().numbers, [2, 0, 0, 0, 0, 0, 1].to_vec());
        let x = BigInt::try_from("onse");
        assert!(x.is_err());
        let x = BigInt::try_from("twenty thousand thousand");
        assert!(x.is_err());
        let x = BigInt::try_from("twenty thousand hundred");
        assert!(x.is_err());
    }

    #[test]
    fn display() {
        let x = BigInt::from(1003);
        assert_eq!(format!("Number is: {x}"), "Number is: 1003");
        let x = BigInt::from(0);
        assert_eq!(format!("Number is: {x}"), "Number is: 0");
        let x = BigInt::from(-100);
        assert_eq!(format!("Number is: {x}"), "Number is: -100");
        let x = BigInt::from(-0);
        assert_eq!(format!("Number is: {x}"), "Number is: -0");
        let x = BigInt::from(-1);
        assert_eq!(format!("Number is: {x}"), "Number is: -1");
        let x = BigInt::from(320020000981234567890);
        assert_eq!(
            format!("Number is: {x}"),
            "Number is: 320020000981234567890"
        );
    }

    #[test]
    fn debug() {
        let x = BigInt::from(1003);
        assert_eq!(format!("Number is: {x:?}"), "Number is: +1003");
        let x = BigInt::from(0);
        assert_eq!(format!("Number is: {x:?}"), "Number is: +0");
        let x = BigInt::from(-100);
        assert_eq!(format!("Number is: {x:?}"), "Number is: -100");
        let x = BigInt::from(-0);
        assert_eq!(format!("Number is: {x:?}"), "Number is: -0");
        let x = BigInt::from(-1);
        assert_eq!(format!("Number is: {x:?}"), "Number is: -1");
        let x = BigInt::from(320020000981234567890);
        assert_eq!(
            format!("Number is: {x:?}"),
            "Number is: +320020000981234567890"
        );
    }

    #[test]
    fn to_words() {
        let x = BigInt::from(20);
        assert_eq!(x.to_words(), "two zero");
        let x = BigInt::from(-24);
        assert_eq!(x.to_words(), "minus two four");
        let x = BigInt::from(2550021);
        assert_eq!(x.to_words(), "two five five zero zero two one");
        let x = BigInt::from(-2000000000001);
        assert_eq!(
            x.to_words(),
            "minus two zero zero zero zero zero zero zero zero zero zero zero one"
        );
    }

    #[test]
    fn equal() {
        let x = BigInt::from(10);
        let y = BigInt::from(10);
        assert!(x == y);
        let x = BigInt::from(101010);
        let y = BigInt::from(101010);
        assert!(x == y);
        let x = BigInt::from(0);
        let y = BigInt::from(0);
        assert!(x == y);
        let x = BigInt::from(10);
        let y = BigInt::from(-10);
        assert!(x != y);
        let x = BigInt::from(11);
        let y = BigInt::from(10);
        assert!(x != y);
        let x = BigInt::from(-0);
        let y = BigInt::from(0);
        assert!(x == y);
    }
}

fn main() {}

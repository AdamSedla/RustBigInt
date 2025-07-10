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
    fn negation() {
        let mut x = BigInt::from(1);
        assert_eq!(x.positive, true);
        assert_eq!(x.to_string(), "1");
        x = !x;
        assert_eq!(x.positive, false);
        assert_eq!(x.to_string(), "-1");
        let mut x = BigInt::from(-22);
        assert_eq!(x.positive, false);
        assert_eq!(x.to_string(), "-22");
        x = !x;
        assert_eq!(x.positive, true);
        assert_eq!(x.to_string(), "22");
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

    #[test]
    fn greater() {
        let x: BigInt = BigInt::from(15);
        let y = BigInt::from(10);
        assert!(x > y);
        let x: BigInt = BigInt::from(8);
        let y = BigInt::from(7);
        assert!(x > y);
        let x = BigInt::from(10);
        let y = BigInt::from(10);
        assert!(!(x > y));
        let x = BigInt::from(10);
        let y = BigInt::from(10);
        assert!(x >= y);
        let x = BigInt::from(101010);
        let y = BigInt::from(101010);
        assert!(x >= y);
        let x = BigInt::from(0);
        let y = BigInt::from(0);
        assert!(!(x > y));
        let x = BigInt::from(10);
        let y = BigInt::from(-10);
        assert!(x > y);
        let x = BigInt::from(11);
        let y = BigInt::from(10);
        assert!(x > y);
    }

    #[test]
    fn lesser() {
        let x: BigInt = BigInt::from(0);
        let y = BigInt::from(10);
        assert!(x < y);
        let x: BigInt = BigInt::from(8);
        let y = BigInt::from(9);
        assert!(x < y);
        let x = BigInt::from(10);
        let y = BigInt::from(10);
        assert!(!(x < y));
        let x = BigInt::from(10);
        let y = BigInt::from(10);
        assert!(x <= y);
        let x = BigInt::from(99999999999);
        let y = BigInt::from(99999999999);
        assert!(x <= y);
        let x = BigInt::from(0);
        let y = BigInt::from(0);
        assert!(!(x < y));
        let x = BigInt::from(-10);
        let y = BigInt::from(10);
        assert!(x < y);
        let x = BigInt::from(11);
        let y = BigInt::from(99999999999);
        assert!(x < y);
    }

    #[test]
    fn add() {
        let mut x: BigInt = BigInt::from(10);
        let y = BigInt::from(10);
        let z = x + y;
        x += y;
        assert_eq!(z, 20);
        assert_eq!(z, x);

        let mut x = BigInt::from(101010);
        let y = BigInt::from(101010);
        let z = x + y;
        x += y;
        assert_eq!(z, 202020);
        assert_eq!(z, x);

        let mut x = BigInt::from(0);
        let y = BigInt::from(0);
        let z = x + y;
        x += y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x = BigInt::from(10);
        let y = BigInt::from(-10);
        let z = x + y;
        x += y;
        assert_eq!(z, 0);
        assert!(x != y);

        let mut x = BigInt::from(11);
        let y = BigInt::from(10);
        let z = x + y;
        x += y;
        assert_eq!(z, 21);
        assert!(x != y);

        let mut x = BigInt::from(-0);
        let y = BigInt::from(0);
        let z = x + y;
        x += y;
        assert_eq!(z, 0);
        assert!(x == y);

        let mut x = BigInt::from(6);
        let y = BigInt::from(4);
        let z = x + y;
        x += y;
        assert_eq!(z, 10);
        assert!(x == y);

        let mut x = BigInt::from(-15);
        let y = BigInt::from(-4);
        let z = x + y;
        x += y;
        assert_eq!(z, -19);
        assert!(x == y);
    }

    #[test]
    fn sub() {
        let mut x: BigInt = BigInt::from(10);
        let y = BigInt::from(10);
        let z = x - y;
        x -= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x = BigInt::from(101010);
        let y = BigInt::from(10);
        let z = x - y;
        x -= y;
        assert_eq!(z, 101000);
        assert_eq!(z, x);

        let mut x = BigInt::from(0);
        let y = BigInt::from(0);
        let z = x - y;
        x -= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x = BigInt::from(10);
        let y = BigInt::from(-10);
        let z = x - y;
        x -= y;
        assert_eq!(z, 20);
        assert!(x != y);

        let mut x = BigInt::from(-10);
        let y = BigInt::from(10);
        let z = x - y;
        x -= y;
        assert_eq!(z, -20);
        assert!(x != y);

        let mut x = BigInt::from(11);
        let y = BigInt::from(10);
        let z = x - y;
        x -= y;
        assert_eq!(z, 1);
        assert!(x != y);

        let mut x = BigInt::from(-0);
        let y = BigInt::from(0);
        let z = x - y;
        x -= y;
        assert_eq!(z, 0);
        assert!(x == y);

        let mut x = BigInt::from(6);
        let y = BigInt::from(4);
        let z = x - y;
        x -= y;
        assert_eq!(z, 2);
        assert!(x == y);

        let mut x = BigInt::from(-15);
        let y = BigInt::from(-4);
        let z = x - y;
        x -= y;
        assert_eq!(z, -11);
        assert!(x == y);
    }

    #[test]
    fn mul() {
        let mut x: BigInt = BigInt::from(10);
        let y = BigInt::from(10);
        let z = x * y;
        x *= y;
        assert_eq!(z, 100);
        assert_eq!(z, x);

        let mut x = BigInt::from(101);
        let y = BigInt::from(101);
        let z = x * y;
        x *= y;
        assert_eq!(z, 10201);
        assert_eq!(z, x);

        let mut x = BigInt::from(0);
        let y = BigInt::from(0);
        let z = x * y;
        x *= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x = BigInt::from(20100000100);
        let y = BigInt::from(0);
        let z = x * y;
        x *= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x = BigInt::from(10);
        let y = BigInt::from(-10);
        let z = x * y;
        x *= y;
        assert_eq!(z, -100);
        assert!(x != y);

        let mut x = BigInt::from(11);
        let y = BigInt::from(10);
        let z = x * y;
        x *= y;
        assert_eq!(z, 110);
        assert!(x != y);

        let mut x = BigInt::from(-0);
        let y = BigInt::from(0);
        let z = x * y;
        x *= y;
        assert_eq!(z, 0);
        assert!(x == y);

        let mut x = BigInt::from(6);
        let y = BigInt::from(4);
        let z = x * y;
        x *= y;
        assert_eq!(z, 24);
        assert!(x == y);

        let mut x = BigInt::from(-15);
        let y = BigInt::from(-4);
        let z = x * y;
        x *= y;
        assert_eq!(z, 60);
        assert!(x == y);
    }

    #[test]
    fn div() {
        let mut x: BigInt = BigInt::from(10000);
        let y = BigInt::from(10);
        let z = x / y;
        x /= y;
        assert_eq!(z, 1000);
        assert_eq!(z, x);

        let mut x = BigInt::from(101);
        let y = BigInt::from(101);
        let z = x / y;
        x /= y;
        assert_eq!(z, 1);
        assert_eq!(z, x);

        let mut x = BigInt::from(0);
        let y = BigInt::from(2);
        let z = x / y;
        x /= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x = BigInt::from(0);
        let y = BigInt::from(0);
        let z = x / y;
        x /= y;
        assert_eq!(z, BigInt::default());
        assert_eq!(z, x);

        let mut x = BigInt::from(10000);
        let y = BigInt::from(0);
        let z = x / y;
        x /= y;
        assert_eq!(z, BigInt::default());
        assert_eq!(z, x);

        let mut x = BigInt::from(20100000100);
        let y = BigInt::from(200);
        let z = x / y;
        x /= y;
        assert_eq!(z, 100500);
        assert_eq!(z, x);

        let mut x = BigInt::from(10);
        let y = BigInt::from(-10);
        let z = x / y;
        x /= y;
        assert_eq!(z, -1);
        assert!(x != y);

        let mut x = BigInt::from(110);
        let y = BigInt::from(10);
        let z = x / y;
        x /= y;
        assert_eq!(z, 11);
        assert!(x != y);

        let mut x = BigInt::from(6);
        let y = BigInt::from(2);
        let z = x / y;
        x /= y;
        assert_eq!(z, 3);
        assert!(x == y);

        let mut x = BigInt::from(-15);
        let y = BigInt::from(-5);
        let z = x / y;
        x /= y;
        assert_eq!(z, 3);
        assert!(x == y);
    }

    #[test]
    fn reminder() {
        let mut x: BigInt = BigInt::from(10000);
        let y = BigInt::from(10);
        let z = x % y;
        x %= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x: BigInt = BigInt::from(10);
        let y = BigInt::from(0);
        let z = x % y;
        x %= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x: BigInt = BigInt::from(10);
        let y = BigInt::from(7);
        let z = x % y;
        x %= y;
        assert_eq!(z, 3);
        assert_eq!(z, x);

        let mut x: BigInt = BigInt::from(10000);
        let y = BigInt::from(10);
        let z = x % y;
        x %= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x: BigInt = BigInt::from(-104);
        let y = BigInt::from(10);
        let z = x % y;
        x %= y;
        assert_eq!(z, 4);
        assert_eq!(z, x);

        let mut x: BigInt = BigInt::from(24);
        let y = BigInt::from(3);
        let z = x % y;
        x %= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x: BigInt = BigInt::from(33);
        let y = BigInt::from(7);
        let z = x % y;
        x %= y;
        assert_eq!(z, 5);
        assert_eq!(z, x);
    }

    #[test]
    fn binary() {
        let x = BigInt::from(4);
        assert_eq!(format!("{x:b}"), "100");
        let x = BigInt::from(4);
        assert_eq!(format!("{x:#b}"), "0b100");
        let x = BigInt::from(4);
        assert_eq!(format!("{x:010b}"), "0000000100");
        let x = BigInt::from(10);
        assert_eq!(format!("{x:#110b}"), "0b11111010");
        let x = BigInt::from(172);
        assert_eq!(format!("{x:b}"), "10101100");
        let x = BigInt::from(17220003931);
        assert_eq!(format!("{x:#b}"), "0b10000000010011001000110100001011011");
    }

    #[test]
    fn bit_and() {
        let mut x = BigInt::from(4); //100
        let y = BigInt::from(12); //1100
        let z = x & y; //0100
        x &= y;
        assert_eq!(x, z);
        assert_eq!(z, 4);

        let mut x = BigInt::from(10); //1010
        let y = BigInt::from(13); //1101
        let z = x & y; //1000
        x &= y;
        assert_eq!(x, z);
        assert_eq!(z, 8);

        let mut x = BigInt::from(172); //10101100
        let y = BigInt::from(223); //11011111
        let z = x & y; //10001100
        x &= y;
        assert_eq!(x, z);
        assert_eq!(z, 140);

        let mut x = BigInt::from(172); //10101100
        let y = BigInt::from(1); //1
        let z = x & y; //0
        x &= y;
        assert_eq!(x, z);
        assert_eq!(z, 0);

        let mut x = BigInt::from(173); //10101101
        let y = BigInt::from(1); //1
        let z = x & y; //1
        x &= y;
        assert_eq!(x, z);
        assert_eq!(z, 1);
    }

    #[test]
    fn bit_or() {
        let mut x = BigInt::from(4); //100
        let y = BigInt::from(12); //1100
        let z = x | y; //1100
        x |= y;
        assert_eq!(x, z);
        assert_eq!(z, 12);

        let mut x = BigInt::from(10); //1010
        let y = BigInt::from(13); //1101
        let z = x | y; //1111
        x |= y;
        assert_eq!(x, z);
        assert_eq!(z, 15);

        let mut x = BigInt::from(172); //10101100
        let y = BigInt::from(223); //11011111
        let z = x | y; //11111111
        x |= y;
        assert_eq!(x, z);
        assert_eq!(z, 255);

        let mut x = BigInt::from(172); //10101100
        let y = BigInt::from(1); //1
        let z = x | y; //10101101
        x |= y;
        assert_eq!(x, z);
        assert_eq!(z, 173);

        let mut x = BigInt::from(173); //10101101
        let y = BigInt::from(1); //1
        let z = x | y; //10101101
        x |= y;
        assert_eq!(x, z);
        assert_eq!(z, 173);
    }

    #[test]
    fn bit_xor() {
        let mut x = BigInt::from(4); //100
        let y = BigInt::from(12); //1100
        let z = x ^ y; //1000
        x ^= y;
        assert_eq!(x, z);
        assert_eq!(z, 8);

        let mut x = BigInt::from(10); //1010
        let y = BigInt::from(13); //1101
        let z = x ^ y; //0111
        x ^= y;
        assert_eq!(x, z);
        assert_eq!(z, 7);

        let mut x = BigInt::from(172); //10101100
        let y = BigInt::from(223); //11011111
        let z = x ^ y; //01110011
        x ^= y;
        assert_eq!(x, z);
        assert_eq!(z, 115);

        let mut x = BigInt::from(172); //10101100
        let y = BigInt::from(1); //1
        let z = x ^ y; //10101101
        x ^= y;
        assert_eq!(x, z);
        assert_eq!(z, 173);

        let mut x = BigInt::from(173); //10101101
        let y = BigInt::from(1); //1
        let z = x ^ y; //10101100
        x ^= y;
        assert_eq!(x, z);
        assert_eq!(z, 172);
    }

    #[test]
    fn bit_shift_left() {
        let mut x = BigInt::from(4); //100
        let y = BigInt::from(2);
        let z = x << y; //10000
        x <<= y;
        assert_eq!(x, z);
        assert_eq!(z, 16);

        let mut x = BigInt::from(10); //1010
        let y = BigInt::from(1);
        let z = x << y; //10100
        x <<= y;
        assert_eq!(x, z);
        assert_eq!(z, 20);

        let mut x = BigInt::from(172); //10101100
        let y = BigInt::from(0);
        let z = x << y;
        x <<= y;
        assert_eq!(x, z);
        assert_eq!(z, 172);

        let mut x = BigInt::from(172); //10101100
        let y = BigInt::from(10);
        let z = x << y; //101011000000000000
        x <<= y;
        assert_eq!(x, z);
        assert_eq!(z, 176128);

        let mut x = BigInt::from(173); //10101101
        let y = BigInt::from(1);
        let z = x << y; //101011010
        x <<= y;
        assert_eq!(x, z);
        assert_eq!(z, 346);
    }

    #[test]
    fn bit_shift_right() {
        let mut x = BigInt::from(4); //100
        let y = BigInt::from(2);
        let z = x >> y; //1
        x >>= y;
        assert_eq!(x, z);
        assert_eq!(z, 1);

        let mut x = BigInt::from(10); //1010
        let y = BigInt::from(1);
        let z = x >> y; //101
        x >>= y;
        assert_eq!(x, z);
        assert_eq!(z, 5);

        let mut x = BigInt::from(172); //10101100
        let y = BigInt::from(0);
        let z = x >> y; //10101100
        x >>= y;
        assert_eq!(x, z);
        assert_eq!(z, 172);

        let mut x = BigInt::from(172); //10101100
        let y = BigInt::from(10);
        let z = x >> y; //0
        x >>= y;
        assert_eq!(x, z);
        assert_eq!(z, 0);

        let mut x = BigInt::from(173); //10101101
        let y = BigInt::from(1);
        let z = x >> y; //1010110
        x >>= y;
        assert_eq!(x, z);
        assert_eq!(z, 86);
    }
}

fn main() {}

use std::str::FromStr;

use num_traits::Pow;

use crate::BigInt;
use crate::BigIntError;

#[test]
fn default() {
    let def = BigInt::default();
    assert!(def.positive);
    assert_eq!(def.numbers, [0].to_vec());
}

#[test]
fn new() {
    let new = BigInt::new();
    assert!(new.positive);
    assert_eq!(new.numbers, [0].to_vec());
}

#[test]
fn from() {
    let x = BigInt::from(20);
    assert!(x.positive);
    assert_eq!(x.numbers, [2, 0].to_vec());
    let x = BigInt::from(-20);
    assert!(!x.positive);
    assert_eq!(x.numbers, [2, 0].to_vec());
    let x = BigInt::from(-320020000981234567890_i128);
    assert!(!x.positive);
    assert_eq!(
        x.numbers,
        [3, 2, 0, 0, 2, 0, 0, 0, 0, 9, 8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0].to_vec()
    );
    let x = BigInt::from(0);
    assert!(x.positive);
    assert_eq!(x.numbers, [0].to_vec());
    let x = BigInt::from(-0);
    assert!(x.positive);
    assert_eq!(x.numbers, [0].to_vec());
}

#[test]
fn from_string_numbers() {
    let x = BigInt::from_str("20").unwrap();
    assert!(x.positive);
    assert_eq!(x.numbers, [2, 0].to_vec());
    let x = BigInt::from_str("666").unwrap();
    assert!(x.positive);
    assert_eq!(x.numbers, [6, 6, 6].to_vec());
    let x = BigInt::from_str("-20").unwrap();
    assert!(!x.positive);
    assert_eq!(x.numbers, [2, 0].to_vec());
    let x = BigInt::from_str("-320020000981234567890").unwrap();
    assert!(!x.positive);
    assert_eq!(
        x.numbers,
        [3, 2, 0, 0, 2, 0, 0, 0, 0, 9, 8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0].to_vec()
    );
    let x = BigInt::from_str("-0").unwrap();
    assert!(x.positive);
    assert_eq!(x.numbers, [0].to_vec());
    let x = BigInt::from_str("0sw");
    assert!(x.is_err());
    let x = BigInt::from_str("0 2020000");
    assert!(x.is_err());
    let x = BigInt::from_str("--200");
    assert!(x.is_err());
    let x = BigInt::from_str("+2000303");
    assert!(x.is_err());
    let x = BigInt::from_str("minus20003002");
    assert!(x.is_err());
}

#[test]
fn from_string_words_from_str_digits() {
    let x = BigInt::from_str("two zero     ").unwrap();
    assert!(x.positive);
    assert_eq!(x.numbers, [2, 0].to_vec());
    let x = BigInt::from_str("minus two four").unwrap();
    assert!(!x.positive);
    assert_eq!(x.numbers, [2, 4].to_vec());
    let x = BigInt::from_str("two five five zero zero two one").unwrap();
    assert!(x.positive);
    assert_eq!(x.numbers, [2, 5, 5, 0, 0, 2, 1].to_vec());
    let x = BigInt::from_str("minus two     zero zero zero zero zero one").unwrap();
    assert!(!x.positive);
    assert_eq!(x.numbers, [2, 0, 0, 0, 0, 0, 1].to_vec());
    let x = BigInt::from_str("zero").unwrap();
    assert!(x.positive);
    assert_eq!(x.numbers, [0].to_vec());
    let x = BigInt::from_str("onse");
    assert!(x.is_err());
    let x = BigInt::from_str("        ");
    assert!(x.is_err());
    let x = BigInt::from_str("twenty thousand thousand");
    assert!(x.is_err());
    let x: Result<BigInt, BigIntError> = BigInt::from_str("twenty thousand hundred");
    assert!(x.is_err());
    let x = BigInt::from_str("- five four").unwrap();
    assert!(!x.positive);
    assert_eq!(x.numbers, [5, 4].to_vec());
}

#[test]
fn try_into() {
    let x = BigInt::from(666);
    assert_eq!(666_i128, x.try_into().unwrap());
    let x = BigInt::from(666);
    assert_eq!(666_u128, x.try_into().unwrap());
    let x = BigInt::from(-123);
    assert_eq!(-123_i128, x.try_into().unwrap());
    let x = BigInt::from(-123);
    assert_eq!(123_u128, x.try_into().unwrap());
    let x = BigInt::from(0);
    assert_eq!(0_i128, x.try_into().unwrap());
    let x = BigInt::from(0);
    assert_eq!(0_u128, x.try_into().unwrap());
    let x = BigInt::from(10);
    assert_eq!(10_i128, x.try_into().unwrap());
    let x = BigInt::from(10);
    assert_eq!(10_u128, x.try_into().unwrap());
    let x = BigInt::from(-123);
    assert_eq!(-123_i8, x.try_into().unwrap());
    let x = BigInt::from(-123);
    assert_eq!(123_u8, x.try_into().unwrap());
    let x = BigInt::from(-123);
    assert_eq!(-123_i16, x.try_into().unwrap());
    let x = BigInt::from(-123);
    assert_eq!(123_u32, x.try_into().unwrap());
    let x = BigInt::from(-123);
    assert_eq!(-123_i64, x.try_into().unwrap());
    let x = BigInt::from(-123);
    assert_eq!(123_u64, x.try_into().unwrap());
}

#[test]
#[should_panic]
fn try_into_panic_i128() {
    let x = BigInt::from_str("999999999999999999999999999999999999999999999999999999999").unwrap();
    assert_eq!(666_i128, x.try_into().unwrap());
}

#[test]
#[should_panic]
fn try_into_panic_u128() {
    let x = BigInt::from_str("999999999999999999999999999999999999999999999999999999999").unwrap();
    assert_eq!(666_u128, x.try_into().unwrap());
}

#[test]
fn display() {
    let x = BigInt::from(1003);
    assert_eq!(format!("Number is: {x:0^10}"), "Number is: 0001003000");
    let x = BigInt::from(1003);
    assert_eq!(format!("Number is: {x:0^11}"), "Number is: 00010030000");
    let x = BigInt::from(1003);
    assert_eq!(format!("Number is: {x:0>10}"), "Number is: 0000001003");
    let x = BigInt::from(1003);
    assert_eq!(format!("Number is: {x:0<10}"), "Number is: 1003000000");
    let x = BigInt::from(1003);
    assert_eq!(format!("Number is: {x}"), "Number is: 1003");
    let x = BigInt::from(0);
    assert_eq!(format!("Number is: {x}"), "Number is: 0");
    let x = BigInt::from(-100);
    assert_eq!(format!("Number is: {x}"), "Number is: -100");
    let x = BigInt::from(-0);
    assert_eq!(format!("Number is: {x}"), "Number is: 0");
    let x = BigInt::from(-1);
    assert_eq!(format!("Number is: {x}"), "Number is: -1");
    let x = BigInt::from(320020000981234567890_i128);
    assert_eq!(
        format!("Number is: {x}"),
        "Number is: 320020000981234567890"
    );
    let x = BigInt::from(1003);
    assert_eq!(format!("Number is: {x:->10}"), "Number is: ------1003");
    let x = BigInt::from(1003);
    assert_eq!(format!("Number is: {x:9}"), "Number is:      1003");
    let x = BigInt::from(0);
    assert_eq!(format!("Number is: {x:2}"), "Number is:  0");
    let x = BigInt::from(-100);
    assert_eq!(format!("Number is: {x}"), "Number is: -100");
    let x = BigInt::from(-0);
    assert_eq!(format!("Number is: {x}"), "Number is: 0");
    let x = BigInt::from(-100);
    assert_eq!(format!("Number is: {x:X>5}"), "Number is: X-100");
    let x = BigInt::from(-0);
    assert_eq!(format!("Number is: {x:+}"), "Number is: +0");
}

#[test]
fn to_words() {
    let x = BigInt::from(20);
    assert_eq!(x.to_words(), "two zero");
    let x = BigInt::from(-24);
    assert_eq!(x.to_words(), "minus two four");
    let x = BigInt::from(2550021);
    assert_eq!(x.to_words(), "two five five zero zero two one");
    let x = BigInt::from(-2000000000001_i128);
    assert_eq!(
        x.to_words(),
        "minus two zero zero zero zero zero zero zero zero zero zero zero one"
    );
}

#[test]
fn not() {
    let mut x = BigInt::from(1);
    assert!(x.positive);
    assert_eq!(x.to_string(), "1");
    x = -x;
    assert!(!x.positive);
    assert_eq!(x.to_string(), "-1");
    x = -x;
    assert!(x.positive);
    assert_eq!(x.to_string(), "1");
    x = BigInt::from(-22);
    assert!(!x.positive);
    assert_eq!(x.to_string(), "-22");
    x = -x;
    assert!(x.positive);
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
    let x = BigInt::from(101010);
    let y = BigInt::from(101210);
    assert!(!(x == y));
    let x = BigInt::from(101010);
    let y = BigInt::from(1);
    assert!(!(x == y));
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
fn equal_int() {
    let x = BigInt::from(10);
    assert!(x == 10);
    let x = BigInt::from(101010);
    assert!(x == 101010);
    let x = BigInt::from(101010);
    assert!(!(x == 101210));
    let x = BigInt::from(101010);
    assert!(!(x == 1));
    let x = BigInt::from(0);
    assert!(x == 0);
    let x = BigInt::from(10);
    assert!(x != -10);
    let x = BigInt::from(11);
    assert!(x != 10);
    let x = BigInt::from(-0);
    assert!(x == 0);
}

#[test]
fn equal_str() {
    let x = BigInt::from(10);
    assert!(x == "10");
    let x = BigInt::from(101010);
    assert!(x == "101010");
    let x = BigInt::from(101010);
    assert!(!(x == "101210"));
    let x = BigInt::from(101010);
    assert!(!(x == "1"));
    let x = BigInt::from(0);
    assert!(x == "0");
    let x = BigInt::from(10);
    assert!(x != "-10");
    let x = BigInt::from(11);
    assert!(x != "10");
    let x = BigInt::from(-0);
    assert!(x == "0");
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
    assert!(x <= y);
    let x = BigInt::from(10);
    let y = BigInt::from(10);
    assert!(x >= y);
    let x = BigInt::from(101010);
    let y = BigInt::from(101010);
    assert!(x >= y);
    let x = BigInt::from(0);
    let y = BigInt::from(0);
    assert!(x >= y);
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
    assert!(x >= y);
    let x = BigInt::from(10);
    let y = BigInt::from(10);
    assert!(x <= y);
    let x = BigInt::from(99999999999_i64);
    let y = BigInt::from(99999999999_i128);
    assert!(x <= y);
    let x = BigInt::from(0);
    let y = BigInt::from(0);
    assert!(x <= y);
    let x = BigInt::from(-10);
    let y = BigInt::from(10);
    assert!(x < y);
    let x = BigInt::from(11);
    let y = BigInt::from(99999999999_u64);
    assert!(x < y);
    let x = BigInt::from(10000010);
    let y = BigInt::from(20000000);
    assert!(x < y);
}

#[test]
fn add() {
    let mut x: BigInt = BigInt::from(1000);
    let y = BigInt::from(10);
    let z = x.clone() + y.clone();
    x += y;
    assert_eq!(z, 1010);
    assert_eq!(x, z);

    let mut x: BigInt = BigInt::from(10);
    let y = BigInt::from(10);
    let z = x.clone() + y.clone();
    x += y;
    assert_eq!(z, 20);
    assert_eq!(x, z);

    let mut x = BigInt::from(101010);
    let y = BigInt::from(101010);
    let z = x.clone() + y.clone();
    x += y;
    assert_eq!(z, 202020);
    assert_eq!(x, z);

    let mut x = BigInt::from(0);
    let y = BigInt::from(0);
    let z = x.clone() + y.clone();
    x += y;
    assert_eq!(z, 0);
    assert_eq!(x, z);

    let mut x = BigInt::from(10);
    let y = BigInt::from(-10);
    let z = x.clone() + y.clone();
    x += y.clone();
    assert_eq!(z, 0);
    assert_ne!(x, y);

    let mut x = BigInt::from(11);
    let y = BigInt::from(10);
    let z = x.clone() + y.clone();
    x += y.clone();
    assert_eq!(z, 21);
    assert_ne!(x, y);

    let mut x = BigInt::from(-0);
    let y = BigInt::from(0);
    let z = x.clone() + y.clone();
    x += y.clone();
    assert_eq!(z, 0);
    assert_eq!(x, z);

    let mut x = BigInt::from(6);
    let y = BigInt::from(4);
    let z = x.clone() + y.clone();
    x += y.clone();
    assert_eq!(z, 10);
    assert_eq!(x, z);

    let mut x = BigInt::from(-15);
    let y = BigInt::from(-4);
    let z = x.clone() + y.clone();
    x += y.clone();
    assert_eq!(z, -19);
    assert_eq!(x, z);
}

#[test]
fn sub() {
    let mut x: BigInt = BigInt::from(10);
    let y = BigInt::from(10);
    let z = x.clone() - y.clone();
    x -= y;
    assert_eq!(z, 0);
    assert_eq!(x, z);

    let mut x = BigInt::from(101010);
    let y = BigInt::from(10);
    let z = x.clone() - y.clone();
    x -= y;
    assert_eq!(z, 101000);
    assert_eq!(x, z);

    let mut x = BigInt::from(0);
    let y = BigInt::from(0);
    let z = x.clone() - y.clone();
    x -= y;
    assert_eq!(z, 0);
    assert_eq!(x, z);

    let mut x = BigInt::from(10);
    let y = BigInt::from(-10);
    let z = x.clone() - y.clone();
    x -= y.clone();
    assert_eq!(z, 20);
    assert_ne!(x, y);

    let mut x = BigInt::from(-10);
    let y = BigInt::from(10);
    let z = x.clone() - y.clone();
    x -= y.clone();
    assert_eq!(z, -20);
    assert_ne!(x, y);

    let mut x = BigInt::from(11);
    let y = BigInt::from(10);
    let z = x.clone() - y.clone();
    x -= y.clone();
    assert_eq!(z, 1);
    assert_ne!(x, y);

    let mut x = BigInt::from(-0);
    let y = BigInt::from(0);
    let z = x.clone() - y.clone();
    x -= y.clone();
    assert_eq!(z, 0);
    assert_eq!(x, z);

    let mut x = BigInt::from(6);
    let y = BigInt::from(4);
    let z = x.clone() - y.clone();
    x -= y.clone();
    assert_eq!(z, 2);
    assert_eq!(x, z);

    let mut x = BigInt::from(4);
    let y = BigInt::from(15);
    let z = x.clone() - y.clone();
    x -= y.clone();
    assert_eq!(z, -11);
    assert_eq!(x, z);

    let mut x = BigInt::from(-15);
    let y = BigInt::from(-4);
    let z = x.clone() - y.clone();
    x -= y.clone();
    assert_eq!(z, -11);
    assert_eq!(x, z);

    let mut x = BigInt::from(987654);
    let y = BigInt::from(987653);
    let z = x.clone() - y.clone();
    x -= y.clone();
    assert_eq!(z, 1);
    assert_eq!(x, z);

    let mut x = BigInt::from(1);
    let y = BigInt::from(1000);
    let z = x.clone() - y.clone();
    x -= y.clone();
    assert_eq!(z, -999);
    assert_eq!(x, z);
}

#[test]
fn mul() {
    let mut x: BigInt = BigInt::from(10);
    let y = BigInt::from(10);
    let z: BigInt = x.clone() * y.clone();
    x *= y;
    assert_eq!(z, 100);
    assert_eq!(z, x);

    let mut x = BigInt::from(101);
    let y = BigInt::from(101);
    let z = x.clone() * y.clone();
    x *= y;
    assert_eq!(z, 10201);
    assert_eq!(z, x);

    let mut x = BigInt::from(0);
    let y = BigInt::from(0);
    let z: BigInt = x.clone() * y.clone();
    x *= y;
    assert_eq!(z, 0);
    assert_eq!(z, x);

    let mut x = BigInt::from(20100000100_u64);
    let y = BigInt::from(0);
    let z: BigInt = x.clone() * y.clone();
    x *= y;
    assert_eq!(z, 0);
    assert_eq!(z, x);

    let mut x = BigInt::from(10);
    let y = BigInt::from(-10);
    let z: BigInt = x.clone() * y.clone();
    x *= y.clone();
    assert_eq!(z, -100);
    assert_ne!(x, y);

    let mut x = BigInt::from(11);
    let y = BigInt::from(10);
    let z: BigInt = x.clone() * y.clone();
    x *= y.clone();
    assert_eq!(z, 110);
    assert_ne!(x, y);

    let mut x = BigInt::from(-0);
    let y = BigInt::from(0);
    let z: BigInt = x.clone() * y.clone();
    x *= y.clone();
    assert_eq!(z, 0);
    assert_eq!(x, y);

    let mut x = BigInt::from(6);
    let y = BigInt::from(4);
    let z: BigInt = x.clone() * y.clone();
    x *= y.clone();
    assert_eq!(z, 24);
    assert_eq!(x, z);

    let mut x = BigInt::from(-15);
    let y = BigInt::from(-4);
    let z: BigInt = x.clone() * y.clone();
    x *= y.clone();
    assert_eq!(z, 60);
    assert_eq!(x, z);
}

#[test]
fn div() {
    let mut x: BigInt = BigInt::from(10000000);
    let y = BigInt::from(10);
    let z = x.clone() / y.clone();
    x /= y;
    assert_eq!(z, 1000000);
    assert_eq!(z, x);

    let mut x = BigInt::from(101);
    let y = BigInt::from(101);
    let z = x.clone() / y.clone();
    x /= y;
    assert_eq!(z, 1);
    assert_eq!(z, x);

    let mut x = BigInt::from(9999_u128);
    let y = BigInt::from(2);
    let z = x.clone() / y.clone();
    x /= y;
    assert_eq!(z, 4999_u128);
    assert_eq!(z, x);

    let mut x = BigInt::from_str("999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999").unwrap();
    let y = BigInt::from(2);
    let z = x.clone() / y.clone();
    x /= y;
    assert_eq!(
        z,
        "499999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999999"
    );
    assert_eq!(z, x);

    let mut x = BigInt::from(0);
    let y = BigInt::from(2);
    let z = x.clone() / y.clone();
    x /= y;
    assert_eq!(z, 0);
    assert_eq!(z, x);

    let mut x = BigInt::from(1);
    let y = BigInt::from(2);
    let z = x.clone() / y.clone();
    x /= y;
    assert_eq!(z, 0);
    assert_eq!(z, x);

    let mut x = BigInt::from(1);
    let y = BigInt::from(1);
    let z = x.clone() / y.clone();
    x /= y;
    assert_eq!(z, 1);
    assert_eq!(z, x);

    let mut x = BigInt::from(0);
    let y = BigInt::from(666);
    let z = x.clone() / y.clone();
    x /= y;
    assert_eq!(z, 0);
    assert_eq!(z, x);

    let mut x = BigInt::from(0);
    let y = BigInt::from(100);
    let z = x.clone() / y.clone();
    x /= y;
    assert_eq!(z, 0);
    assert_eq!(z, x);

    let mut x = BigInt::from(20100000100_u64);
    let y = BigInt::from(20000000);
    let z = x.clone() / y.clone();
    x /= y;
    assert_eq!(z, 1005);
    assert_eq!(z, x);

    let mut x = BigInt::from(10);
    let y = BigInt::from(-10);
    let z = x.clone() / y.clone();
    x /= y.clone();
    assert_eq!(z, -1);
    assert_ne!(x, y);

    let mut x = BigInt::from(110);
    let y = BigInt::from(10);
    let z = x.clone() / y.clone();
    x /= y.clone();
    assert_eq!(z, 11);
    assert_ne!(x, y);

    let mut x = BigInt::from(6);
    let y = BigInt::from(2);
    let z = x.clone() / y.clone();
    x /= y.clone();
    assert_eq!(z, 3);
    assert_eq!(x, z);

    let mut x = BigInt::from(-15);
    let y = BigInt::from(-5);
    let z = x.clone() / y.clone();
    x /= y.clone();
    assert_eq!(z, 3);
    assert_eq!(x, z);
}

#[test]
#[should_panic]
fn div_by_zero() {
    let mut x = BigInt::from(10);
    let y = BigInt::from(0);
    x /= y;
}

#[test]
fn remainder() {
    let mut x: BigInt = BigInt::from(10000);
    let y = BigInt::from(10);
    let z = x.clone() % y.clone();
    x %= y;
    assert_eq!(z, 0);
    assert_eq!(z, x);

    let mut x: BigInt = BigInt::from(10);
    let y = BigInt::from(7);
    let z = x.clone() % y.clone();
    x %= y;
    assert_eq!(z, 3);
    assert_eq!(z, x);

    let mut x: BigInt = BigInt::from(10000);
    let y = BigInt::from(10);
    let z = x.clone() % y.clone();
    x %= y;
    assert_eq!(z, 0);
    assert_eq!(z, x);

    let mut x: BigInt = BigInt::from(-104);
    let y = BigInt::from(10);
    let z = x.clone() % y.clone();
    x %= y;
    assert_eq!(z, -4);
    assert_eq!(z, x);

    let mut x: BigInt = BigInt::from(24);
    let y = BigInt::from(3);
    let z = x.clone() % y.clone();
    x %= y;
    assert_eq!(z, 0);
    assert_eq!(z, x);

    let mut x: BigInt = BigInt::from(33);
    let y = BigInt::from(7);
    let z = x.clone() % y.clone();
    x %= y;
    assert_eq!(z, 5);
    assert_eq!(z, x);
}

#[test]
fn power_of() {
    let x: BigInt = BigInt::from(2);
    let y = BigInt::from(2);
    let z = x.pow(y);
    assert_eq!(z, 4);

    let x: BigInt = BigInt::from(2);
    let y = BigInt::from(1);
    let z = x.pow(y);
    assert_eq!(z, 2);

    let x: BigInt = BigInt::from(1);
    let y = BigInt::from(1000000);
    let z = x.pow(y);
    assert_eq!(z, 1);

    let x: BigInt = BigInt::from(3);
    let y = BigInt::from(3);
    let z = x.pow(y);
    assert_eq!(z, 27);

    let x: BigInt = BigInt::from(-3);
    let y = BigInt::from(3);
    let z = x.pow(y);
    assert_eq!(z, -27);

    let x: BigInt = BigInt::from(-3);
    let y = BigInt::from(2);
    let z = x.pow(y);
    assert_eq!(z, 9);

    let x: BigInt = BigInt::from(2);
    let y = BigInt::from(-1);
    let z = x.pow(y);
    assert_eq!(z, 0);

    let x: BigInt = BigInt::from(1);
    let y = BigInt::from(-10);
    let z = x.pow(y);
    assert_eq!(z, 1);

    let x: BigInt = BigInt::from(5);
    let y = BigInt::from(-10);
    let z = x.pow(y);
    assert_eq!(z, 0);
}

#[test]
#[should_panic]
fn rem_of_zero() {
    let mut x: BigInt = BigInt::from(10);
    let y = BigInt::from(0);
    let z = x.clone() % y.clone();
    x %= y;
    assert_eq!(z, 0);
    assert_eq!(z, x);
}

#[test]
fn binary() {
    let x = BigInt::from(11);
    assert_eq!(format!("{x:#10b}"), "    0b1011");
    let x = BigInt::from(4);
    assert_eq!(format!("{x:#b}"), "0b100");
    let x = BigInt::from(4);
    assert_eq!(format!("{x:b}"), "100");
    let x = BigInt::from(4);
    assert_eq!(format!("{x:0>10b}"), "0000000100");
    let x = BigInt::from(10);
    assert_eq!(format!("{x:#10b}"), "    0b1010");
    let x = BigInt::from(10);
    assert_eq!(format!("{x:x>#010b}"), "xxxx0b1010");
    let x = BigInt::from(172);
    assert_eq!(format!("{x:b}"), "10101100");
    let x = BigInt::from(-17220003931_i64);
    assert_eq!(format!("{x:#b}"), "-0b10000000010011001000110100001011011");
}

#[test]
fn binary_transformation() {
    let x: BigInt = BigInt::from(10);
    let (positive, binary) = x.to_binary();
    assert_eq!(x, BigInt::from_binary(positive, binary));

    let x: BigInt = BigInt::from(-42);
    let (positive, binary) = x.to_binary();
    assert_eq!(x, BigInt::from_binary(positive, binary));

    let x: BigInt = BigInt::from(-0);
    let (positive, binary) = x.to_binary();
    assert_eq!(x, BigInt::from_binary(positive, binary));

    let x: BigInt = BigInt::from(0);
    let (positive, binary) = x.to_binary();
    assert_eq!(x, BigInt::from_binary(positive, binary));

    let x: BigInt = BigInt::from(-9999);
    let (positive, binary) = x.to_binary();
    assert_eq!(x, BigInt::from_binary(positive, binary));

    let x: BigInt = BigInt::from(666);
    let (positive, binary) = x.to_binary();
    assert_eq!(x, BigInt::from_binary(positive, binary));
}

#[test]
fn bit_and() {
    let mut x = BigInt::from(4); //100
    let y = BigInt::from(12); //1100
    let z = x.clone() & y.clone(); //0100
    x &= y;
    assert_eq!(x, z);
    assert_eq!(z, 4);

    let mut x = BigInt::from(10); //1010
    let y = BigInt::from(13); //1101
    let z = x.clone() & y.clone(); //1000
    x &= y;
    assert_eq!(x, z);
    assert_eq!(z, 8);

    let mut x = BigInt::from(172); //10101100
    let y = BigInt::from(223); //11011111
    let z = x.clone() & y.clone(); //10001100
    x &= y;
    assert_eq!(x, z);
    assert_eq!(z, 140);

    let mut x = BigInt::from(172); //10101100
    let y = BigInt::from(1); //1
    let z = x.clone() & y.clone(); //0
    x &= y;
    assert_eq!(x, z);
    assert_eq!(z, 0);

    let mut x = BigInt::from(173); //10101101
    let y = BigInt::from(1); //1
    let z = x.clone() & y.clone(); //1
    x &= y;
    assert_eq!(x, z);
    assert_eq!(z, 1);
}

#[test]
fn bit_or() {
    let mut x = BigInt::from(4); //100
    let y = BigInt::from(12); //1100
    let z = x.clone() | y.clone(); //1100
    x |= y;
    assert_eq!(x, z);
    assert_eq!(z, 12);

    let mut x = BigInt::from(10); //1010
    let y = BigInt::from(13); //1101
    let z = x.clone() | y.clone(); //1111
    x |= y;
    assert_eq!(x, z);
    assert_eq!(z, 15);

    let mut x = BigInt::from(172); //10101100
    let y = BigInt::from(223); //11011111
    let z = x.clone() | y.clone(); //11111111
    x |= y;
    assert_eq!(x, z);
    assert_eq!(z, 255);

    let mut x = BigInt::from(172); //10101100
    let y = BigInt::from(1); //1
    let z = x.clone() | y.clone(); //10101101
    x |= y;
    assert_eq!(x, z);
    assert_eq!(z, 173);

    let mut x = BigInt::from(173); //10101101
    let y = BigInt::from(1); //1
    let z = x.clone() | y.clone(); //10101101
    x |= y;
    assert_eq!(x, z);
    assert_eq!(z, 173);
}

#[test]
fn bit_xor() {
    let mut x = BigInt::from(4); //100
    let y = BigInt::from(12); //1100
    let z = x.clone() ^ y.clone(); //1000
    x ^= y;
    assert_eq!(x, z);
    assert_eq!(z, -8);

    let mut x = BigInt::from(10); //1010
    let y = BigInt::from(13); //1101
    let z = x.clone() ^ y.clone(); //0111
    x ^= y;
    assert_eq!(x, z);
    assert_eq!(z, -7);

    let mut x = BigInt::from(-172); //10101100
    let y = BigInt::from(-223); //11011111
    let z = x.clone() ^ y.clone(); //01110011
    x ^= y;
    assert_eq!(x, z);
    assert_eq!(z, -115);

    let mut x = BigInt::from(172); //10101100
    let y = BigInt::from(-1); //1
    let z = x.clone() ^ y.clone(); //10101101
    x ^= y;
    assert_eq!(x, z);
    assert_eq!(z, 173);

    let mut x = BigInt::from(-173); //10101101
    let y = BigInt::from(1); //1
    let z = x.clone() ^ y.clone(); //10101100
    x ^= y;
    assert_eq!(x, z);
    assert_eq!(z, 172);
}

#[test]
fn bit_shift_left() {
    let mut x = BigInt::from(4); //100
    let y = BigInt::from(2);
    let z = x.clone() << y.clone(); //10000
    x <<= y;
    assert_eq!(x, z);
    assert_eq!(z, 16);

    let mut x = BigInt::from(10); //1010
    let y = BigInt::from(1);
    let z = x.clone() << y.clone(); //10100
    x <<= y;
    assert_eq!(x, z);
    assert_eq!(z, 20);

    let mut x = BigInt::from(172); //10101100
    let y = BigInt::from(0);
    let z = x.clone() << y.clone();
    x <<= y;
    assert_eq!(x, z);
    assert_eq!(z, 172);

    let mut x = BigInt::from(172); //10101100
    let y = BigInt::from(10);
    let z = x.clone() << y.clone(); //101011000000000000
    x <<= y;
    assert_eq!(x, z);
    assert_eq!(z, 176128);

    let mut x = BigInt::from(173); //10101101
    let y = BigInt::from(1);
    let z = x.clone() << y.clone(); //101011010
    x <<= y;
    assert_eq!(x, z);
    assert_eq!(z, 346);
}

#[test]
fn bit_shift_right() {
    let mut x = BigInt::from(4); //100
    let y = BigInt::from(2);
    let z = x.clone() >> y.clone(); //1
    x >>= y;
    assert_eq!(x, z);
    assert_eq!(z, 1);

    let mut x = BigInt::from(10); //1010
    let y = BigInt::from(1);
    let z = x.clone() >> y.clone(); //101
    x >>= y;
    assert_eq!(x, z);
    assert_eq!(z, 5);

    let mut x = BigInt::from(172); //10101100
    let y = BigInt::from(0);
    let z = x.clone() >> y.clone(); //10101100
    x >>= y;
    assert_eq!(x, z);
    assert_eq!(z, 172);

    let mut x = BigInt::from(172); //10101100
    let y = BigInt::from(10);
    let z = x.clone() >> y.clone(); //0
    x >>= y;
    assert_eq!(x, z);
    assert_eq!(z, 0);

    let mut x = BigInt::from(173); //10101101
    let y = BigInt::from(1);
    let z = x.clone() >> y.clone(); //1010110
    x >>= y;
    assert_eq!(x, z);
    assert_eq!(z, 86);
}

#[test]
fn hexadecimal() {
    let x = BigInt::from(4);
    assert_eq!(format!("{x:X}"), "4");
    let x = BigInt::from(16);
    assert_eq!(format!("{x:#X}"), "0x10");
    let x = BigInt::from(-4);
    assert_eq!(format!("{x:X}"), "-4");
    let x = BigInt::from(-16);
    assert_eq!(format!("{x:#X}"), "-0x10");

    let x = BigInt::from(10);
    assert_eq!(format!("{x:#X}"), "0xA");
    let x = BigInt::from(172);
    assert_eq!(format!("{x:X}"), "AC");
    let x = BigInt::from(17220003931_u128);
    assert_eq!(format!("{x:#X}"), "0x40264685B");

    let x = BigInt::from(4);
    assert_eq!(format!("{x:x}"), "4");
    let x = BigInt::from(16);
    assert_eq!(format!("{x:#x}"), "0x10");
    let x = BigInt::from(10);
    assert_eq!(format!("{x:#x}"), "0xa");
    let x = BigInt::from(172);
    assert_eq!(format!("{x:x}"), "ac");
    let x = BigInt::from(17220003931_u128);
    assert_eq!(format!("{x:#x}"), "0x40264685b");

    let x = BigInt::from(11);
    assert_eq!(format!("{x:#10X}"), "       0xB");
    let x = BigInt::from(4);
    assert_eq!(format!("{x:#x}"), "0x4");
    let x = BigInt::from(4);
    assert_eq!(format!("{x:X}"), "4");
    let x = BigInt::from(4);
    assert_eq!(format!("{x:0>10x}"), "0000000004");
    let x = BigInt::from(10);
    assert_eq!(format!("{x:#10x}"), "       0xa");
    let x = BigInt::from(10);
    assert_eq!(format!("{x:x>#010X}"), "xxxxxxx0xA");
    let x = BigInt::from(172);
    assert_eq!(format!("{x:X}"), "AC");
    let x = BigInt::from(-17220003931_i64);
    assert_eq!(format!("{x:#x}"), "-0x40264685b");
}

#[test]
fn progtest_tests() {
    let mut a = BigInt::from(10);
    a += BigInt::from(20);
    assert_eq!(a, 30);
    a *= BigInt::from(5);
    assert_eq!(a, 150);
    let mut b = a.clone() + BigInt::from(3);
    assert_eq!(b, 153);
    b = a.clone() * BigInt::from(7);
    assert_eq!(b, 1050);
    assert_eq!(a, 150);
    assert_eq!(format!("{a:X}"), "96");

    a = BigInt::from(10);
    a += BigInt::from(-20);
    assert_eq!(a, -10);
    a *= BigInt::from(5);
    assert_eq!(a, -50);
    b = a.clone() + BigInt::from(73);
    assert_eq!(b, 23);
    b = a.clone() * BigInt::from(-7);
    assert_eq!(b, 350);
    assert_eq!(a, -50);
    assert_eq!(format!("{a:X}"), "-32");

    a = BigInt::from(12345678901234567890_i128);
    a += BigInt::from(-99999999999999999999_i128);
    assert_eq!(a, -87654321098765432109_i128);
    a *= BigInt::from(54321987654321987654_i128);
    assert_eq!(a, "-4761556948575111126880627366067073182286");
    a *= BigInt::from(0);
    assert_eq!(a, 0);
    a = BigInt::from(10);
    b = a.clone() + 400;
    assert_eq!(b, "410");
    b = a.clone() * BigInt::from_str("15").unwrap_or_default();
    assert_eq!(b, "150");
    assert_eq!(a, "10");
    assert_eq!(format!("{a:X}"), "A");

    b = BigInt::from_str("1234").unwrap_or_default();
    assert_eq!(format!("{b}"), "1234");
    assert!(BigInt::from_str(" 12 34").is_err());
    assert!(BigInt::from_str("999z").is_err());
    assert!(BigInt::from_str("abcd").is_err());
    assert!(BigInt::from_str("-xyz").is_err());
    assert!(BigInt::from_str(":").is_err());
    assert!(BigInt::from_str("%").is_err());
    assert!(BigInt::from_str("- 758").is_err());
    a = BigInt::from(42);
    assert_eq!(a, 42);

    a = BigInt::from(73786976294838206464_i128);
    assert_eq!(a, 73786976294838206464_u128);
    assert_eq!(format!("{a:X}"), "40000000000000000");
    assert!(a < "1361129467683753853853498429727072845824");
    assert!(a <= "1361129467683753853853498429727072845824");
    assert!(a != "1361129467683753853853498429727072845824");
    assert!(a <= 73786976294838206464_u128);
    assert!(a >= 73786976294838206464_u128);
    assert!(a == 73786976294838206464_u128);
    assert!(!(a != 73786976294838206464_i128));
    assert!(a <= 73786976294838206464_u128);
    assert!(a >= 73786976294838206464_u128);
    a = BigInt::from_str("2147483648").unwrap_or(BigInt::new());
    assert!(a > -2147483648_i128);
    assert!(a >= -2147483648_i128);
    assert!(!(a == -2147483648_i128));
    assert!(a != -2147483648_i128);
    a = BigInt::from_str("-12345678").unwrap_or(BigInt::new());
    assert!(a > -87654321);
    assert!(a >= -87654321);
    assert!(!(a == -87654321));
    assert!(a != -87654321);
}

use num_traits::ToPrimitive;
use std::{
    cmp::Ordering,
    fmt::{self, Binary, Display, LowerHex, UpperHex},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
    str::FromStr,
};

#[derive(Debug)]
enum BigIntError {
    NaN,
}

#[derive(Clone, Eq)]
struct big_int {
    positive: bool,
    numbers: Vec<u8>,
}

impl Default for big_int {
    fn default() -> Self {
        big_int {
            positive: true,
            numbers: vec![0],
        }
    }
}

impl FromStr for big_int {
    type Err = BigIntError;
    fn from_str(mut string_of_numbers: &str) -> Result<Self, Self::Err> {
        //empty string edgecaase
        if string_of_numbers.is_empty() {
            return Err(BigIntError::NaN);
        }

        let mut positive = !(string_of_numbers.starts_with('-'));
        let mut numbers: Vec<u8> = Vec::new();

        //if negative - remove '-'
        if !positive {
            string_of_numbers = string_of_numbers.split_at(1).1;
        }

        for char in string_of_numbers.chars() {
            if !char.is_ascii_digit() {
                return big_int::parse_word_digits(if positive {
                    string_of_numbers.to_string()
                } else {
                    format!("-{}", string_of_numbers)
                });
            }

            numbers.push(char.to_digit(10).unwrap().to_u8().unwrap());
        }

        if numbers == vec![0] {
            positive = true;
        }

        Ok(big_int { positive, numbers })
    }
}

macro_rules! from_int {
    ($($t:ty),*)=>{
        $(
            impl From<$t> for big_int{
                fn from(mut original_number: $t) -> Self {

                //zero edgecase
                if original_number == 0 {
                    return big_int::default();
                }

                let mut numbers = Vec::new();
                let positive = !(original_number < 0);

                //negative number
                if original_number < 0 {
                    original_number = -original_number;
                }

                //transformation of digits
                while original_number != 0{
                    numbers.insert(0, (original_number % 10).to_u8().unwrap());
                    original_number /= 10;
                }

                //return value
                big_int {positive, numbers}

                }
            }
        )*
    }
}

macro_rules! from_uint {
    ($($t:ty),*)=>{
        $(
            impl From<$t> for big_int{
                fn from(mut original_number: $t) -> Self {

                //zero edgecase
                if original_number == 0 {
                    return big_int::default();
                }

                let mut numbers = Vec::new();

                //transformation of digits
                while original_number != 0{
                    numbers.insert(0, (original_number % 10).to_u8().unwrap());
                    original_number /= 10;
                }

                //return value
                big_int {
                    positive: true, numbers}

                }
            }
        )*
    }
}

from_int!(i8, i16, i32, i64, i128);
from_uint!(u8, u16, u32, u64, u128);

impl Display for big_int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.positive {
            write!(f, "-")?;
        }

        for digit in self.numbers.clone() {
            write!(f, "{digit}")?;
        }
        Ok(())
    }
}

impl fmt::Debug for big_int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.positive {
            write!(f, "+")?;
        }

        write!(f, "{}", self)
    }
}

impl Binary for big_int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl UpperHex for big_int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl LowerHex for big_int {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

macro_rules! eq_with_int {
    ($($t:ty),*) => {
        $(
            impl PartialEq<$t> for big_int{
                fn eq(&self, other: &$t) -> bool {
                    self == &big_int::from(*other)
                }
            }
        )*
    };
}

eq_with_int!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

impl PartialEq<&str> for big_int {
    fn eq(&self, other: &&str) -> bool {
        let right_side = big_int::from_str(other);
        if right_side.is_err() {
            return false;
        }
        self == &right_side.unwrap()
    }
}

impl PartialEq for big_int {
    fn eq(&self, other: &big_int) -> bool {
        if self.positive != other.positive {
            false
        } else {
            self.numbers == other.numbers
        }
    }
}

impl Neg for big_int {
    type Output = Self;
    fn neg(self) -> Self::Output {
        !self
    }
}

impl Not for big_int {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        self.positive = !self.positive;
        self
    }
}

impl PartialOrd for big_int {
    fn ge(&self, other: &Self) -> bool {
        todo!()
    }
    fn gt(&self, other: &Self) -> bool {
        todo!()
    }
    fn le(&self, other: &Self) -> bool {
        todo!()
    }
    fn lt(&self, other: &Self) -> bool {
        todo!()
    }
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        todo!()
    }
}

impl PartialOrd<i128> for big_int {
    fn ge(&self, other: &i128) -> bool {
        todo!()
    }
    fn gt(&self, other: &i128) -> bool {
        todo!()
    }
    fn le(&self, other: &i128) -> bool {
        todo!()
    }
    fn lt(&self, other: &i128) -> bool {
        todo!()
    }
    fn partial_cmp(&self, other: &i128) -> Option<Ordering> {
        todo!()
    }
}

impl PartialOrd<&str> for big_int {
    fn ge(&self, other: &&str) -> bool {
        todo!()
    }
    fn gt(&self, other: &&str) -> bool {
        todo!()
    }
    fn le(&self, other: &&str) -> bool {
        todo!()
    }
    fn lt(&self, other: &&str) -> bool {
        todo!()
    }
    fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
        todo!()
    }
}

impl<T> Add<T> for big_int
where
    T: Into<big_int>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> AddAssign<T> for big_int
where
    T: Into<big_int>,
{
    fn add_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> Sub<T> for big_int
where
    T: Into<big_int>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> SubAssign<T> for big_int
where
    T: Into<big_int>,
{
    fn sub_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> Mul<T> for big_int
where
    T: Into<big_int>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> MulAssign<T> for big_int
where
    T: Into<big_int>,
{
    fn mul_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> Div<T> for big_int
where
    T: Into<big_int>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> DivAssign<T> for big_int
where
    T: Into<big_int>,
{
    fn div_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> Rem<T> for big_int
where
    T: Into<big_int>,
{
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> RemAssign<T> for big_int
where
    T: Into<big_int>,
{
    fn rem_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> BitAnd<T> for big_int
where
    T: Into<big_int>,
{
    type Output = Self;
    fn bitand(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> BitAndAssign<T> for big_int
where
    T: Into<big_int>,
{
    fn bitand_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> BitOr<T> for big_int
where
    T: Into<big_int>,
{
    type Output = Self;
    fn bitor(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> BitOrAssign<T> for big_int
where
    T: Into<big_int>,
{
    fn bitor_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> BitXor<T> for big_int
where
    T: Into<big_int>,
{
    type Output = Self;
    fn bitxor(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> BitXorAssign<T> for big_int
where
    T: Into<big_int>,
{
    fn bitxor_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> Shl<T> for big_int
where
    T: Into<big_int>,
{
    type Output = Self;
    fn shl(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> ShlAssign<T> for big_int
where
    T: Into<big_int>,
{
    fn shl_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> Shr<T> for big_int
where
    T: Into<big_int>,
{
    type Output = Self;
    fn shr(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> ShrAssign<T> for big_int
where
    T: Into<big_int>,
{
    fn shr_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl big_int {
    pub fn new() -> big_int {
        big_int::default()
    }

    pub fn to_words(&self) -> String {
        let mut fin_str = String::new();
        let mut nmr_iter = self.numbers.iter();

        //print minus or first digit
        if !self.positive {
            fin_str = "minus".to_string();
        } else {
            fin_str = big_int::number_to_word(*nmr_iter.next().unwrap_or(&0));
        }

        //print all digits
        for num in nmr_iter {
            fin_str = format!("{} {}", fin_str, big_int::number_to_word(*num));
        }

        fin_str
    }

    fn word_to_number(word: &str) -> Result<u8, BigIntError> {
        match word {
            "zero" => Ok(0),
            "one" => Ok(1),
            "two" => Ok(2),
            "three" => Ok(3),
            "four" => Ok(4),
            "five" => Ok(5),
            "six" => Ok(6),
            "seven" => Ok(7),
            "eight" => Ok(8),
            "nine" => Ok(9),
            _ => Err(BigIntError::NaN),
        }
    }

    fn number_to_word(number: u8) -> String {
        match number {
            0 => "zero",
            1 => "one",
            2 => "two",
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            7 => "seven",
            8 => "eight",
            9 => "nine",
            _ => "zero",
        }
        .to_string()
    }

    fn parse_word_digits(string_of_numbers: String) -> Result<big_int, BigIntError> {
        //create lowercase iterator
        let mut parsed = string_of_numbers
            .split_whitespace()
            .map(str::to_lowercase)
            .peekable();
        let mut positive = true;
        let mut numbers: Vec<u8> = Vec::new();

        //if empty string
        if parsed.peek().is_none() {
            return Err(BigIntError::NaN);
        }

        //positive/negative
        if let Some("-" | "minus") = parsed.peek().map(String::as_str) {
            positive = false;
            parsed.next();
        }

        //loop for translating words to u8
        for word in parsed {
            numbers.push(big_int::word_to_number(&word)?);
        }

        //additional check
        if numbers.is_empty() {
            return Err(BigIntError::NaN);
        }

        Ok(big_int { positive, numbers })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let def = big_int::default();
        assert_eq!(def.positive, true);
        assert_eq!(def.numbers, [0].to_vec());
    }

    #[test]
    fn new() {
        let new = big_int::new();
        assert_eq!(new.positive, true);
        assert_eq!(new.numbers, [0].to_vec());
    }

    #[test]
    fn from() {
        let x = big_int::from(20);
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [2, 0].to_vec());
        let x = big_int::from(-20);
        assert_eq!(x.positive, false);
        assert_eq!(x.numbers, [2, 0].to_vec());
        let x = big_int::from(-320020000981234567890i128);
        assert_eq!(x.positive, false);
        assert_eq!(
            x.numbers,
            [
                3, 2, 0, 0, 2, 0, 0, 0, 0, 9, 8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0
            ]
            .to_vec()
        );
        let x = big_int::from(0);
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [0].to_vec());
        let x = big_int::from(-0);
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [0].to_vec());
    }

    #[test]
    fn from_string_numbers() {
        let x = big_int::from_str("20").unwrap();
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [2, 0].to_vec());
        let x = big_int::from_str("666").unwrap();
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [6, 6, 6].to_vec());
        let x = big_int::from_str("-20").unwrap();
        assert_eq!(x.positive, false);
        assert_eq!(x.numbers, [2, 0].to_vec());
        let x = big_int::from_str("-320020000981234567890").unwrap();
        assert_eq!(x.positive, false);
        assert_eq!(
            x.numbers,
            [
                3, 2, 0, 0, 2, 0, 0, 0, 0, 9, 8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0
            ]
            .to_vec()
        );
        let x = big_int::from_str("-0").unwrap();
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [0].to_vec());
        let x = big_int::from_str("0sw");
        assert!(x.is_err());
        let x = big_int::from_str("0 2020000");
        assert!(x.is_err());
        let x = big_int::from_str("--200");
        assert!(x.is_err());
        let x = big_int::from_str("+2000303");
        assert!(x.is_err());
        let x = big_int::from_str("minus20003002");
        assert!(x.is_err());
    }

    #[test]
    fn from_string_words_from_str_digits() {
        let x = big_int::from_str("two zero     ").unwrap();
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [2, 0].to_vec());
        let x = big_int::from_str("minus two four").unwrap();
        assert_eq!(x.positive, false);
        assert_eq!(x.numbers, [2, 4].to_vec());
        let x = big_int::from_str("two five five zero zero two one").unwrap();
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [2, 5, 5, 0, 0, 2, 1].to_vec());
        let x = big_int::from_str("minus two     zero zero zero zero zero one").unwrap();
        assert_eq!(x.positive, false);
        assert_eq!(x.numbers, [2, 0, 0, 0, 0, 0, 1].to_vec());
        let x = big_int::from_str("zero").unwrap();
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [0].to_vec());
        let x = big_int::from_str("onse");
        assert!(x.is_err());
        let x = big_int::from_str("        ");
        assert!(x.is_err());
        let x = big_int::from_str("twenty thousand thousand");
        assert!(x.is_err());
        let x: Result<big_int, BigIntError> = big_int::from_str("twenty thousand hundred");
        assert!(x.is_err());
        let x = big_int::from_str("- five four").unwrap();
        assert_eq!(x.positive, false);
        assert_eq!(x.numbers, [5, 4].to_vec());
    }

    #[test]
    fn display() {
        let x = big_int::from(1003);
        assert_eq!(format!("Number is: {x}"), "Number is: 1003");
        let x = big_int::from(0);
        assert_eq!(format!("Number is: {x}"), "Number is: 0");
        let x = big_int::from(-100);
        assert_eq!(format!("Number is: {x}"), "Number is: -100");
        let x = big_int::from(-0);
        assert_eq!(format!("Number is: {x}"), "Number is: 0");
        let x = big_int::from(-1);
        assert_eq!(format!("Number is: {x}"), "Number is: -1");
        let x = big_int::from(320020000981234567890i128);
        assert_eq!(
            format!("Number is: {x}"),
            "Number is: 320020000981234567890"
        );
    }

    #[test]
    fn debug() {
        let x = big_int::from(1003);
        assert_eq!(format!("Number is: {x:?}"), "Number is: +1003");
        let x = big_int::from(0);
        assert_eq!(format!("Number is: {x:?}"), "Number is: +0");
        let x = big_int::from(-100);
        assert_eq!(format!("Number is: {x:?}"), "Number is: -100");
        let x = big_int::from(-0);
        assert_eq!(format!("Number is: {x:?}"), "Number is: +0");
        let x = big_int::from(-1);
        assert_eq!(format!("Number is: {x:?}"), "Number is: -1");
        let x = big_int::from(320020000981234567890i128);
        assert_eq!(
            format!("Number is: {x:?}"),
            "Number is: +320020000981234567890"
        );
    }

    #[test]
    fn to_words() {
        let x = big_int::from(20);
        assert_eq!(x.to_words(), "two zero");
        let x = big_int::from(-24);
        assert_eq!(x.to_words(), "minus two four");
        let x = big_int::from(2550021);
        assert_eq!(x.to_words(), "two five five zero zero two one");
        let x = big_int::from(-2000000000001i128);
        assert_eq!(
            x.to_words(),
            "minus two zero zero zero zero zero zero zero zero zero zero zero one"
        );
    }

    #[test]
    fn negation() {
        let mut x = big_int::from(1);
        assert_eq!(x.positive, true);
        assert_eq!(x.to_string(), "1");
        x = !x;
        assert_eq!(x.positive, false);
        assert_eq!(x.to_string(), "-1");
        x = !x;
        assert_eq!(x.positive, true);
        assert_eq!(x.to_string(), "1");
        let mut x = big_int::from(-22);
        assert_eq!(x.positive, false);
        assert_eq!(x.to_string(), "-22");
        x = !x;
        assert_eq!(x.positive, true);
        assert_eq!(x.to_string(), "22");
    }

    #[test]
    fn not() {
        let mut x = big_int::from(1);
        assert_eq!(x.positive, true);
        assert_eq!(x.to_string(), "1");
        x = -x;
        assert_eq!(x.positive, false);
        assert_eq!(x.to_string(), "-1");
        x = -x;
        assert_eq!(x.positive, true);
        assert_eq!(x.to_string(), "1");
        let mut x = big_int::from(-22);
        assert_eq!(x.positive, false);
        assert_eq!(x.to_string(), "-22");
        x = -x;
        assert_eq!(x.positive, true);
        assert_eq!(x.to_string(), "22");
    }

    #[test]
    fn equal() {
        let x = big_int::from(10);
        let y = big_int::from(10);
        assert!(x == y);
        let x = big_int::from(101010);
        let y = big_int::from(101010);
        assert!(x == y);
        let x = big_int::from(101010);
        let y = big_int::from(101210);
        assert!(!(x == y));
        let x = big_int::from(101010);
        let y = big_int::from(1);
        assert!(!(x == y));
        let x = big_int::from(0);
        let y = big_int::from(0);
        assert!(x == y);
        let x = big_int::from(10);
        let y = big_int::from(-10);
        assert!(x != y);
        let x = big_int::from(11);
        let y = big_int::from(10);
        assert!(x != y);
        let x = big_int::from(-0);
        let y = big_int::from(0);
        assert!(x == y);
    }

    #[test]
    fn equal_int() {
        let x = big_int::from(10);
        assert!(x == 10);
        let x = big_int::from(101010);
        assert!(x == 101010);
        let x = big_int::from(101010);
        assert!(!(x == 101210));
        let x = big_int::from(101010);
        assert!(!(x == 1));
        let x = big_int::from(0);
        assert!(x == 0);
        let x = big_int::from(10);
        assert!(x != -10);
        let x = big_int::from(11);
        assert!(x != 10);
        let x = big_int::from(-0);
        assert!(x == 0);
    }

    #[test]
    fn equal_str() {
        let x = big_int::from(10);
        assert!(x == "10");
        let x = big_int::from(101010);
        assert!(x == "101010");
        let x = big_int::from(101010);
        assert!(!(x == "101210"));
        let x = big_int::from(101010);
        assert!(!(x == "1"));
        let x = big_int::from(0);
        assert!(x == "0");
        let x = big_int::from(10);
        assert!(x != "-10");
        let x = big_int::from(11);
        assert!(x != "10");
        let x = big_int::from(-0);
        assert!(x == "0");
    }

    #[test]
    fn greater() {
        let x: big_int = big_int::from(15);
        let y = big_int::from(10);
        assert!(x > y);
        let x: big_int = big_int::from(8);
        let y = big_int::from(7);
        assert!(x > y);
        let x = big_int::from(10);
        let y = big_int::from(10);
        assert!(!(x > y));
        let x = big_int::from(10);
        let y = big_int::from(10);
        assert!(x >= y);
        let x = big_int::from(101010);
        let y = big_int::from(101010);
        assert!(x >= y);
        let x = big_int::from(0);
        let y = big_int::from(0);
        assert!(!(x > y));
        let x = big_int::from(10);
        let y = big_int::from(-10);
        assert!(x > y);
        let x = big_int::from(11);
        let y = big_int::from(10);
        assert!(x > y);
    }

    #[test]
    fn lesser() {
        let x: big_int = big_int::from(0);
        let y = big_int::from(10);
        assert!(x < y);
        let x: big_int = big_int::from(8);
        let y = big_int::from(9);
        assert!(x < y);
        let x = big_int::from(10);
        let y = big_int::from(10);
        assert!(!(x < y));
        let x = big_int::from(10);
        let y = big_int::from(10);
        assert!(x <= y);
        let x = big_int::from(99999999999i64);
        let y = big_int::from(99999999999i128);
        assert!(x <= y);
        let x = big_int::from(0);
        let y = big_int::from(0);
        assert!(!(x < y));
        let x = big_int::from(-10);
        let y = big_int::from(10);
        assert!(x < y);
        let x = big_int::from(11);
        let y = big_int::from(99999999999u64);
        assert!(x < y);
    }

    #[test]
    fn add() {
        let mut x: big_int = big_int::from(10);
        let y = big_int::from(10);
        let z = x.clone() + y.clone();
        x += y;
        assert_eq!(z, 20);
        assert_eq!(z, x);

        let mut x = big_int::from(101010);
        let y = big_int::from(101010);
        let z = x.clone() + y.clone();
        x += y;
        assert_eq!(z, 202020);
        assert_eq!(z, x);

        let mut x = big_int::from(0);
        let y = big_int::from(0);
        let z = x.clone() + y.clone();
        x += y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x = big_int::from(10);
        let y = big_int::from(-10);
        let z = x.clone() + y.clone();
        x += y.clone();
        assert_eq!(z, 0);
        assert_ne!(x, y);

        let mut x = big_int::from(11);
        let y = big_int::from(10);
        let z = x.clone() + y.clone();
        x += y.clone();
        assert_eq!(z, 21);
        assert_ne!(x, y);

        let mut x = big_int::from(-0);
        let y = big_int::from(0);
        let z = x.clone() + y.clone();
        x += y.clone();
        assert_eq!(z, 0);
        assert_eq!(x, y);

        let mut x = big_int::from(6);
        let y = big_int::from(4);
        let z = x.clone() + y.clone();
        x += y.clone();
        assert_eq!(z, 10);
        assert_eq!(x, y);

        let mut x = big_int::from(-15);
        let y = big_int::from(-4);
        let z = x.clone() + y.clone();
        x += y.clone();
        assert_eq!(z, -19);
        assert_eq!(x, y);
    }

    #[test]
    fn sub() {
        let mut x: big_int = big_int::from(10);
        let y = big_int::from(10);
        let z = x.clone() - y.clone();
        x -= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x = big_int::from(101010);
        let y = big_int::from(10);
        let z = x.clone() - y.clone();
        x -= y;
        assert_eq!(z, 101000);
        assert_eq!(z, x);

        let mut x = big_int::from(0);
        let y = big_int::from(0);
        let z = x.clone() - y.clone();
        x -= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x = big_int::from(10);
        let y = big_int::from(-10);
        let z = x.clone() - y.clone();
        x -= y.clone();
        assert_eq!(z, 20);
        assert_ne!(x, y);

        let mut x = big_int::from(-10);
        let y = big_int::from(10);
        let z = x.clone() - y.clone();
        x -= y.clone();
        assert_eq!(z, -20);
        assert_ne!(x, y);

        let mut x = big_int::from(11);
        let y = big_int::from(10);
        let z = x.clone() - y.clone();
        x -= y.clone();
        assert_eq!(z, 1);
        assert_ne!(x, y);

        let mut x = big_int::from(-0);
        let y = big_int::from(0);
        let z = x.clone() - y.clone();
        x -= y.clone();
        assert_eq!(z, 0);
        assert_eq!(x, y);

        let mut x = big_int::from(6);
        let y = big_int::from(4);
        let z = x.clone() - y.clone();
        x -= y.clone();
        assert_eq!(z, 2);
        assert_eq!(x, y);

        let mut x = big_int::from(-15);
        let y = big_int::from(-4);
        let z = x.clone() - y.clone();
        x -= y.clone();
        assert_eq!(z, -11);
        assert_eq!(x, y);
    }

    #[test]
    fn mul() {
        let mut x: big_int = big_int::from(10);
        let y = big_int::from(10);
        let z: big_int = x.clone() * y.clone();
        x *= y;
        assert_eq!(z, 100);
        assert_eq!(z, x);

        let mut x = big_int::from(101);
        let y = big_int::from(101);
        let z = x.clone() * y.clone();
        x *= y;
        assert_eq!(z, 10201);
        assert_eq!(z, x);

        let mut x = big_int::from(0);
        let y = big_int::from(0);
        let z: big_int = x.clone() * y.clone();
        x *= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x = big_int::from(20100000100u64);
        let y = big_int::from(0);
        let z: big_int = x.clone() * y.clone();
        x *= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x = big_int::from(10);
        let y = big_int::from(-10);
        let z: big_int = x.clone() * y.clone();
        x *= y.clone();
        assert_eq!(z, -100);
        assert_ne!(x, y);

        let mut x = big_int::from(11);
        let y = big_int::from(10);
        let z: big_int = x.clone() * y.clone();
        x *= y.clone();
        assert_eq!(z, 110);
        assert_ne!(x, y);

        let mut x = big_int::from(-0);
        let y = big_int::from(0);
        let z: big_int = x.clone() * y.clone();
        x *= y.clone();
        assert_eq!(z, 0);
        assert_eq!(x, y);

        let mut x = big_int::from(6);
        let y = big_int::from(4);
        let z: big_int = x.clone() * y.clone();
        x *= y.clone();
        assert_eq!(z, 24);
        assert_eq!(x, y);

        let mut x = big_int::from(-15);
        let y = big_int::from(-4);
        let z: big_int = x.clone() * y.clone();
        x *= y.clone();
        assert_eq!(z, 60);
        assert_eq!(x, y);
    }

    #[test]
    fn div() {
        let mut x: big_int = big_int::from(10000);
        let y = big_int::from(10);
        let z = x.clone() / y.clone();
        x /= y;
        assert_eq!(z, 1000);
        assert_eq!(z, x);

        let mut x = big_int::from(101);
        let y = big_int::from(101);
        let z = x.clone() / y.clone();
        x /= y;
        assert_eq!(z, 1);
        assert_eq!(z, x);

        let mut x = big_int::from(0);
        let y = big_int::from(2);
        let z = x.clone() / y.clone();
        x /= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x = big_int::from(0);
        let y = big_int::from(0);
        let z = x.clone() / y.clone();
        x /= y;
        assert_eq!(z, big_int::default());
        assert_eq!(z, x);

        let mut x = big_int::from(10000);
        let y = big_int::from(0);
        let z = x.clone() / y.clone();
        x /= y;
        assert_eq!(z, big_int::default());
        assert_eq!(z, x);

        let mut x = big_int::from(20100000100u64);
        let y = big_int::from(200);
        let z = x.clone() / y.clone();
        x /= y;
        assert_eq!(z, 100500);
        assert_eq!(z, x);

        let mut x = big_int::from(10);
        let y = big_int::from(-10);
        let z = x.clone() / y.clone();
        x /= y.clone();
        assert_eq!(z, -1);
        assert_ne!(x, y);

        let mut x = big_int::from(110);
        let y = big_int::from(10);
        let z = x.clone() / y.clone();
        x /= y.clone();
        assert_eq!(z, 11);
        assert_ne!(x, y);

        let mut x = big_int::from(6);
        let y = big_int::from(2);
        let z = x.clone() / y.clone();
        x /= y.clone();
        assert_eq!(z, 3);
        assert_eq!(x, y);

        let mut x = big_int::from(-15);
        let y = big_int::from(-5);
        let z = x.clone() / y.clone();
        x /= y.clone();
        assert_eq!(z, 3);
        assert_eq!(x, y);
    }

    #[test]
    fn reminder() {
        let mut x: big_int = big_int::from(10000);
        let y = big_int::from(10);
        let z = x.clone() % y.clone();
        x %= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x: big_int = big_int::from(10);
        let y = big_int::from(0);
        let z = x.clone() % y.clone();
        x %= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x: big_int = big_int::from(10);
        let y = big_int::from(7);
        let z = x.clone() % y.clone();
        x %= y;
        assert_eq!(z, 3);
        assert_eq!(z, x);

        let mut x: big_int = big_int::from(10000);
        let y = big_int::from(10);
        let z = x.clone() % y.clone();
        x %= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x: big_int = big_int::from(-104);
        let y = big_int::from(10);
        let z = x.clone() % y.clone();
        x %= y;
        assert_eq!(z, 4);
        assert_eq!(z, x);

        let mut x: big_int = big_int::from(24);
        let y = big_int::from(3);
        let z = x.clone() % y.clone();
        x %= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x: big_int = big_int::from(33);
        let y = big_int::from(7);
        let z = x.clone() % y.clone();
        x %= y;
        assert_eq!(z, 5);
        assert_eq!(z, x);
    }

    #[test]
    fn binary() {
        let x = big_int::from(4);
        assert_eq!(format!("{x:b}"), "100");
        let x = big_int::from(4);
        assert_eq!(format!("{x:#b}"), "0b100");
        let x = big_int::from(4);
        assert_eq!(format!("{x:010b}"), "0000000100");
        let x = big_int::from(10);
        assert_eq!(format!("{x:#110b}"), "0b11111010");
        let x = big_int::from(172);
        assert_eq!(format!("{x:b}"), "10101100");
        let x = big_int::from(17220003931i64);
        assert_eq!(format!("{x:#b}"), "0b10000000010011001000110100001011011");
    }

    #[test]
    fn bit_and() {
        let mut x = big_int::from(4); //100
        let y = big_int::from(12); //1100
        let z = x.clone() & y.clone(); //0100
        x &= y;
        assert_eq!(x, z);
        assert_eq!(z, 4);

        let mut x = big_int::from(10); //1010
        let y = big_int::from(13); //1101
        let z = x.clone() & y.clone(); //1000
        x &= y;
        assert_eq!(x, z);
        assert_eq!(z, 8);

        let mut x = big_int::from(172); //10101100
        let y = big_int::from(223); //11011111
        let z = x.clone() & y.clone(); //10001100
        x &= y;
        assert_eq!(x, z);
        assert_eq!(z, 140);

        let mut x = big_int::from(172); //10101100
        let y = big_int::from(1); //1
        let z = x.clone() & y.clone(); //0
        x &= y;
        assert_eq!(x, z);
        assert_eq!(z, 0);

        let mut x = big_int::from(173); //10101101
        let y = big_int::from(1); //1
        let z = x.clone() & y.clone(); //1
        x &= y;
        assert_eq!(x, z);
        assert_eq!(z, 1);
    }

    #[test]
    fn bit_or() {
        let mut x = big_int::from(4); //100
        let y = big_int::from(12); //1100
        let z = x.clone() | y.clone(); //1100
        x |= y;
        assert_eq!(x, z);
        assert_eq!(z, 12);

        let mut x = big_int::from(10); //1010
        let y = big_int::from(13); //1101
        let z = x.clone() | y.clone(); //1111
        x |= y;
        assert_eq!(x, z);
        assert_eq!(z, 15);

        let mut x = big_int::from(172); //10101100
        let y = big_int::from(223); //11011111
        let z = x.clone() | y.clone(); //11111111
        x |= y;
        assert_eq!(x, z);
        assert_eq!(z, 255);

        let mut x = big_int::from(172); //10101100
        let y = big_int::from(1); //1
        let z = x.clone() | y.clone(); //10101101
        x |= y;
        assert_eq!(x, z);
        assert_eq!(z, 173);

        let mut x = big_int::from(173); //10101101
        let y = big_int::from(1); //1
        let z = x.clone() | y.clone(); //10101101
        x |= y;
        assert_eq!(x, z);
        assert_eq!(z, 173);
    }

    #[test]
    fn bit_xor() {
        let mut x = big_int::from(4); //100
        let y = big_int::from(12); //1100
        let z = x.clone() ^ y.clone(); //1000
        x ^= y;
        assert_eq!(x, z);
        assert_eq!(z, 8);

        let mut x = big_int::from(10); //1010
        let y = big_int::from(13); //1101
        let z = x.clone() ^ y.clone(); //0111
        x ^= y;
        assert_eq!(x, z);
        assert_eq!(z, 7);

        let mut x = big_int::from(172); //10101100
        let y = big_int::from(223); //11011111
        let z = x.clone() ^ y.clone(); //01110011
        x ^= y;
        assert_eq!(x, z);
        assert_eq!(z, 115);

        let mut x = big_int::from(172); //10101100
        let y = big_int::from(1); //1
        let z = x.clone() ^ y.clone(); //10101101
        x ^= y;
        assert_eq!(x, z);
        assert_eq!(z, 173);

        let mut x = big_int::from(173); //10101101
        let y = big_int::from(1); //1
        let z = x.clone() ^ y.clone(); //10101100
        x ^= y;
        assert_eq!(x, z);
        assert_eq!(z, 172);
    }

    #[test]
    fn bit_shift_left() {
        let mut x = big_int::from(4); //100
        let y = big_int::from(2);
        let z = x.clone() << y.clone(); //10000
        x <<= y;
        assert_eq!(x, z);
        assert_eq!(z, 16);

        let mut x = big_int::from(10); //1010
        let y = big_int::from(1);
        let z = x.clone() << y.clone(); //10100
        x <<= y;
        assert_eq!(x, z);
        assert_eq!(z, 20);

        let mut x = big_int::from(172); //10101100
        let y = big_int::from(0);
        let z = x.clone() << y.clone();
        x <<= y;
        assert_eq!(x, z);
        assert_eq!(z, 172);

        let mut x = big_int::from(172); //10101100
        let y = big_int::from(10);
        let z = x.clone() << y.clone(); //101011000000000000
        x <<= y;
        assert_eq!(x, z);
        assert_eq!(z, 176128);

        let mut x = big_int::from(173); //10101101
        let y = big_int::from(1);
        let z = x.clone() << y.clone(); //101011010
        x <<= y;
        assert_eq!(x, z);
        assert_eq!(z, 346);
    }

    #[test]
    fn bit_shift_right() {
        let mut x = big_int::from(4); //100
        let y = big_int::from(2);
        let z = x.clone() >> y.clone(); //1
        x >>= y;
        assert_eq!(x, z);
        assert_eq!(z, 1);

        let mut x = big_int::from(10); //1010
        let y = big_int::from(1);
        let z = x.clone() >> y.clone(); //101
        x >>= y;
        assert_eq!(x, z);
        assert_eq!(z, 5);

        let mut x = big_int::from(172); //10101100
        let y = big_int::from(0);
        let z = x.clone() >> y.clone(); //10101100
        x >>= y;
        assert_eq!(x, z);
        assert_eq!(z, 172);

        let mut x = big_int::from(172); //10101100
        let y = big_int::from(10);
        let z = x.clone() >> y.clone(); //0
        x >>= y;
        assert_eq!(x, z);
        assert_eq!(z, 0);

        let mut x = big_int::from(173); //10101101
        let y = big_int::from(1);
        let z = x.clone() >> y.clone(); //1010110
        x >>= y;
        assert_eq!(x, z);
        assert_eq!(z, 86);
    }

    #[test]
    fn hexadecimal() {
        let x = big_int::from(4);
        assert_eq!(format!("{x:X}"), "4");
        let x = big_int::from(16);
        assert_eq!(format!("{x:#X}"), "0xF");
        let x = big_int::from(10);
        assert_eq!(format!("{x:#X}"), "0x10");
        let x = big_int::from(172);
        assert_eq!(format!("{x:X}"), "AC");
        let x = big_int::from(17220003931u128);
        assert_eq!(format!("{x:#X}"), "0x40264685B");

        let x = big_int::from(4);
        assert_eq!(format!("{x:x}"), "4");
        let x = big_int::from(16);
        assert_eq!(format!("{x:#x}"), "0xf");
        let x = big_int::from(10);
        assert_eq!(format!("{x:#x}"), "0x10");
        let x = big_int::from(172);
        assert_eq!(format!("{x:x}"), "ac");
        let x = big_int::from(17220003931u128);
        assert_eq!(format!("{x:#x}"), "0x40264685b");
    }

    #[test]
    fn progtest_tests() {
        let mut a = big_int::new();
        let mut b = big_int::new();
        a = big_int::from(10);
        a += big_int::from(20);
        assert_eq!(a, 30);
        a *= big_int::from(5);
        assert_eq!(a, 150);
        b = a.clone() + big_int::from(3);
        assert_eq!(b, 153);
        b = a.clone() * big_int::from(7);
        assert_eq!(b, 1050);
        assert_eq!(a, 150);
        assert_eq!(format!("{a:X}"), "96");

        a = big_int::from(10);
        a += big_int::from(-20);
        assert_eq!(a, -10);
        a *= big_int::from(5);
        assert_eq!(a, -50);
        b = a.clone() + big_int::from(73);
        assert_eq!(b, 23);
        b = a.clone() * big_int::from(-7);
        assert_eq!(b, 350);
        assert_eq!(a, -50);
        assert_eq!(format!("{a:X}"), "-32");

        a = big_int::from(12345678901234567890i128);
        a += big_int::from(-99999999999999999999i128);
        assert_eq!(a, -87654321098765432109i128);
        a *= big_int::from(54321987654321987654i128);
        assert_eq!(a, "-4761556948575111126880627366067073182286");
        a *= big_int::from(0);
        assert_eq!(a, 0);
        a = big_int::from(10);
        b = a.clone() + 400;
        assert_eq!(b, "410");
        b = a.clone() * big_int::from_str("15").unwrap_or_default();
        assert_eq!(b, "150");
        assert_eq!(a, "10");
        assert_eq!(format!("{a:X}"), "A");

        b = big_int::from_str("1234").unwrap_or_default();
        assert_eq!(format!("{b}"), "1234");
        assert!(big_int::from_str(" 12 34").is_err());
        assert!(big_int::from_str("999z").is_err());
        assert!(big_int::from_str("abcd").is_err());
        assert!(big_int::from_str("-xyz").is_err());
        assert!(big_int::from_str(":").is_err());
        assert!(big_int::from_str("%").is_err());
        assert!(big_int::from_str("- 758").is_err());
        a = big_int::from(42);
        assert_eq!(a, 42);

        a = big_int::from(73786976294838206464i128);
        assert_eq!(a, 73786976294838206464u128);
        assert_eq!(format!("{a:X}"), "40000000000000000");
        assert!(a < "1361129467683753853853498429727072845824");
        assert!(a <= "1361129467683753853853498429727072845824");
        assert!(!(a > "1361129467683753853853498429727072845824"));
        assert!(!(a >= "1361129467683753853853498429727072845824"));
        assert!(!(a == "1361129467683753853853498429727072845824"));
        assert!(a != "1361129467683753853853498429727072845824");
        assert!(!(a < 73786976294838206464));
        assert!(a <= 73786976294838206464);
        assert!(!(a > 73786976294838206464));
        assert!(a >= 73786976294838206464);
        assert!(a == 73786976294838206464u128);
        assert!(!(a != 73786976294838206464i128));
        assert!(a < 73786976294838206465);
        assert!(a <= 73786976294838206465);
        assert!(!(a > 73786976294838206465));
        assert!(!(a >= 73786976294838206465));
        assert!(!(a == 73786976294838206465u128));
        assert!(a != 73786976294838206465i128);
        a = big_int::from_str("2147483648").unwrap_or(big_int::new());
        assert!(!(a < -2147483648));
        assert!(!(a <= -2147483648));
        assert!(a > -2147483648);
        assert!(a >= -2147483648);
        assert!(!(a == -2147483648));
        assert!(a != -2147483648);
        a = big_int::from_str("-12345678").unwrap_or(big_int::new());
        assert!(!(a < -87654321));
        assert!(!(a <= -87654321));
        assert!(a > -87654321);
        assert!(a >= -87654321);
        assert!(!(a == -87654321));
        assert!(a != -87654321);
    }
}

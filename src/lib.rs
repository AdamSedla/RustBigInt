use core::num;
use num_traits::{PrimInt, ToPrimitive};
use std::{
    char,
    cmp::Ordering,
    f32::DIGITS,
    fmt::{self, Binary, Display, LowerHex, UpperHex, format},
    ops::{
        Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div,
        DivAssign, Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub,
        SubAssign,
    },
    str::FromStr,
    sync::Arc,
    u8, vec,
};

#[derive(Debug)]
enum BigIntError {
    NaN,
}

#[derive(Clone, Eq)]
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

fn number_to_word(nmr: u8) -> String {
    match nmr {
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

fn parse_word_digits(num_str: String) -> Result<BigInt, BigIntError> {
    //create lowercase iterator
    let mut parsed = num_str.split_whitespace().map(str::to_lowercase).peekable();
    let mut fin_pos = true;
    let mut fin_vec: Vec<u8> = Vec::new();

    //if empty string
    if parsed.peek().is_none() {
        return Err(BigIntError::NaN);
    }

    //positive/negative
    if let Some("-" | "minus") = parsed.peek().map(String::as_str) {
        fin_pos = false;
        parsed.next();
    }

    //loop for translating words to u8
    for word in parsed {
        fin_vec.push(word_to_number(&word)?);
    }

    //additional check
    if fin_vec.is_empty() {
        return Err(BigIntError::NaN);
    }

    Ok(BigInt {
        positive: fin_pos,
        numbers: fin_vec,
    })
}

impl FromStr for BigInt {
    type Err = BigIntError;
    fn from_str(mut num_str: &str) -> Result<Self, Self::Err> {
        //empty string edgecaase
        if num_str.is_empty() {
            return Err(BigIntError::NaN);
        }

        let mut fin_pos = !(num_str.starts_with('-'));
        let mut fin_vec: Vec<u8> = Vec::new();

        //if negative - remove '-'
        if !fin_pos {
            num_str = num_str.split_at(1).1;
        }

        for char in num_str.chars() {
            if !char.is_ascii_digit() {
                return parse_word_digits(if fin_pos {
                    num_str.to_string()
                } else {
                    format!("-{}", num_str)
                });
            }

            fin_vec.push(char.to_digit(10).unwrap().to_u8().unwrap());
        }

        if fin_vec == vec![0] {
            fin_pos = true;
        }

        Ok(BigInt {
            positive: fin_pos,
            numbers: fin_vec,
        })
    }
}

impl<T> From<T> for BigInt
where
    T: PrimInt,
{
    fn from(value: T) -> Self {
        //zero edgecase
        if value.is_zero() {
            return BigInt::default();
        }

        let mut val = value;
        let mut fin_vec = Vec::new();
        let mut fin_pos = true;

        //negative number
        if val < T::zero() {
            fin_pos = false;
            val = val * T::from(-1).unwrap();
        }

        //transformation of digits
        while !val.is_zero() {
            fin_vec.insert(0, (val % T::from(10).unwrap()).to_u8().unwrap());
            val = val / T::from(10).unwrap();
        }

        //return value
        BigInt {
            positive: fin_pos,
            numbers: fin_vec,
        }
    }
}

impl Display for BigInt {
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

impl fmt::Debug for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.positive {
            write!(f, "+")?;
        }

        write!(f, "{}", self)
    }
}

impl Binary for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl UpperHex for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl LowerHex for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl PartialEq<i128> for BigInt {
    fn eq(&self, other: &i128) -> bool {
        todo!()
    }
    fn ne(&self, other: &i128) -> bool {
        todo!()
    }
}

impl PartialEq<&str> for BigInt {
    fn eq(&self, other: &&str) -> bool {
        todo!()
    }
    fn ne(&self, other: &&str) -> bool {
        todo!()
    }
}

impl PartialEq for BigInt {
    fn eq(&self, other: &BigInt) -> bool {
        todo!()
    }
    fn ne(&self, other: &BigInt) -> bool {
        todo!()
    }
}

impl Neg for BigInt {
    type Output = Self;
    fn neg(self) -> Self::Output {
        !self
    }
}

impl Not for BigInt {
    type Output = Self;
    fn not(mut self) -> Self::Output {
        self.positive = !self.positive;
        self
    }
}

impl PartialOrd for BigInt {
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

impl PartialOrd<i128> for BigInt {
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

impl PartialOrd<&str> for BigInt {
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

impl<T> Add<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn add(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> AddAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn add_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> Sub<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;

    fn sub(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> SubAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn sub_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> Mul<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> MulAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn mul_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> Div<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn div(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> DivAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn div_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> Rem<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> RemAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn rem_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> BitAnd<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn bitand(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> BitAndAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn bitand_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> BitOr<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn bitor(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> BitOrAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn bitor_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> BitXor<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn bitxor(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> BitXorAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn bitxor_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> Shl<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn shl(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> ShlAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn shl_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl<T> Shr<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn shr(self, rhs: T) -> Self::Output {
        todo!()
    }
}

impl<T> ShrAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn shr_assign(&mut self, rhs: T) {
        todo!()
    }
}

impl BigInt {
    pub fn new() -> BigInt {
        BigInt::default()
    }

    pub fn to_words(&self) -> String {
        let mut fin_str = String::new();
        let mut nmr_iter = self.numbers.iter();

        //print minus or first digit
        if !self.positive {
            fin_str = "minus".to_string();
        } else {
            fin_str = number_to_word(*nmr_iter.next().unwrap_or(&0));
        }

        //print all digits
        for num in nmr_iter {
            fin_str = format!("{} {}", fin_str, number_to_word(*num));
        }

        fin_str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let x = BigInt::from(-320020000981234567890i128);
        assert_eq!(x.positive, false);
        assert_eq!(
            x.numbers,
            [
                3, 2, 0, 0, 2, 0, 0, 0, 0, 9, 8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0
            ]
            .to_vec()
        );
        let x = BigInt::from(0);
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [0].to_vec());
        let x = BigInt::from(-0);
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [0].to_vec());
    }

    #[test]
    fn from_string_numbers() {
        let x = BigInt::from_str("20").unwrap();
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [2, 0].to_vec());
        let x = BigInt::from_str("666").unwrap();
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [6, 6, 6].to_vec());
        let x = BigInt::from_str("-20").unwrap();
        assert_eq!(x.positive, false);
        assert_eq!(x.numbers, [2, 0].to_vec());
        let x = BigInt::from_str("-320020000981234567890").unwrap();
        assert_eq!(x.positive, false);
        assert_eq!(
            x.numbers,
            [
                3, 2, 0, 0, 2, 0, 0, 0, 0, 9, 8, 1, 2, 3, 4, 5, 6, 7, 8, 9, 0
            ]
            .to_vec()
        );
        let x = BigInt::from_str("-0").unwrap();
        assert_eq!(x.positive, true);
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
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [2, 0].to_vec());
        let x = BigInt::from_str("minus two four").unwrap();
        assert_eq!(x.positive, false);
        assert_eq!(x.numbers, [2, 4].to_vec());
        let x = BigInt::from_str("two five five zero zero two one").unwrap();
        assert_eq!(x.positive, true);
        assert_eq!(x.numbers, [2, 5, 5, 0, 0, 2, 1].to_vec());
        let x = BigInt::from_str("minus two     zero zero zero zero zero one").unwrap();
        assert_eq!(x.positive, false);
        assert_eq!(x.numbers, [2, 0, 0, 0, 0, 0, 1].to_vec());
        let x = BigInt::from_str("zero").unwrap();
        assert_eq!(x.positive, true);
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
        assert_eq!(x.positive, false);
        assert_eq!(x.numbers, [5, 4].to_vec());
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
        assert_eq!(format!("Number is: {x}"), "Number is: 0");
        let x = BigInt::from(-1);
        assert_eq!(format!("Number is: {x}"), "Number is: -1");
        let x = BigInt::from(320020000981234567890i128);
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
        assert_eq!(format!("Number is: {x:?}"), "Number is: +0");
        let x = BigInt::from(-1);
        assert_eq!(format!("Number is: {x:?}"), "Number is: -1");
        let x = BigInt::from(320020000981234567890i128);
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
        let x = BigInt::from(-2000000000001i128);
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
        x = !x;
        assert_eq!(x.positive, true);
        assert_eq!(x.to_string(), "1");
        let mut x = BigInt::from(-22);
        assert_eq!(x.positive, false);
        assert_eq!(x.to_string(), "-22");
        x = !x;
        assert_eq!(x.positive, true);
        assert_eq!(x.to_string(), "22");
    }

    #[test]
    fn not() {
        let mut x = BigInt::from(1);
        assert_eq!(x.positive, true);
        assert_eq!(x.to_string(), "1");
        x = -x;
        assert_eq!(x.positive, false);
        assert_eq!(x.to_string(), "-1");
        x = -x;
        assert_eq!(x.positive, true);
        assert_eq!(x.to_string(), "1");
        let mut x = BigInt::from(-22);
        assert_eq!(x.positive, false);
        assert_eq!(x.to_string(), "-22");
        x = -x;
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
        let x = BigInt::from(99999999999i64);
        let y = BigInt::from(99999999999i128);
        assert!(x <= y);
        let x = BigInt::from(0);
        let y = BigInt::from(0);
        assert!(!(x < y));
        let x = BigInt::from(-10);
        let y = BigInt::from(10);
        assert!(x < y);
        let x = BigInt::from(11);
        let y = BigInt::from(99999999999u64);
        assert!(x < y);
    }

    #[test]
    fn add() {
        let mut x: BigInt = BigInt::from(10);
        let y = BigInt::from(10);
        let z = x.clone() + y.clone();
        x += y;
        assert_eq!(z, 20);
        assert_eq!(z, x);

        let mut x = BigInt::from(101010);
        let y = BigInt::from(101010);
        let z = x.clone() + y.clone();
        x += y;
        assert_eq!(z, 202020);
        assert_eq!(z, x);

        let mut x = BigInt::from(0);
        let y = BigInt::from(0);
        let z = x.clone() + y.clone();
        x += y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

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
        assert_eq!(x, y);

        let mut x = BigInt::from(6);
        let y = BigInt::from(4);
        let z = x.clone() + y.clone();
        x += y.clone();
        assert_eq!(z, 10);
        assert_eq!(x, y);

        let mut x = BigInt::from(-15);
        let y = BigInt::from(-4);
        let z = x.clone() + y.clone();
        x += y.clone();
        assert_eq!(z, -19);
        assert_eq!(x, y);
    }

    #[test]
    fn sub() {
        let mut x: BigInt = BigInt::from(10);
        let y = BigInt::from(10);
        let z = x.clone() - y.clone();
        x -= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x = BigInt::from(101010);
        let y = BigInt::from(10);
        let z = x.clone() - y.clone();
        x -= y;
        assert_eq!(z, 101000);
        assert_eq!(z, x);

        let mut x = BigInt::from(0);
        let y = BigInt::from(0);
        let z = x.clone() - y.clone();
        x -= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

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
        assert_eq!(x, y);

        let mut x = BigInt::from(6);
        let y = BigInt::from(4);
        let z = x.clone() - y.clone();
        x -= y.clone();
        assert_eq!(z, 2);
        assert_eq!(x, y);

        let mut x = BigInt::from(-15);
        let y = BigInt::from(-4);
        let z = x.clone() - y.clone();
        x -= y.clone();
        assert_eq!(z, -11);
        assert_eq!(x, y);
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

        let mut x = BigInt::from(20100000100u64);
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
        assert_eq!(x, y);

        let mut x = BigInt::from(-15);
        let y = BigInt::from(-4);
        let z: BigInt = x.clone() * y.clone();
        x *= y.clone();
        assert_eq!(z, 60);
        assert_eq!(x, y);
    }

    #[test]
    fn div() {
        let mut x: BigInt = BigInt::from(10000);
        let y = BigInt::from(10);
        let z = x.clone() / y.clone();
        x /= y;
        assert_eq!(z, 1000);
        assert_eq!(z, x);

        let mut x = BigInt::from(101);
        let y = BigInt::from(101);
        let z = x.clone() / y.clone();
        x /= y;
        assert_eq!(z, 1);
        assert_eq!(z, x);

        let mut x = BigInt::from(0);
        let y = BigInt::from(2);
        let z = x.clone() / y.clone();
        x /= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x = BigInt::from(0);
        let y = BigInt::from(0);
        let z = x.clone() / y.clone();
        x /= y;
        assert_eq!(z, BigInt::default());
        assert_eq!(z, x);

        let mut x = BigInt::from(10000);
        let y = BigInt::from(0);
        let z = x.clone() / y.clone();
        x /= y;
        assert_eq!(z, BigInt::default());
        assert_eq!(z, x);

        let mut x = BigInt::from(20100000100u64);
        let y = BigInt::from(200);
        let z = x.clone() / y.clone();
        x /= y;
        assert_eq!(z, 100500);
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
        assert_eq!(x, y);

        let mut x = BigInt::from(-15);
        let y = BigInt::from(-5);
        let z = x.clone() / y.clone();
        x /= y.clone();
        assert_eq!(z, 3);
        assert_eq!(x, y);
    }

    #[test]
    fn reminder() {
        let mut x: BigInt = BigInt::from(10000);
        let y = BigInt::from(10);
        let z = x.clone() % y.clone();
        x %= y;
        assert_eq!(z, 0);
        assert_eq!(z, x);

        let mut x: BigInt = BigInt::from(10);
        let y = BigInt::from(0);
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
        assert_eq!(z, 4);
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
        let x = BigInt::from(17220003931i64);
        assert_eq!(format!("{x:#b}"), "0b10000000010011001000110100001011011");
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
        assert_eq!(z, 8);

        let mut x = BigInt::from(10); //1010
        let y = BigInt::from(13); //1101
        let z = x.clone() ^ y.clone(); //0111
        x ^= y;
        assert_eq!(x, z);
        assert_eq!(z, 7);

        let mut x = BigInt::from(172); //10101100
        let y = BigInt::from(223); //11011111
        let z = x.clone() ^ y.clone(); //01110011
        x ^= y;
        assert_eq!(x, z);
        assert_eq!(z, 115);

        let mut x = BigInt::from(172); //10101100
        let y = BigInt::from(1); //1
        let z = x.clone() ^ y.clone(); //10101101
        x ^= y;
        assert_eq!(x, z);
        assert_eq!(z, 173);

        let mut x = BigInt::from(173); //10101101
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
        assert_eq!(format!("{x:#X}"), "0xF");
        let x = BigInt::from(10);
        assert_eq!(format!("{x:#X}"), "0x10");
        let x = BigInt::from(172);
        assert_eq!(format!("{x:X}"), "AC");
        let x = BigInt::from(17220003931u128);
        assert_eq!(format!("{x:#X}"), "0x40264685B");

        let x = BigInt::from(4);
        assert_eq!(format!("{x:x}"), "4");
        let x = BigInt::from(16);
        assert_eq!(format!("{x:#x}"), "0xf");
        let x = BigInt::from(10);
        assert_eq!(format!("{x:#x}"), "0x10");
        let x = BigInt::from(172);
        assert_eq!(format!("{x:x}"), "ac");
        let x = BigInt::from(17220003931u128);
        assert_eq!(format!("{x:#x}"), "0x40264685b");
    }

    #[test]
    fn progtest_tests() {
        let mut a = BigInt::new();
        let mut b = BigInt::new();
        a = BigInt::from(10);
        a += BigInt::from(20);
        assert_eq!(a, 30);
        a *= BigInt::from(5);
        assert_eq!(a, 150);
        b = a.clone() + BigInt::from(3);
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

        a = BigInt::from(12345678901234567890i128);
        a += BigInt::from(-99999999999999999999i128);
        assert_eq!(a, -87654321098765432109);
        a *= BigInt::from(54321987654321987654i128);
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

        a = BigInt::from(73786976294838206464i128);
        assert_eq!(a, 73786976294838206464);
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
        assert!(a == 73786976294838206464);
        assert!(!(a != 73786976294838206464));
        assert!(a < 73786976294838206465);
        assert!(a <= 73786976294838206465);
        assert!(!(a > 73786976294838206465));
        assert!(!(a >= 73786976294838206465));
        assert!(!(a == 73786976294838206465));
        assert!(a != 73786976294838206465);
        a = BigInt::from_str("2147483648").unwrap_or(BigInt::new());
        assert!(!(a < -2147483648));
        assert!(!(a <= -2147483648));
        assert!(a > -2147483648);
        assert!(a >= -2147483648);
        assert!(!(a == -2147483648));
        assert!(a != -2147483648);
        a = BigInt::from_str("-12345678").unwrap_or(BigInt::new());
        assert!(!(a < -87654321));
        assert!(!(a <= -87654321));
        assert!(a > -87654321);
        assert!(a >= -87654321);
        assert!(!(a == -87654321));
        assert!(a != -87654321);
    }
}

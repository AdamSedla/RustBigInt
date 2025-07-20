#![allow(unused)]

use num_traits::ToPrimitive;

use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{self, Binary, Display, LowerHex, UpperHex};
use std::ops::*;
use std::str::FromStr;

#[derive(Debug)]
enum BigIntError {
    NaN,
}

impl Error for BigIntError {}

impl Display for BigIntError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Not a Number")
    }
}

#[derive(Clone, Eq, Debug)]
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

impl FromStr for BigInt {
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
                return BigInt::parse_word_digits(if positive {
                    string_of_numbers.to_string()
                } else {
                    format!("-{}", string_of_numbers)
                });
            }

            numbers.push(char.to_digit(10).unwrap().to_u8().unwrap());
        }

        if numbers.as_slice() == &[0] {
            positive = true;
        }

        Ok(BigInt { positive, numbers })
    }
}

macro_rules! from_int {
    ($($t:ty),*)=>{
        $(
            impl From<$t> for BigInt{
                fn from(mut original_number: $t) -> Self {

                //zero edgecase
                if original_number == 0 {
                    return BigInt::default();
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
                BigInt {positive, numbers}

                }
            }
        )*
    }
}

macro_rules! from_uint {
    ($($t:ty),*)=>{
        $(
            impl From<$t> for BigInt{
                fn from(mut original_number: $t) -> Self {

                //zero edgecase
                if original_number == 0 {
                    return BigInt::default();
                }

                let mut numbers = Vec::new();

                //transformation of digits
                while original_number != 0{
                    numbers.insert(0, (original_number % 10).to_u8().unwrap());
                    original_number /= 10;
                }

                //return value
                BigInt {
                    positive: true, numbers}

                }
            }
        )*
    }
}

from_int!(i8, i16, i32, i64, i128);
from_uint!(u8, u16, u32, u64, u128);

impl Display for BigInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if !self.positive {
            write!(f, "-")?;
        }

        for digit in &self.numbers {
            write!(f, "{digit}")?;
        }
        Ok(())
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

impl PartialEq for BigInt {
    fn eq(&self, other: &BigInt) -> bool {
        if self.positive != other.positive {
            false
        } else {
            self.numbers == other.numbers
        }
    }
}

macro_rules! eq_with_int {
    ($($t:ty),*) => {
        $(
            impl PartialEq<$t> for BigInt{
                fn eq(&self, other: &$t) -> bool {
                    self == &BigInt::from(*other)
                }
            }
        )*
    };
}

eq_with_int!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

impl PartialEq<&str> for BigInt {
    fn eq(&self, other: &&str) -> bool {
        match BigInt::from_str(other) {
            Ok(right) => self == &right,
            Err(_) => false,
        }
    }
}

impl Neg for BigInt {
    type Output = Self;
    fn neg(mut self) -> Self::Output {
        self.positive = !self.positive;
        self
    }
}

impl PartialOrd for BigInt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self == other {
            return Some(Ordering::Equal);
        }

        let mut greater = false;

        //check +/-
        if self.positive != other.positive {
            greater = self.positive;
        }
        //check length
        else if self.numbers.len() != other.numbers.len() {
            if self.positive {
                greater = self.numbers.len() > other.numbers.len();
            } else {
                greater = self.numbers.len() < other.numbers.len();
            }
        }
        //compare digits
        else {
            let mut numbers_iterator = self.numbers.iter().zip(other.numbers.iter());

            for (left, right) in numbers_iterator {
                if left != right {
                    greater = left > right;
                }
            }
        }

        //return value
        return if greater {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        };
    }
}

impl PartialOrd<i128> for BigInt {
    fn partial_cmp(&self, other: &i128) -> Option<Ordering> {
        todo!()
    }
}

impl PartialOrd<&str> for BigInt {
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
            fin_str = BigInt::number_to_word(*nmr_iter.next().unwrap_or(&0));
        }

        //print all digits
        for num in nmr_iter {
            fin_str = format!("{} {}", fin_str, BigInt::number_to_word(*num));
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

    fn parse_word_digits(string_of_numbers: String) -> Result<BigInt, BigIntError> {
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
            numbers.push(BigInt::word_to_number(&word)?);
        }

        //additional check
        if numbers.is_empty() {
            return Err(BigIntError::NaN);
        }

        Ok(BigInt { positive, numbers })
    }
}

#[cfg(test)]
mod tests;

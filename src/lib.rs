use num_traits::{Pow, ToPrimitive, Zero};

use std::cmp::Ordering;
use std::error::Error;
use std::fmt::{self, Alignment, Binary, Display, LowerHex, UpperHex};
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
    fn from_str(string_of_numbers: &str) -> Result<Self, Self::Err> {
        let original = string_of_numbers;

        //empty string edgecaase
        if string_of_numbers.is_empty() {
            return Err(BigIntError::NaN);
        }

        let mut positive = !(string_of_numbers.starts_with('-'));
        let mut numbers: Vec<u8> = Vec::new();

        //if negative - remove '-'
        let string_of_numbers = &string_of_numbers[!positive as usize..];

        for char in string_of_numbers.chars() {
            if !char.is_ascii_digit() {
                return BigInt::parse_word_digits(if positive {
                    string_of_numbers
                } else {
                    original
                });
            }

            numbers.push(char.to_digit(10).unwrap().to_u8().unwrap());
        }

        if numbers.as_slice() == [0] {
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
                    numbers.push((original_number % 10).to_u8().unwrap());
                    original_number /= 10;
                }

                numbers.reverse();

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
                    numbers.push((original_number % 10).to_u8().unwrap());
                    original_number /= 10;
                }

                numbers.reverse();

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
        let mut output = String::new();

        if !self.positive {
            output.push('-');
        }
        if f.sign_plus() && self.positive {
            output.push('+');
        }

        for digit in &self.numbers {
            output.push_str(&digit.to_string());
        }

        let mut right_fill = true;

        if f.width().is_some() {
            while output.len() < f.width().unwrap() {
                match f.align() {
                    Some(Alignment::Left) => output.push(f.fill()),
                    Some(Alignment::Center) => {
                        if right_fill {
                            output.push(f.fill());
                            right_fill = false;
                        } else {
                            output.insert(0, f.fill());
                            right_fill = true;
                        }
                    }
                    Some(Alignment::Right) => output.insert(0, f.fill()),
                    _ => output.insert(0, f.fill()),
                }
            }
        }

        write!(f, "{output}")?;

        Ok(())
    }
}

impl Binary for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let binary = self.to_binary();
        let mut output = String::new();

        if f.alternate() {
            output.push_str("0b");
        }

        for bit in binary {
            if bit {
                output.push('1');
            } else {
                output.push('0');
            }
        }

        let mut right_fill = true;

        if f.width().is_some() {
            while output.len() < f.width().unwrap() {
                match f.align() {
                    Some(Alignment::Left) => output.push(f.fill()),
                    Some(Alignment::Center) => {
                        if right_fill {
                            output.push(f.fill());
                            right_fill = false;
                        } else {
                            output.insert(0, f.fill());
                            right_fill = true;
                        }
                    }
                    Some(Alignment::Right) => output.insert(0, f.fill()),
                    _ => output.insert(0, f.fill()),
                }
            }
        }

        write!(f, "{output}")?;

        Ok(())
    }
}

impl UpperHex for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = self.create_hexa_string(f, true);

        write!(f, "{output}")?;

        Ok(())
    }
}

impl LowerHex for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output = self.create_hexa_string(f, false);

        write!(f, "{output}")?;

        Ok(())
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
        if self != 0 {
            self.positive = !self.positive;
        }
        self
    }
}

macro_rules! partial_ord_intieger {
    ($($t:ty),*) => {
        $(
            impl PartialOrd<$t> for BigInt{
                fn partial_cmp(&self, other: &$t) -> Option<Ordering> {
                    return self.partial_cmp(&BigInt::from(*other));
                 }
            }
        )*
    };
}

partial_ord_intieger!(u8, u16, u32, u64, u128, i8, i16, i32, i64, i128);

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
            let numbers_iterator = self.numbers.iter().zip(other.numbers.iter());

            for (left, right) in numbers_iterator {
                if left != right {
                    greater = left > right;
                    break;
                }
            }

            if !self.positive {
                greater = !greater;
            }
        }

        //return value
        if greater {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Less)
        }
    }
}

impl PartialOrd<&str> for BigInt {
    fn partial_cmp(&self, other: &&str) -> Option<Ordering> {
        let other = BigInt::from_str(other);

        if other.is_err() {
            return None;
        }

        self.partial_cmp(&other.unwrap())
    }
}

fn create_numbers_set(left: BigInt, right: BigInt) -> (Vec<u8>, Vec<u8>) {
    let (mut longer, mut shorter) = {
        if left.numbers.len() >= right.numbers.len() {
            (left.numbers, right.numbers)
        } else {
            (right.numbers, left.numbers)
        }
    };

    longer.reverse();
    shorter.reverse();

    shorter.extend(vec![0; longer.len() - shorter.len()]);

    (shorter, longer)
}

impl<T> Add<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn add(mut self, rhs: T) -> Self::Output {
        let mut right: BigInt = rhs.into();

        // X + 0 edgecase
        if self == 0 {
            return right;
        }
        if right == 0 {
            return self;
        }

        // negative numbers edgecases
        if !self.positive && right.positive {
            // -X + Y => Y - X => -(X - Y)
            self.positive = true;
            return right - self;
        } else if self.positive && !right.positive {
            //X - Y
            right.positive = true;
            return self - right;
        }

        let mut result = BigInt {
            positive: self.positive,
            numbers: vec![],
        };

        let (longer, shorter) = create_numbers_set(self, right);

        let numbers_set = longer.iter().zip(shorter.iter());

        let mut new_number;
        let mut carry: u8 = 0;

        for number_set in numbers_set {
            new_number = 0;

            new_number += number_set.0;
            new_number += number_set.1;
            new_number += carry;

            let new_digit = new_number % 10;
            carry = new_number / 10;

            result.numbers.push(new_digit);
        }

        if carry != 0 {
            result.numbers.push(carry);
        }

        result.numbers.reverse();

        result
    }
}

impl<T> AddAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn add_assign(&mut self, rhs: T) {
        *self = self.clone().add(rhs);
    }
}

impl<T> Sub<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;

    fn sub(mut self, rhs: T) -> Self::Output {
        let mut right: BigInt = rhs.into();

        // 0 - X edgecase
        if self == 0 {
            return -right;
        }
        if right == 0 {
            return self;
        }
        if right == self {
            return 0.into();
        }

        // negative numbers edgecases
        if !self.positive && right.positive {
            // -X - Y => -(X+Y)
            self.positive = true;
            return -(self + right);
        } else if self.positive && !right.positive {
            //X - (-Y)
            right.positive = true;
            return self + right;
        } else if !self.positive && !right.positive {
            // -X - (-Y) => -X + Y => Y - X
            self.positive = true;
            right.positive = true;
            return right - self;
        }

        if self < right {
            return -(right - self);
        }

        let mut result: BigInt = BigInt {
            positive: self >= right,
            numbers: vec![],
        };

        self.numbers.reverse();
        right.numbers.reverse();

        if self.numbers.len() >= right.numbers.len() {
            right
                .numbers
                .extend(vec![0; self.numbers.len() - right.numbers.len()]);
        } else {
            self.numbers
                .extend(vec![0; right.numbers.len() - self.numbers.len()]);
        }

        let numbers_set = self.numbers.iter().zip(right.numbers.iter());

        let mut new_number: i8;
        let mut carry = 0;

        for number_set in numbers_set {
            new_number = 0;

            new_number += *number_set.0 as i8;
            new_number -= *number_set.1 as i8;
            new_number -= carry;

            if new_number < 0 {
                new_number += 10;
                carry = 1;
            } else {
                carry = 0;
            }

            result.numbers.push(new_number as u8);
        }

        result.numbers.reverse();

        while result.numbers.first().unwrap_or(&1).is_zero() {
            result.numbers.remove(0);
        }

        result
    }
}

impl<T> SubAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn sub_assign(&mut self, rhs: T) {
        *self = self.clone().sub(rhs);
    }
}

impl<T> Mul<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        let right: BigInt = rhs.into();

        //X * 0 edgecase
        if self == 0 || right == 0 {
            return BigInt::default();
        }

        if self == 1 {
            return right;
        }
        if right == 1 {
            return self;
        }

        if self == -1 {
            return -right;
        }
        if right == -1 {
            return -self;
        }

        let mut result = BigInt::default();
        let mut sub_total;
        let mut new_digit;
        let mut carry;

        for (position, &left) in self.numbers.iter().rev().enumerate() {
            // *0 edgecase
            if left == 0 {
                continue;
            }

            sub_total = BigInt {
                positive: true,
                numbers: vec![],
            };

            carry = 0;

            //right digit loop
            for &right in right.numbers.iter().rev() {
                new_digit = left * right;
                new_digit += carry;
                sub_total.numbers.push(new_digit % 10);
                carry = new_digit / 10;
            }

            if !carry.is_zero() {
                sub_total.numbers.push(carry);
            }

            sub_total.numbers.reverse();

            //zero offset
            sub_total.numbers.extend(vec![0; position]);

            result += sub_total;
        }

        result.positive = self.positive == right.positive;
        result
    }
}

impl<T> MulAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn mul_assign(&mut self, rhs: T) {
        *self = self.clone().mul(rhs);
    }
}

impl<T> Div<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn div(mut self, rhs: T) -> Self::Output {
        let mut right: BigInt = rhs.into();

        if right == 0 {
            panic!("division by zero!");
        }
        if self == 0 {
            return BigInt::default();
        }
        if right == 1 {
            return self;
        }
        if self == 1 {
            if right == 1 {
                return 1.into();
            }
            return BigInt::default();
        }

        let mut result = BigInt {
            positive: self.positive == right.positive,
            numbers: vec![],
        };

        self.positive = true;
        right.positive = true;

        let mut left = BigInt {
            positive: true,
            numbers: vec![],
        };

        let mut new_digit;

        for digit in self.numbers.iter() {
            left.numbers.push(*digit);
            new_digit = 0;

            while left >= right {
                left -= right.clone();
                new_digit += 1;
            }

            result.numbers.push(new_digit);

            if left.numbers == vec![0] {
                left.numbers.clear();
            }
        }

        while result.numbers.first().unwrap_or(&1).is_zero() {
            result.numbers.remove(0);
        }

        if result.numbers.is_empty() {
            return BigInt::default();
        }

        result
    }
}

impl<T> DivAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn div_assign(&mut self, rhs: T) {
        *self = self.clone().div(rhs);
    }
}

impl<T> Rem<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn rem(self, rhs: T) -> Self::Output {
        let right: BigInt = rhs.into();

        if right == 0 {
            panic!("division by zero!");
        }
        if self == 0 || right == 1 {
            return 0.into();
        }

        let divide_result = self.clone() / right.clone();

        self - divide_result * right
    }
}

impl<T> RemAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn rem_assign(&mut self, rhs: T) {
        *self = self.clone().rem(rhs);
    }
}

impl<T> Pow<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;

    fn pow(self, rhs: T) -> Self::Output {
        let mut right: BigInt = rhs.into();

        if self == 0 || self == 1 || right == 1 {
            return self;
        }
        if right == 0 {
            return 1.into();
        }
        if !right.positive {
            if self == 1 {
                return 1.into();
            }
            return 0.into();
        }

        let mut result: BigInt = 1.into();

        while right != 0 {
            result *= self.clone();

            right -= 1;
        }

        result
    }
}

impl<T> BitAnd<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn bitand(self, rhs: T) -> Self::Output {
        let right: BigInt = rhs.into();

        let mut left = self.to_binary();
        let mut right = right.to_binary();
        let mut result = vec![];

        left.reverse();
        right.reverse();

        let (longer, mut shorter) = if left.len() > right.len() {
            (left, right)
        } else {
            (right, left)
        };

        shorter.extend(vec![false; longer.len() - shorter.len()]);

        let binary_set = longer.iter().zip(shorter.iter());

        for (left, right) in binary_set {
            result.push(left & right);
        }

        result.reverse();

        BigInt::from_binary(result)
    }
}

impl<T> BitAndAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn bitand_assign(&mut self, rhs: T) {
        *self = self.clone().bitand(rhs);
    }
}

impl<T> BitOr<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn bitor(self, rhs: T) -> Self::Output {
        let right: BigInt = rhs.into();

        let mut left = self.to_binary();
        let mut right = right.to_binary();
        let mut result = vec![];

        left.reverse();
        right.reverse();

        let (longer, mut shorter) = if left.len() > right.len() {
            (left, right)
        } else {
            (right, left)
        };

        shorter.extend(vec![false; longer.len() - shorter.len()]);

        let binary_set = longer.iter().zip(shorter.iter());

        for (left, right) in binary_set {
            result.push(left | right);
        }

        result.reverse();

        BigInt::from_binary(result)
    }
}

impl<T> BitOrAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn bitor_assign(&mut self, rhs: T) {
        *self = self.clone().bitor(rhs);
    }
}

impl<T> BitXor<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn bitxor(self, rhs: T) -> Self::Output {
        let right: BigInt = rhs.into();

        let mut left = self.to_binary();
        let mut right = right.to_binary();
        let mut result = vec![];

        left.reverse();
        right.reverse();

        let (longer, mut shorter) = if left.len() > right.len() {
            (left, right)
        } else {
            (right, left)
        };

        shorter.extend(vec![false; longer.len() - shorter.len()]);

        let binary_set = longer.iter().zip(shorter.iter());

        for (left, right) in binary_set {
            result.push(left ^ right);
        }

        result.reverse();

        BigInt::from_binary(result)
    }
}

impl<T> BitXorAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn bitxor_assign(&mut self, rhs: T) {
        *self = self.clone().bitxor(rhs);
    }
}

impl<T> Shl<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn shl(self, rhs: T) -> Self::Output {
        let right: BigInt = rhs.into();

        if right == 0 {
            return self;
        }

        let base: BigInt = 2.into();

        self * base.pow(right)
    }
}

impl<T> ShlAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn shl_assign(&mut self, rhs: T) {
        *self = self.clone().shl(rhs);
    }
}

impl<T> Shr<T> for BigInt
where
    T: Into<BigInt>,
{
    type Output = Self;
    fn shr(self, rhs: T) -> Self::Output {
        let right: BigInt = rhs.into();

        if right == 0 {
            return self;
        }

        let base: BigInt = 2.into();

        self / base.pow(right)
    }
}

impl<T> ShrAssign<T> for BigInt
where
    T: Into<BigInt>,
{
    fn shr_assign(&mut self, rhs: T) {
        *self = self.clone().shr(rhs);
    }
}

impl BigInt {
    pub fn new() -> BigInt {
        BigInt::default()
    }

    pub fn to_words(&self) -> String {
        let mut final_string: String;
        let mut number_iterator = self.numbers.iter();

        //print minus or first digit
        if !self.positive {
            final_string = "minus".to_string();
        } else {
            final_string =
                BigInt::number_to_word(*number_iterator.next().unwrap_or(&0)).to_string();
        }

        //print all digits
        for num in number_iterator {
            final_string = format!("{} {}", final_string, BigInt::number_to_word(*num));
        }

        final_string
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

    fn number_to_word(number: u8) -> &'static str {
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
    }

    fn parse_word_digits(string_of_numbers: &str) -> Result<BigInt, BigIntError> {
        let mut parsed: Vec<String> = string_of_numbers
            .split_whitespace()
            .map(str::to_lowercase)
            .collect();

        let positive;

        if matches!(parsed.first().map(String::as_str), Some("-" | "minus")) {
            positive = false;
            parsed.remove(0);
        } else {
            positive = true;
        }

        let numbers: Result<Vec<u8>, _> = parsed
            .iter()
            .map(String::as_str)
            .map(BigInt::word_to_number)
            .collect();
        let numbers = numbers?;

        //additional check
        if numbers.is_empty() {
            return Err(BigIntError::NaN);
        }

        Ok(BigInt { positive, numbers })
    }

    fn is_even(&self) -> bool {
        (*self.numbers.last().unwrap() % 2).is_zero()
    }

    fn to_binary(&self) -> Vec<bool> {
        if *self == 0 {
            return vec![false];
        }

        let mut final_vec = vec![];

        let mut number = self.clone();

        let positive = number.positive;
        number.positive = true;

        while number != 0 {
            if number.is_even() {
                final_vec.push(false);
            } else {
                final_vec.push(true);
            }
            number /= 2;
        }

        if positive {
            final_vec.push(false);
        } else {
            final_vec.push(true);
        }

        final_vec.reverse();

        final_vec
    }

    fn from_binary(binary: Vec<bool>) -> BigInt {
        if binary.is_empty() {
            return BigInt::default();
        }

        let positive = !binary.first().unwrap();

        let mut result = BigInt::default();

        for (position, bit) in binary.iter().skip(1).rev().enumerate() {
            if *bit {
                result += 2.pow(position);
            }
        }

        result.positive = positive;

        result
    }

    fn number_to_hexa(number: BigInt) -> char {
        if number == 0 {
            '0'
        } else if number == 1 {
            '1'
        } else if number == 2 {
            '2'
        } else if number == 3 {
            '3'
        } else if number == 4 {
            '4'
        } else if number == 5 {
            '5'
        } else if number == 6 {
            '6'
        } else if number == 7 {
            '7'
        } else if number == 8 {
            '8'
        } else if number == 9 {
            '9'
        } else if number == 10 {
            'A'
        } else if number == 11 {
            'B'
        } else if number == 12 {
            'C'
        } else if number == 13 {
            'D'
        } else if number == 14 {
            'E'
        } else if number == 15 {
            'F'
        } else {
            '0'
        }
    }

    fn to_hexa(&self) -> Vec<char> {
        let mut final_vec = vec![];
        let mut number = self.clone();

        number.positive = true;

        while number > 0 {
            final_vec.push(BigInt::number_to_hexa(number.clone() % 16));
            number /= 16;
        }

        final_vec.reverse();

        final_vec
    }

    fn create_hexa_string(&self, f: &mut fmt::Formatter<'_>, uppercase: bool) -> String {
        let hexa = self.to_hexa();
        let mut output = String::new();

        if !self.positive {
            output.push('-');
        }

        if f.alternate() {
            output.push_str("0x");
        }

        for hex in hexa {
            output.push(if uppercase {
                hex.to_ascii_uppercase()
            } else {
                hex.to_ascii_lowercase()
            })
        }

        let mut right_fill = true;

        if f.width().is_some() {
            while output.len() < f.width().unwrap() {
                match f.align() {
                    Some(Alignment::Left) => output.push(f.fill()),
                    Some(Alignment::Center) => {
                        if right_fill {
                            output.push(f.fill());
                            right_fill = false;
                        } else {
                            output.insert(0, f.fill());
                            right_fill = true;
                        }
                    }
                    Some(Alignment::Right) => output.insert(0, f.fill()),
                    _ => output.insert(0, f.fill()),
                }
            }
        }

        output
    }
}

#[cfg(test)]
mod tests;

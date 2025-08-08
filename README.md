# RustBigInt

BigInt written in Rust.

## Description

My own implementation of BigInt (type of integer with unlimited size) as project to learn Rust.

## Example

```rs
use RustBigInt::BigInt;

//from
let x: BigInt = 66.into();
let y: BigInt = BigInt::from(34);
let z: BigInt = BigInt::from_str("999999").unwrap();

//tryInto
let x: BigInt = 32.into();
let x_num: i32 = x.try_into.unwrap();
assert_eq!(32, x_num);

let y: BigInt = i16:MAX.into();
let y_num: i8 = y.try_into();
assert!(y_num.is_err());

//Display
let x: BigInt = BigInt::from(1003);
assert_eq!(format!("Number is: {x}"), "Number is: 1003");
assert_eq!(format!("Number is: {x:0^11}"), "Number is: 00010030000");
assert_eq!(format!("Number is: {x:0>10}"), "Number is: 0000001003");
assert_eq!(format!("Number is: {x:0<10}"), "Number is: 1003000000");
assert_eq!(format!("Number is: {x:->10}"), "Number is: ------1003");
assert_eq!(format!("Number is: {x:9}"), "Number is:      1003");

//Binary
let x: BigInt = BigInt::from(11);
assert_eq!(format!("{x:#b}"), "0b1011");
assert_eq!(format!("{x:b}"), "1011");
assert_eq!(format!("{x:0^11b}"), "00010110000");


//Hexadecimal
let x: BigInt = BigInt::from(11);
assert_eq!(format!("{x:x}"), "a");
assert_eq!(format!("{x:X}"), "A");
assert_eq!(format!("{x:#X}"), "0xA");
assert_eq!(format!("{x:0^11X}"), "00000A00000");

let x: BigInt = BigInt::from(20);
assert_eq!(x.to_words(), "two zero");

//Math operations
let x: BigInt = BigInt::from(66);
let y: BigInt = BigInt::from(34);
assert!(x != y);
assert_eq!(x + y, 100);
assert_eq!(x - y, 32);
assert_eq!(x * y, 2244);
assert_eq!(x / y, 1);
assert_eq!(x % y, 32);
assert_eq!(x.pow(2), 4356);

//Binary operations
let x: BigInt = 11; //Ob1011
let y: BigInt = 6; //Ob0110
assert_eq!(x & y, 2); //0b0010
assert_eq!(x | y, 15); //0b1111
assert_eq!(x ^ y, 13); //0b1101
assert_eq!(x >> 2, 2) //0b10
assert_eq!(x << 2, 44) //0b101100
```

## All implemented traits and functions

<details>
<summary>From and Into traits</summary>

- Default
- New
- FromStr
- From<i8, i16, i32, i64, i128, u8, u16, u32, u64, u128>
- TryInto<i8, i16, i32, i64, i128, u8, u16, u32, u64, u128>

</details>

<details>
<summary> Display traits </summary>

- Display
- Binary
- UpperHex
- LowerHex
- to_words
</details>

<details>
<summary> Comparing traits </summary>

- PartialEq<BigInt, &str, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128>
- PartialOrd<BigInt, &str, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128>
</details>

<details>
<summary> Math traits </summary>

- Neg (-)
- Add (+)
- AddAssign(+=)
- Sub (-)
- SubAssign (-=)
- Mul (*)
- MulAssign (*=)
- Div (/)
- DivAssign (/=)
- Rem (%)
- RemAssign (%=)
- Pow
</details>


<details>
<summary> Bit operation traits </summary>

- BitAnd (&)
- BitAndAssign (&=)
- BitOr (|)
- BitOrAssign (|=)
- BitXor (^)
- BitXorAssign (^=)
- Shl (<<)
- ShlAssign (<<=)
- Shr (>>)
- ShrAssign (>>=)
</details>


## Acknowledgments

Special thanks to [magnusi](https://github.com/luciusmagn) for leading me throughout this project.

## License

Project is licensed under the MIT license. See the LICENSE file for more info.

Adam Sedláček, 2025 (C)
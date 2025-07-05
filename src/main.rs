struct BigInt {
    positive: bool,
    numbers: Vec<i32>,
}

impl Default for BigInt {
    fn default() -> Self {
        BigInt {
            positive: true,
            numbers: [0].to_vec(),
        }
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
}

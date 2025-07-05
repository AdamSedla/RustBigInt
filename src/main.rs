mod rust_big_int {
    struct Internals {
        positive: bool,
        numbers: Vec<i32>,
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

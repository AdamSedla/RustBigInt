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
}

fn main() {}

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
}

fn main() {}

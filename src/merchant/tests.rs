

#[cfg(test)]
mod tests {
    use std::ops::Sub;

    use candid::Nat;

    #[test]
    fn test_nat_arithmetic() {
        let a = Nat::from(1000);
        let b = Nat::from(900);
        let c = Nat::from(1);

        let res = a.sub(b).sub(c);

        let expected = Nat::from(99);

        assert_eq!(res, expected);
    }
}
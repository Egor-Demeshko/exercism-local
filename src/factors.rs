pub fn factors(n: u64) -> Vec<u64> {
    if n == 1 {
        return vec![];
    }
    if n == 2 {
        return vec![2];
    }
    let mut factors = Vec::new();
    let mut mut_n: u64 = n.clone();

    while mut_n % 2 == 0 {
        mut_n /= 2;
        factors.push(2);
    }

    let mut devider: u64 = 3;
    while devider <= mut_n && 1 < mut_n {
        if mut_n % devider == 0 {
            factors.push(devider.clone());
            mut_n /= devider;
        } else {
            devider += 2;
        }
    }

    factors.sort_unstable();
    factors
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[ignore]
    fn no_factors() {
        let factors = factors(1);

        let expected = [];

        assert_eq!(factors, expected);
    }

    #[test]
    #[ignore]
    fn prime_number() {
        let factors = factors(2);

        let expected = [2];

        assert_eq!(factors, expected);
    }

    #[test]
    #[ignore]
    fn another_prime_number() {
        let factors = factors(3);

        let expected = [3];

        assert_eq!(factors, expected);
    }

    #[test]
    #[ignore]
    fn square_of_a_prime() {
        let factors = factors(9);

        let expected = [3, 3];

        assert_eq!(factors, expected);
    }

    #[test]
    fn factors_include_a_large_prime() {
        let factors = factors(93_819_012_551);
        let expected = [11, 9_539, 894_119];
        assert_eq!(factors, expected);
    }
}

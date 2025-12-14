use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set: HashSet<u32> = HashSet::new();

    for &factor in factors {
        if factor == 0 || factor > limit {
            continue;
        }

        let mut acc: u32 = factor;
        while acc < limit {
            set.insert(acc);
            acc += factor;
        }
    }

    set.iter().sum()
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn no_multiples_within_limit() {
        let factors = &[3, 5];
        let limit = 1;
        let output = sum_of_multiples(limit, factors);
        let expected = 0;
        assert_eq!(output, expected);
    }

    #[test]
    fn one_factor_has_multiples_within_limit() {
        let factors = &[3, 5];
        let limit = 4;
        let output = sum_of_multiples(limit, factors);
        let expected = 3;
        assert_eq!(output, expected);
    }

    #[test]
    fn more_than_one_multiple_within_limit() {
        let factors = &[3];
        let limit = 7;
        let output = sum_of_multiples(limit, factors);
        let expected = 9;
        assert_eq!(output, expected);
    }

    #[test]
    fn more_than_one_factor_with_multiples_within_limit() {
        let factors = &[3, 5];
        let limit = 10;
        let output = sum_of_multiples(limit, factors);
        let expected = 23;
        assert_eq!(output, expected);
    }

    #[test]
    fn solutions_using_include_exclude_must_extend_to_cardinality_greater_than_3() {
        let factors = &[2, 3, 5, 7, 11];
        let limit = 10_000;
        let output = sum_of_multiples(limit, factors);
        let expected = 39_614_537;
        assert_eq!(output, expected);
    }
}

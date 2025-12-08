pub fn square_of_sum(n: u32) -> u32 {
    (n * (n + 1) / 2).pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    n * (n + 1) * (2 * n + 1) / 6
}

pub fn difference(n: u32) -> u32 {
    sum_of_squares(n).abs_diff(square_of_sum(n))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn square_of_sum_1() {
        assert_eq!(1, square_of_sum(1));
    }

    #[test]
    fn square_of_sum_5() {
        assert_eq!(225, square_of_sum(5));
    }

    #[test]
    fn square_of_sum_100() {
        assert_eq!(25_502_500, square_of_sum(100));
    }
}

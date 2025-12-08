pub fn is_armstrong_number(num: u32) -> bool {
    let num_str = format!("{num}");
    let power = num_str.len() as u32;
    let mut sum = 0;

    for digit in num_str.chars() {
        let digit_u32 = digit.to_digit(10).unwrap();
        sum += digit_u32.pow(power);
    }

    num == sum
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zero_is_an_armstrong_number() {
        assert!(is_armstrong_number(0))
    }

    #[test]
    fn single_digit_numbers_are_armstrong_numbers() {
        assert!(is_armstrong_number(5))
    }

    #[test]
    fn there_are_no_two_digit_armstrong_numbers() {
        assert!(!is_armstrong_number(10))
    }

    #[test]
    fn three_digit_number_that_is_an_armstrong_number() {
        assert!(is_armstrong_number(153))
    }

    #[test]
    fn three_digit_number_that_is_not_an_armstrong_number() {
        assert!(!is_armstrong_number(100))
    }

    #[test]
    fn four_digit_number_that_is_an_armstrong_number() {
        assert!(is_armstrong_number(9_474))
    }
}

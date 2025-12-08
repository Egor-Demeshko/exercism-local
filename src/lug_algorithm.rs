use std::collections::HashSet;

pub fn is_valid(number: &str) -> bool {
    if number.is_empty() || number.len() == 1 {
        return false;
    }

    let str = format_string(number, "0123456789 ");
    if str.is_empty() || str == "0" {
        return false;
    }

    let mut counter: u32 = 1;
    let mut sum: u64 = 0;
    for digit in str.chars().rev() {
        dbg!(digit);
        let mut digit_u16 = digit.to_digit(10).unwrap_or_default() as u16;
        if counter % 2 == 0 {
            digit_u16 *= 2;
            if digit_u16 > 9 {
                digit_u16 -= 9;
            }
        }
        dbg!(digit_u16);
        sum += digit_u16 as u64;
        counter += 1;
    }

    if sum % 10 == 0 { true } else { false }
}

fn format_string(str: &str, valid_chars: &str) -> String {
    let mut set: HashSet<char> = HashSet::new();
    let mut new_str = String::new();

    for valid_char in valid_chars.chars() {
        set.insert(valid_char);
    }

    for char in str.chars() {
        if set.contains(&char) {
            new_str.push(char);
        } else {
            return String::new();
        }
    }

    new_str.replace(" ", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_digit_strings_can_not_be_valid() {
        assert!(!is_valid("1"));
    }

    #[test]
    fn a_single_zero_is_invalid() {
        assert!(!is_valid("0"));
    }

    #[test]
    fn a_simple_valid_sin_that_remains_valid_if_reversed() {
        assert!(is_valid("059"));
    }

    #[test]
    fn a_simple_valid_sin_that_becomes_invalid_if_reversed() {
        assert!(is_valid("59"));
    }

    #[test]
    fn a_valid_canadian_sin() {
        assert!(is_valid("055 444 285"));
    }

    #[test]
    fn invalid_canadian_sin() {
        assert!(!is_valid("055 444 286"));
    }

    #[test]
    fn invalid_credit_card() {
        assert!(!is_valid("8273 1232 7352 0569"));
    }

    #[test]
    fn invalid_long_number_with_an_even_remainder() {
        assert!(!is_valid("1 2345 6789 1234 5678 9012"));
    }

    #[test]
    fn invalid_long_number_with_a_remainder_divisible_by_5() {
        assert!(!is_valid("1 2345 6789 1234 5678 9013"));
    }

    #[test]
    fn valid_number_with_an_even_number_of_digits() {
        assert!(is_valid("095 245 88"));
    }

    #[test]
    fn valid_number_with_an_odd_number_of_spaces() {
        assert!(is_valid("234 567 891 234"));
    }

    #[test]
    fn valid_strings_with_a_non_digit_added_at_the_end_become_invalid() {
        assert!(!is_valid("059a"));
    }

    #[test]
    fn valid_strings_with_punctuation_included_become_invalid() {
        assert!(!is_valid("055-444-285"));
    }

    #[test]
    fn valid_strings_with_symbols_included_become_invalid() {
        assert!(!is_valid("055# 444$ 285"));
    }

    #[test]
    fn single_zero_with_space_is_invalid() {
        assert!(!is_valid(" 0"));
    }

    #[test]
    fn more_than_a_single_zero_is_valid() {
        assert!(is_valid("0000 0"));
    }

    #[test]
    fn input_digit_9_is_correctly_converted_to_output_digit_9() {
        assert!(is_valid("091"));
    }

    #[test]
    fn very_long_input_is_valid() {
        assert!(is_valid("9999999999 9999999999 9999999999 9999999999"));
    }
}

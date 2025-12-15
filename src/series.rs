pub fn series(digits: &str, len: usize) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    if digits.len() < len {
        return result;
    }

    let mut e: usize = len;
    while e <= digits.len() {
        result.push(String::from(&digits[(e - len)..e]));
        e += 1;
    }

    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn slices_of_one_from_one() {
        let input = "1";
        let length = 1;
        let output = series(input, length);
        let expected = &["1"];
        assert_eq!(output, expected);
    }

    #[test]
    fn slices_of_one_from_two() {
        let input = "12";

        let length = 1;

        let output = series(input, length);

        let expected = &["1", "2"];

        assert_eq!(output, expected);
    }

    #[test]
    fn slices_of_two() {
        let input = "35";
        let length = 2;
        let output = series(input, length);
        let expected = &["35"];
        assert_eq!(output, expected);
    }

    #[test]
    fn slices_of_two_overlap() {
        let input = "9142";
        let length = 2;
        let output = series(input, length);
        let expected = &["91", "14", "42"];
        assert_eq!(output, expected);
    }
}

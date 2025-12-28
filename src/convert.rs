#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    if from_base < 2 {
        return Err(Error::InvalidInputBase);
    }
    if to_base < 2 {
        return Err(Error::InvalidOutputBase);
    }

    let mut number_10: u32 = 0;
    let max = number.len().saturating_sub(1);

    for (i, &digit) in number.iter().enumerate() {
        if digit >= from_base {
            return Err(Error::InvalidDigit(digit));
        }
        let position = max - i;
        number_10 += digit * from_base.pow(position as u32);
    }

    Ok(transform_to_base(number_10, to_base))
}

fn transform_to_base(number: u32, to_base: u32) -> Vec<u32> {
    let mut collected: Vec<u32> = Vec::new();
    let mut rest: u32 = number;

    loop {
        if rest < to_base {
            collected.push(rest);
            collected.reverse();
            return collected;
        }

        collected.push(rest % to_base);
        rest /= to_base;
    }
}

mod test {
    use super::*;

    #[test]
    fn single_bit_one_to_decimal() {
        let input_base = 2;
        let input_digits = &[1];
        let output_base = 10;
        let output_digits = vec![1];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }

    #[test]
    fn binary_to_single_decimal() {
        let input_base = 2;
        let input_digits = &[1, 0, 1];
        let output_base = 10;
        let output_digits = vec![5];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }

    #[test]
    fn single_decimal_to_binary() {
        let input_base = 10;
        let input_digits = &[5];
        let output_base = 2;
        let output_digits = vec![1, 0, 1];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }

    #[test]
    fn binary_to_multiple_decimal() {
        let input_base = 2;
        let input_digits = &[1, 0, 1, 0, 1, 0];
        let output_base = 10;
        let output_digits = vec![4, 2];
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Ok(output_digits)
        );
    }
    #[test]
    fn invalid_positive_digit() {
        let input_base = 2;
        let input_digits = &[1, 2, 1, 0, 1, 0];
        let output_base = 10;
        assert_eq!(
            convert(input_digits, input_base, output_base),
            Err(Error::InvalidDigit(2))
        );
    }
}

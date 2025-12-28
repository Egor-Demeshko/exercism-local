pub fn egg_count(display_value: u32) -> usize {
    let mut value = display_value;
    let mut eggs_number = 0;

    if value < 2 {
        return eggs_number;
    }

    loop {
        if value == 1 {
            eggs_number += 1;
            break eggs_number;
        }

        if value % 2 == 1 {
            eggs_number += 1;
        }

        value /= 2
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0_eggs() {
        let input = 0;
        let output = egg_count(input);
        let expected = 0;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_1_egg() {
        let input = 16;
        let output = egg_count(input);
        let expected = 1;
        assert_eq!(output, expected);
    }

    #[test]
    fn test_4_eggs() {
        let input = 89;
        let output = egg_count(input);
        let expected = 4;
        assert_eq!(output, expected);
    }
}

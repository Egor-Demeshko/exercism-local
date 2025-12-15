pub fn collatz(n: u64) -> Option<u64> {
    if n == 0 {
        return None;
    }
    let mut n_mut: u64 = n;
    let mut steps: u64 = 0;
    loop {
        if n_mut == 1 {
            return Some(steps);
        }
        n_mut = match n_mut % 2 {
            0 => n_mut / 2,
            _ => n_mut * 3 + 1,
        };
        steps += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn zero_steps_for_one() {
        let output = collatz(1);
        let expected = Some(0);
        assert_eq!(output, expected);
    }

    #[test]
    fn divide_if_even() {
        let output = collatz(16);
        let expected = Some(4);
        assert_eq!(output, expected);
    }

    #[test]
    fn even_and_odd_steps() {
        let output = collatz(12);
        let expected = Some(9);
        assert_eq!(output, expected);
    }

    #[test]
    fn large_number_of_even_and_odd_steps() {
        let output = collatz(1_000_000);
        let expected = Some(152);
        assert_eq!(output, expected);
    }
}

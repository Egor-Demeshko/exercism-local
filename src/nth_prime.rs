pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    let mut primes: Vec<u32> = vec![2];
    let mut candidate = 3;

    while primes.len() <= n as usize {
        let mut is_prime = true;

        for &prime in &primes {
            if prime * prime > candidate {
                break;
            }
            if candidate % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(candidate);
        }

        candidate += 2;
    }

    primes[n as usize]
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn first_prime() {
        let output = nth(0);

        let expected = 2;

        assert_eq!(output, expected);
    }

    #[test]
    fn second_prime() {
        let output = nth(1);

        let expected = 3;

        assert_eq!(output, expected);
    }

    #[test]
    fn sixth_prime() {
        let output = nth(5);

        let expected = 13;

        assert_eq!(output, expected);
    }

    #[test]
    fn big_prime() {
        let output = nth(10_000);

        let expected = 104_743;

        assert_eq!(output, expected);
    }
}

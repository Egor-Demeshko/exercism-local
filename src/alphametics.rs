use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let parts: Vec<&str> = input.split("==").collect();
    if parts.len() != 2 {
        return None;
    }

    let left_side = parts[0].trim();
    let right_side = parts[1].trim();

    let words: Vec<&str> = left_side
        .split('+')
        .map(|s| s.trim())
        .chain(std::iter::once(right_side))
        .collect();

    let mut letters: Vec<char> = Vec::new();
    for word in &words {
        for ch in word.chars() {
            if ch.is_alphabetic() && !letters.contains(&ch) {
                letters.push(ch);
            }
        }
    }

    if letters.len() > 10 {
        return None;
    }

    let mut digits: Vec<u8> = (0..=9).collect();
    let mut permutation = vec![0; letters.len()];
    find_solution(&letters, &words, &mut digits, &mut permutation, 0)
}

fn find_solution(
    letters: &[char],
    words: &[&str],
    available_digits: &mut Vec<u8>,
    assignment: &mut Vec<u8>,
    index: usize,
) -> Option<HashMap<char, u8>> {
    if index == letters.len() {
        let map: HashMap<char, u8> = letters
            .iter()
            .zip(assignment.iter())
            .map(|(&ch, &digit)| (ch, digit))
            .collect();

        if is_valid_solution(&map, words) {
            return Some(map);
        }
        return None;
    }

    for i in 0..available_digits.len() {
        if available_digits[i] == 0 && is_leading_letter(letters[index], words) {
            continue;
        }

        assignment[index] = available_digits[i];
        let digit = available_digits.remove(i);

        if let Some(solution) =
            find_solution(letters, words, available_digits, assignment, index + 1)
        {
            return Some(solution);
        }

        available_digits.insert(i, digit);
    }

    None
}

fn is_leading_letter(letter: char, words: &[&str]) -> bool {
    words.iter().any(|&word| word.starts_with(letter))
}

fn word_to_number(word: &str, map: &HashMap<char, u8>) -> u64 {
    let mut result = 0;
    for ch in word.chars() {
        result = result * 10 + *map.get(&ch).unwrap() as u64;
    }
    result
}

fn is_valid_solution(map: &HashMap<char, u8>, words: &[&str]) -> bool {
    let (result_word, operands) = words.split_last().unwrap();
    let sum: u64 = operands.iter().map(|&word| word_to_number(word, map)).sum();
    let result = word_to_number(result_word, map);

    sum == result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]

    fn puzzle_with_three_letters() {
        let answer = solve("I + BB == ILL");

        let expected = [('I', 1), ('B', 9), ('L', 0)].into_iter().collect();

        assert_eq!(answer, Some(expected));
    }

    #[test]
    fn solution_must_have_unique_value_for_each_letter() {
        let answer = solve("A == B");

        assert_eq!(answer, None);
    }

    #[test]
    fn leading_zero_solution_is_invalid() {
        let answer = solve("ACA + DD == BD");

        assert_eq!(answer, None);
    }

    #[test]
    fn puzzle_with_two_digits_final_carry() {
        let answer = solve("A + A + A + A + A + A + A + A + A + A + A + B == BCC");

        let expected = [('A', 9), ('B', 1), ('C', 0)].into_iter().collect();

        assert_eq!(answer, Some(expected));
    }

    #[test]
    fn puzzle_with_four_letters() {
        let answer = solve("AS + A == MOM");

        let expected = [('A', 9), ('S', 2), ('M', 1), ('O', 0)]
            .into_iter()
            .collect();

        assert_eq!(answer, Some(expected));
    }

    #[test]
    fn puzzle_with_six_letters() {
        let answer = solve("NO + NO + TOO == LATE");

        let expected = [('N', 7), ('O', 4), ('T', 9), ('L', 1), ('A', 0), ('E', 2)]
            .into_iter()
            .collect();

        assert_eq!(answer, Some(expected));
    }

    #[test]
    #[ignore]

    fn puzzle_with_seven_letters() {
        let answer = solve("HE + SEES + THE == LIGHT");

        let expected = [
            ('E', 4),
            ('G', 2),
            ('H', 5),
            ('I', 0),
            ('L', 1),
            ('S', 9),
            ('T', 7),
        ]
        .into_iter()
        .collect();

        assert_eq!(answer, Some(expected));
    }

    #[test]
    #[ignore]

    fn puzzle_with_eight_letters() {
        let answer = solve("SEND + MORE == MONEY");

        let expected = [
            ('S', 9),
            ('E', 5),
            ('N', 6),
            ('D', 7),
            ('M', 1),
            ('O', 0),
            ('R', 8),
            ('Y', 2),
        ]
        .into_iter()
        .collect();

        assert_eq!(answer, Some(expected));
    }
}

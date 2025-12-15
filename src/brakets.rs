use std::collections::HashMap;

const OPEN_SQUARE: char = '[';
const OPEN_CURLY: char = '{';
const OPEN_ROUND: char = '(';

const CLOSE_SQUARE: char = ']';
const CLOSE_CURLY: char = '}';
const CLOSE_ROUND: char = ')';

pub fn brackets_are_balanced(string: &str) -> bool {
    // shoud have stack collection
    let mut stack: Vec<char> = Vec::new();
    let bracket_pairs: HashMap<char, char> = HashMap::from([
        (OPEN_SQUARE, CLOSE_SQUARE),
        (OPEN_CURLY, CLOSE_CURLY),
        (OPEN_ROUND, CLOSE_ROUND),
    ]);
    let close_to_open: HashMap<char, char> = HashMap::from([
        (CLOSE_SQUARE, OPEN_SQUARE),
        (CLOSE_CURLY, OPEN_CURLY),
        (CLOSE_ROUND, OPEN_ROUND),
    ]);

    // if we find opening bracket then just push it to the stack
    // when find closing bracket, pop from stack and compare, if false return false right away
    // when all string looped and no errors => true, stack should be empty
    for char in string.chars() {
        if bracket_pairs.contains_key(&char) {
            stack.push(char);
        } else if close_to_open.contains_key(&char) {
            if stack.pop() != close_to_open.get(&char).copied() {
                return false;
            }
        } else {
            continue;
        }
    }

    stack.is_empty()
}

mod test {
    use super::*;
    #[test]
    fn paired_square_brackets() {
        assert!(brackets_are_balanced("[]"));
    }

    #[test]
    fn empty_string() {
        assert!(brackets_are_balanced(""));
    }

    #[test]
    fn unpaired_brackets() {
        assert!(!brackets_are_balanced("[["));
    }

    #[test]
    fn wrong_ordered_brackets() {
        assert!(!brackets_are_balanced("}{"));
    }
}

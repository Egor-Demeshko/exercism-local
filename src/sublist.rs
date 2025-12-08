use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    // 0. if both are empty, return the same
    if first_list.is_empty() && second_list.is_empty() {
        return Comparison::Equal;
    }

    match first_list.len().cmp(&second_list.len()) {
        // 2. check for equality if the length is equal
        Ordering::Equal => check_equal(first_list, second_list),
        // 3. If the first one is greater than the second one, check for superlist
        Ordering::Greater => check_superlist(first_list, second_list),
        // 4. If the second is greater than the first, then check for sublist
        Ordering::Less => check_sublist(first_list, second_list),
    }
}

fn check_equal(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.iter().eq(second_list.iter()) {
        return Comparison::Equal;
    }

    Comparison::Unequal
}

fn check_superlist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if second_list.is_empty() {
        return Comparison::Superlist;
    }

    // at this index and beyond there couldn't be any more subs
    let max_index = first_list.len() - second_list.len();

    for first_index in 0..=max_index {
        let mut all_match = false;
        for (second_index, second_value) in second_list.iter().enumerate() {
            all_match = true;
            if *second_value != first_list[first_index + second_index] {
                all_match = false;
                break;
            }
        }

        if all_match {
            return Comparison::Superlist;
        }
    }

    Comparison::Unequal
}

fn check_sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    if first_list.is_empty() {
        return Comparison::Sublist;
    }
    let max_index = second_list.len() - first_list.len();

    for second_index in 0..=max_index {
        let mut all_match = false;

        for (first_index, first_elem) in first_list.iter().enumerate() {
            all_match = true;
            if *first_elem != second_list[second_index + first_index] {
                all_match = false;
                break;
            }
        }

        if all_match {
            return Comparison::Sublist;
        }
    }

    Comparison::Unequal
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn both_empty() {
        let list_one: &[i32] = &[];
        let list_two: &[i32] = &[];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Equal;
        assert_eq!(output, expected);
    }

    #[test]
    fn empty_sublist() {
        let list_one: &[i32] = &[];
        let list_two: &[i32] = &[1, 2, 3];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn equal_length() {
        let list_one: &[i32] = &[1, 2, 3];
        let list_two: &[i32] = &[1, 3, 2];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Unequal;
        assert_eq!(output, expected);
    }

    #[test]
    fn normal_sublist() {
        let list_one: &[i32] = &[3, 4, 5];
        let list_two: &[i32] = &[0, 1, 2, 3, 4, 5];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }

    #[test]
    fn sublist_when_sequance() {
        let list_one: &[i32] = &[1, 1, 2];
        let list_two: &[i32] = &[0, 1, 1, 1, 2, 1, 2];
        let output = sublist(list_one, list_two);
        let expected = Comparison::Sublist;
        assert_eq!(output, expected);
    }
}

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }
    let mut max = array.len() - 1;
    let mut min: usize = 0;
    loop {
        let i = (min + max) / 2;
        if array[i] == key {
            return Some(i);
        } else if min == i || max == i {
            return None;
        } else if array[i] > key {
            max = i - 1;
        } else if array[i] < key {
            min = i + 1;
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn finds_a_value_in_an_array_with_one_element() {
        assert_eq!(find(&[6], 6), Some(0));
    }

    #[test]
    fn finds_a_value_in_the_middle_of_an_array() {
        assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 6), Some(3));
    }

    #[test]
    fn finds_a_value_at_the_beginning_of_an_array() {
        assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 1), Some(0));
    }

    #[test]
    fn finds_a_value_at_the_end_of_an_array() {
        assert_eq!(find(&[1, 3, 4, 6, 8, 9, 11], 11), Some(6));
    }

    #[test]
    fn finds_a_value_in_an_array_of_odd_length() {
        assert_eq!(
            find(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 634], 144),
            Some(9)
        );
    }

    #[test]
    fn finds_a_value_in_an_array_of_even_length() {
        assert_eq!(
            find(&[1, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377], 21),
            Some(5)
        );
    }

    #[test]
    fn nothing_is_found_when_the_left_and_right_bounds_cross() {
        assert_eq!(find(&[1, 2], 0), None);
    }
}

use std::char;

pub fn flowers(input: &[&str]) -> Vec<String> {
    const SPACE: char = ' ';
    const FLOWER: char = '*';
    let mut result: Vec<String> = vec![];
    let input_len = input.len();
    // " * * "
    for (row_index, row_string) in input.into_iter().enumerate() {
        let mut new_str = String::new();
        for (column_index, row_char) in row_string.char_indices() {
            if row_char != SPACE {
                new_str.push(row_char);
                continue;
            }

            let mut sum: u32 = 0;

            let rows = neighbor_indices(row_index, input_len);

            for row_i in rows {
                let row_to_check = match input.get(row_i) {
                    Some(row) => row,
                    None => "",
                };

                if row_to_check.is_empty() {
                    continue;
                }

                let columns = neighbor_indices(column_index, row_to_check.len());

                for column_j in columns.into_iter() {
                    let char = row_to_check.chars().nth(column_j);

                    if let Some(char) = char {
                        if char == FLOWER {
                            sum = sum + 1;
                        }
                    } else {
                        continue;
                    }
                }
            }

            let next_char = if sum > 0 {
                char::from_digit(sum, 10).unwrap_or_default()
            } else {
                SPACE
            };
            new_str.push(next_char);
        }

        result.push(new_str)
    }

    result
}

fn neighbor_indices(index: usize, max: usize) -> impl Iterator<Item = usize> {
    let start = index.saturating_sub(1);
    let end = (index + 1).min(max - 1);
    start..=end
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn horizont_line() {
        let input = &[" * * "];
        let expected = &["1*2*1"];
        let actual = flowers(input);
        assert_eq!(actual, expected);
    }

    #[test]
    fn surrounded_by_flowers() {
        #[rustfmt::skip]
        let (input, expected) = (
        &[
        "***",
        "* *",
        "***",], 
        &[
        "***",
        "*8*",
        "***",
        ]);
        let actual = flowers(input);
        assert_eq!(actual, expected);
    }
}

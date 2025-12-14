pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();
    if list.is_empty() {
        return proverb;
    }
    let mut iter = list.iter();
    let mut previous: &str = iter.next().unwrap();
    let last: String = format!("And all for the want of a {previous}.");
    if list.len() < 2 {
        return last;
    }

    for current in iter {
        proverb.push_str(&format!(
            "For want of a {previous} the {current} was lost.\n"
        ));
        previous = current;
    }

    proverb.push_str(&last);
    proverb
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn one_piece() {
        let input = &["nail"];
        let output = build_proverb(input);
        let expected: String = ["And all for the want of a nail."].join("\n");
        assert_eq!(output, expected);
    }

    #[test]
    fn two_pieces() {
        let input = &["nail", "shoe"];
        let output = build_proverb(input);
        let expected: String = [
            "For want of a nail the shoe was lost.",
            "And all for the want of a nail.",
        ]
        .join("\n");
        assert_eq!(output, expected);
    }
}

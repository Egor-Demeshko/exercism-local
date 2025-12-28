const SPLIT_CHARS: (char, char, char) = (' ', '-', '_');

pub fn abbreviate(phrase: &str) -> String {
    let mut chars = phrase.chars();

    let mut abr = String::from(
        chars
            .next()
            .unwrap_or(' ')
            .to_uppercase()
            .next()
            .unwrap_or(' '),
    );

    let mut next_include = false;
    let mut previous_char: char = 'A';
    for char in chars {
        if is_next(char) {
            next_include = true;
            continue;
        }

        if is_camel(char, previous_char) || next_include {
            abr.push(char.to_uppercase().next().unwrap_or(' '));
            next_include = false;
        }
        previous_char = char;
    }
    abr
}

fn is_camel(ch: char, prev: char) -> bool {
    ch.is_uppercase() && prev.is_lowercase()
}

fn is_next(ch: char) -> bool {
    ch == SPLIT_CHARS.0 || ch == SPLIT_CHARS.1 || ch == SPLIT_CHARS.2
}
mod test {
    use super::*;

    #[test]
    fn basic() {
        let input = "Portable Network Graphics";
        let output = abbreviate(input);
        let expected = "PNG";
        assert_eq!(output, expected);
    }

    #[test]
    fn lowercase_words() {
        let input = "Ruby on Rails";
        let output = abbreviate(input);
        let expected = "ROR";
        assert_eq!(output, expected);
    }

    #[test]
    fn punctuation() {
        let input = "First In, First Out";
        let output = abbreviate(input);
        let expected = "FIFO";
        assert_eq!(output, expected);
    }

    #[test]
    fn all_caps_word() {
        let input = "GNU Image Manipulation Program";
        let output = abbreviate(input);
        let expected = "GIMP";
        assert_eq!(output, expected);
    }

    #[test]
    fn punctuation_without_whitespace() {
        let input = "Complementary metal-oxide semiconductor";
        let output = abbreviate(input);
        let expected = "CMOS";
        assert_eq!(output, expected);
    }

    #[test]
    fn very_long_abbreviation() {
        let input = "Rolling On The Floor Laughing So Hard That My Dogs Came Over And Licked Me";
        let output = abbreviate(input);
        let expected = "ROTFLSHTMDCOALM";
        assert_eq!(output, expected);
    }

    #[test]
    fn camelcase() {
        let input = "HyperText Markup Language";

        let output = abbreviate(input);

        let expected = "HTML";

        assert_eq!(output, expected);
    }
}

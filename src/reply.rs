enum Response {
    Fine,
    Sure,
    Chill,
    QuestionYell,
    Whatever,
}

impl Response {
    fn as_str(&self) -> &str {
        match self {
            Self::Fine => "Fine. Be that way!",
            Self::Sure => "Sure.",
            Self::Chill => "Whoa, chill out!",
            Self::QuestionYell => "Calm down, I know what I'm doing!",
            Self::Whatever => "Whatever.",
        }
    }
}

struct Question<'a> {
    answered: bool,
    all_capital: bool,
    end_with_question: bool,
    question: String,
    pub response: &'a str,
}

impl<'a> Question<'a> {
    fn new(question: &str) -> Self {
        Question {
            answered: false,
            all_capital: false,
            end_with_question: false,
            question: String::from(question.trim()),
            response: Response::Whatever.as_str(),
        }
    }

    fn answer(mut self, answer: &'a str) -> Self {
        self.answered = true;
        self.response = answer;
        self
    }

    fn get_question(&self) -> String {
        self.question.clone()
    }

    fn is_last_question_mark(mut self) -> Self {
        if self.answered {
            return self;
        }
        let question = self.get_question();
        let question_len = question.len();
        if question.chars().nth(question_len - 1).unwrap_or(' ') == '?' {
            self.end_with_question = true
        }
        self
    }

    fn is_all_capital(mut self) -> Self {
        let mut has_letters = false;

        for ch in self.question.chars() {
            if ch.is_ascii_alphabetic() {
                has_letters = true;
                if ch.is_ascii_lowercase() {
                    self.all_capital = false;
                    return self;
                }
            }
        }

        self.all_capital = has_letters;
        self
    }

    fn create_response(self) -> Self {
        if self.answered {
            return self;
        }
        if self.end_with_question && self.all_capital {
            return self.answer(Response::QuestionYell.as_str());
        } else if self.end_with_question {
            return self.answer(Response::Sure.as_str());
        } else if self.all_capital {
            return self.answer(Response::Chill.as_str());
        } else {
            return self.answer(Response::Whatever.as_str());
        }
    }

    fn when_empty(self) -> Self {
        if self.answered {
            return self;
        }
        if self.question.len() == 0 {
            return self.answer(Response::Fine.as_str());
        }

        self
    }
}

pub fn reply(message: &str) -> &str {
    let question = Question::new(message);
    question
        .when_empty()
        .is_last_question_mark()
        .is_all_capital()
        .create_response()
        .response
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn stating_something() {
        assert_eq!(reply("Tom-ay-to, tom-aaaah-to."), "Whatever.");
    }

    #[test]
    fn shouting() {
        assert_eq!(reply("WATCH OUT!"), "Whoa, chill out!");
    }

    #[test]
    fn shouting_gibberish() {
        assert_eq!(reply("FCECDFCAAB"), "Whoa, chill out!");
    }

    #[test]
    fn asking_a_question() {
        assert_eq!(
            reply("Does this cryogenic chamber make me look fat?"),
            "Sure."
        );
    }

    #[test]
    fn asking_a_numeric_question() {
        assert_eq!(reply("You are, what, like 15?"), "Sure.");
    }
}

#[derive(Debug)]
pub struct HighScores {
    scores: Vec<u32>,
}

impl HighScores {
    pub fn new(scores: &[u32]) -> Self {
        HighScores {
            scores: scores.to_vec(),
        }
    }

    pub fn scores(&self) -> &[u32] {
        &self.scores
    }

    pub fn latest(&self) -> Option<u32> {
        if self.scores.is_empty() {
            return None;
        }
        self.scores.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        if self.scores.is_empty() {
            return None;
        }
        let scores_copy = self.get_sorted_copy();
        scores_copy.last().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let scores_copy = self.get_sorted_copy();
        let start = if scores_copy.len() > 3 {
            scores_copy.len() - 3
        } else {
            0
        };
        let mut slice = scores_copy[start..scores_copy.len()].to_vec();
        slice.sort_by(|a, b| b.cmp(a));
        slice
    }

    fn get_sorted_copy(&self) -> Vec<u32> {
        let mut scores_copy: Vec<u32> = self.scores.to_vec();
        scores_copy.sort();
        scores_copy
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn list_of_scores() {
        let expected = [30, 50, 20, 70];

        let high_scores = HighScores::new(&expected);

        assert_eq!(high_scores.scores(), &expected);
    }

    #[test]
    fn latest_score() {
        let high_scores = HighScores::new(&[100, 0, 90, 30]);

        assert_eq!(high_scores.latest(), Some(30));
    }

    #[test]
    fn personal_best() {
        let high_scores = HighScores::new(&[40, 100, 70]);

        assert_eq!(high_scores.personal_best(), Some(100));
    }

    #[test]
    fn personal_top_three_from_a_list_of_scores() {
        let high_scores = HighScores::new(&[10, 30, 90, 30, 100, 20, 10, 0, 30, 40, 40, 70, 70]);

        assert_eq!(high_scores.personal_top_three(), vec![100, 90, 70]);
    }
}

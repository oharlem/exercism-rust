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

    // Return all the scores as a slice
    pub fn scores(&self) -> &[u32] {
        self.scores.as_slice()
    }

    // Return the latest (last) score")
    pub fn latest(&self) -> Option<u32> {
        match self.scores.len() {
            0 => None,
            _ => Some(self.scores[self.scores.len() - 1]),
        }
    }

    // Return the highest score
    pub fn personal_best(&self) -> Option<u32> {
        match self.scores.iter().max() {
            Some(max) => Some(*max),
            None => None,
        }
    }

    // Return 3 highest scores
    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut ss = self.scores.clone();
        ss.sort();
        ss.reverse();
        if ss.len() > 3 {
            return ss[..3].to_vec();
        }

        ss.to_vec()
    }
}

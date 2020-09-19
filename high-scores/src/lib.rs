#[derive(Debug)]
pub struct HighScores<'a> {
    scores: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        Self {
            scores
        }
    }

    // Return all the scores as a slice
    pub fn scores(&self) -> &[u32] {
        self.scores
    }

    // Return the latest (last) score")
    pub fn latest(&self) -> Option<u32> {
        self.scores.last().cloned()
    }

    // Return the highest score
    pub fn personal_best(&self) -> Option<u32> {
        self.scores.iter().max().cloned()
    }

    // Return 3 highest scores
    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut s = self.scores.to_vec();
        s.sort();
        s.reverse();
        if s.len() > 3 {
            return s[..3].to_vec();
        }

        s
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy, Hash, Eq, PartialEq, Debug)]
pub struct Score {
    count: u32,
    score: i32,
}

impl Score {
    pub fn new() -> Self {
        Score {
            count: 0,
            score: 0,
        }
    }

    pub fn plus_one(&mut self) {
        self.score += 1;
        self.count += 1;
    }

    pub fn minus_one(&mut self) {
        self.score -= 1;
        self.count += 1;
    }
}

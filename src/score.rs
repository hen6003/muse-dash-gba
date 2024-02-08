pub enum Grade {
    SSS,
    SS,
    S,
    A,
    B,
    C,
    D,
}

pub struct Score {
    score: usize,
    max_combo: usize,
    accuracy: u8,
}

impl Score {
    pub fn new(score: usize, max_combo: usize, accuracy: u8) -> Self {
        Self {
            score,
            max_combo,
            accuracy,
        }
    }

    pub fn score(&self) -> usize {
        self.score
    }

    pub fn grade(&self) -> Grade {
        match self.accuracy {
            100 => Grade::SSS,
            95..=99 => Grade::SS,
            90..=94 => Grade::S,
            80..=89 => Grade::A,
            70..=79 => Grade::B,
            60..=69 => Grade::C,
            0..=59 => Grade::D,
            _ => unreachable!(),
        }
    }

    pub fn max_combo(&self) -> usize {
        self.max_combo
    }

    pub fn accuracy(&self) -> u8 {
        self.accuracy
    }
}

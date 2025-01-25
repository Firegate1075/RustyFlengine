use crate::datamodel::enums::difficulty::Difficulty;

#[derive(Clone)]
pub struct Options {
    difficulty: Difficulty,
    recursion_depth: u32,
}

impl Options {
    pub fn difficulty(&self) -> &Difficulty {
        &self.difficulty
    }

    pub fn recursion_depth(&self) -> u32 {
        self.recursion_depth
    }

    pub fn set_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
    }

    pub fn set_recursion_depth(&mut self, recursion_depth: u32) {
        self.recursion_depth = recursion_depth;
    }

    pub fn new(difficulty: Difficulty, recursion_depth: u32) -> Self {
        Self { difficulty, recursion_depth }
    }

    pub fn from_default() -> Self {
        Self {
            difficulty: Difficulty::NORMAL,
            recursion_depth: 4,
        }
    }
}
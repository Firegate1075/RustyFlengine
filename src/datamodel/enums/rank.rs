#[derive(Debug, Copy, PartialEq)]
#[repr(usize)]
#[derive(Clone)]
pub enum Rank {
    ONE     = 0,
    TWO     = 1,
    THREE   = 2,
    FOUR    = 3,
    FIVE    = 4,
    SIX     = 5,
    SEVEN   = 6,
    EIGHT   = 7,
}

impl Rank {
    pub fn to_index(&self) -> usize {
        match self {
            Rank::ONE   => 0,
            Rank::TWO   => 1,
            Rank::THREE => 2,
            Rank::FOUR  => 3,
            Rank::FIVE  => 4,
            Rank::SIX   => 5,
            Rank::SEVEN => 6,
            Rank::EIGHT => 7,
        }
    }
}
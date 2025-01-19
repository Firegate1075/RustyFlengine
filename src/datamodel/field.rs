use crate::datamodel::enums::rank::Rank;
use crate::datamodel::enums::file::File;

/// This struct represents a field on the chess board.
#[derive(Debug)]
pub struct Field {
    /// represents the line on the chess board (One, Two, Three, ... Eight)
    rank: Rank,
    /// represents the row on the chess board (A, B, C, ... H)
    file: File,
}

impl Field {
    pub fn new(row: File, rank: Rank) -> Field {
        Field { rank, file: row }
    }
    pub fn rank(&self) -> Rank {
        self.rank
    }

    pub fn file(&self) -> File {
        self.file
    }

    pub fn set_rank(&mut self, rank: Rank) {
        self.rank = rank;
    }

    pub fn set_file(&mut self, file: File) {
        self.file = file;
    }
}
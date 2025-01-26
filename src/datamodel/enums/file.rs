use strum_macros::EnumIter;

#[derive(Debug, Copy, Clone, PartialEq, EnumIter)]
#[repr(usize)]
pub enum File {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
}

impl File {
    pub fn to_index(&self) -> usize {
        match self {
            File::A => 0,
            File::B => 1,
            File::C => 2,
            File::D => 3,
            File::E => 4,
            File::F => 5,
            File::G => 6,
            File::H => 7,
        }
    }

    pub fn from_index(i: usize) -> Self {
        match i {
            0 => File::A,
            1 => File::B,
            2 => File::C,
            3 => File::D,
            4 => File::E,
            5 => File::F,
            6 => File::G,
            7 => File::H,
            _ => panic!("Invalid File Number {}", i),
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "A"|"a" => File::A,
            "B"|"b" => File::B,
            "C"|"c" => File::C,
            "D"|"d" => File::D,
            "E"|"e" => File::E,
            "F"|"f" => File::F,
            "G"|"g" => File::G,
            "H"|"h" => File::H,
            _ => panic!("Invalid File {}", s),
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            File::A => "A",
            File::B => "B",
            File::C => "C",
            File::D => "D",
            File::E => "E",
            File::F => "F",
            File::G => "G",
            File::H => "H",
        }
    }
}
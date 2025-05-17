use std::ops::Not;

#[derive(Copy, Clone, PartialEq)]
#[derive(Debug)]
pub enum Color {
    BLACK,
    WHITE,
}
impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}",  if self == &Color::WHITE { "White" } else { "Black" })
    }
}

impl Not for Color {
    type Output = Color;
    fn not(self) -> Self::Output {
        match self {
            Color::BLACK => Color::WHITE,
            Color::WHITE => Color::BLACK,
        }
    }
}

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
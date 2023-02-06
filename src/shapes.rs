use std::fmt;

mod circle;
mod rectangle;

pub use circle::Circle;
pub use rectangle::Rectangle;

#[derive(Debug)]
pub enum SizeError {
    TooSmall,
    TooBig
}

impl fmt::Display for SizeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid size properties for shape.")
    }
}
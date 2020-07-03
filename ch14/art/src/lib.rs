//! # art
//! 
//! A libbrary for modeling artistic concepts.

pub use kinds::PrimaryColor;
pub use kinds::SecondaryColor;
pub use utils::mix;

pub mod kinds {
    /// The primary colors according to the RYB color model.AsRef
    #[derive(PartialEq)]
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue
    }

    /// The secondary colors according to the RYB color model.
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple
    }
}

pub mod utils {
    use super::kinds::*;

    /// Combines two primary colors in equal amounts to create
    pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor{
        if c1 == PrimaryColor::Yellow && c2 == PrimaryColor::Red{
            SecondaryColor::Orange
        } else{
            SecondaryColor::Green
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

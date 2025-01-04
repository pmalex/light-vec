use crate::LightVec;
use std::ops::Add;

pub use arange::Arange;
pub use constant::Constant;
pub use heavy::Heavy;
pub use random::RandomVec;

pub mod arange;
pub mod constant;
pub mod heavy;
pub mod random;

//
// Arange + Constant = Arange
//
impl Add<Constant> for Arange {
    type Output = Arange;

    fn add(self, rhs: Constant) -> Arange {
        assert_eq!(self.size(), rhs.size());

        Arange {
            start: self.start + rhs.value,
            end: self.end + rhs.value,
            step: self.step,
        }
    }
}

use std::ops::Add;

pub use arange::Arange;
pub use constant::Constant;

use crate::LightVec;

pub mod arange;
pub mod constant;
pub mod vec;

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

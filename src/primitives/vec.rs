//! Для обычного вектора определяем поведение как лёгкого.

use crate::{Int, LightVec};

impl LightVec for Vec<Int> {
    fn size(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Int {
        self[index]
    }

    fn sum(&self) -> Int {
        self.iter().sum()
    }

    fn product(&self) -> Int {
        self.iter().product()
    }
}

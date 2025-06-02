//! Для обычного вектора определяем поведение как лёгкого.

use crate::{Int, LightVec};

/// Тяжёлый вектор = обычный вектор
pub type Heavy = Vec<Int>;

impl LightVec for Heavy {
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

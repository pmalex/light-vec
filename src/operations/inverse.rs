use crate::{Int, LightVec};

/// Обращает порядок следования элементов вектора.
pub struct Inverse<V: LightVec>(pub V);

impl<V: LightVec> LightVec for Inverse<V> {
    fn size(&self) -> usize {
        self.0.size()
    }

    fn get(&self, index: usize) -> Int {
        self.0.get(self.0.size() - 1 - index)
    }

    fn sum(&self) -> Int {
        self.0.sum()
    }

    fn product(&self) -> Int {
        self.0.product()
    }
}

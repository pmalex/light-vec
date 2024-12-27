use crate::{Int, LightVec};

/// Циклически повторяет другой вектор.
pub struct Cycle<V: LightVec> {
    vec: V,

    /// Сколько раз повторять вектор.
    count: usize,
}

impl<V: LightVec> Cycle<V> {
    pub fn new(vec: V, count: usize) -> Self {
        assert!(count > 0);

        Self { vec, count }
    }
}

impl<V: LightVec> LightVec for Cycle<V> {
    fn size(&self) -> usize {
        self.vec.size() * self.count
    }

    fn get(&self, _index: usize) -> crate::Int {
        todo!()
    }

    fn sum(&self) -> crate::Int {
        self.vec.sum() * self.count as Int
    }

    fn prod(&self) -> Int {
        todo!()
    }
}

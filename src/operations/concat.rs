use crate::{Int, LightVec};

/// Объединяет два вектора в один последовательный.
pub struct Concat<V1: LightVec, V2: LightVec>(pub V1, pub V2);

impl<V1, V2> LightVec for Concat<V1, V2>
where
    V1: LightVec,
    V2: LightVec,
{
    fn size(&self) -> usize {
        self.0.size() + self.1.size()
    }

    fn get(&self, index: usize) -> Int {
        assert!(index < self.size());

        if index < self.0.size() {
            self.0.get(index)
        } else {
            self.1.get(index - self.0.size())
        }
    }

    fn sum(&self) -> Int {
        self.0.sum() + self.1.sum()
    }

    fn product(&self) -> Int {
        todo!()
    }
}

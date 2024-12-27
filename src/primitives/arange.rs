use crate::{Int, LightVec};

/// Вектор, значения которого являются арифметической прогрессией.
pub struct ARange {
    /// Начальный индекс (включительно).
    pub start: Int,

    /// Конечный индекс (включительно)
    pub end: Int,

    /// Шаг
    pub step: usize,
}

impl ARange {
    pub fn new(start: Int, end: Int, step: usize) -> Self {
        assert!(start < end);
        assert!(step > 0);

        assert!((end - start) % (step as Int) == 0);

        Self { start, end, step }
    }
}

impl LightVec for ARange {
    fn size(&self) -> usize {
        1 + ((self.end - self.start) / (self.step as Int)) as usize
    }

    fn get(&self, index: usize) -> Int {
        assert!(index < self.size());

        self.start + (index * self.step) as Int
    }

    fn sum(&self) -> Int {
        (self.start + self.end) * (self.size() as Int) / 2
    }

    fn prod(&self) -> Int {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use crate::LightVec;

    use super::ARange;

    #[test]
    fn size_test() {
        assert_eq!(ARange::new(-3, 3, 2).size(), 4);
        assert_eq!(ARange::new(7, 11, 2).size(), 3);
        assert_eq!(ARange::new(4, 22, 3).size(), 7);
    }

    #[test]
    fn to_vec_test() {
        assert_eq!(ARange::new(-3, 3, 1).to_vec(), vec![-3, -2, -1, 0, 1, 2, 3]);
        assert_eq!(
            ARange::new(4, 22, 3).to_vec(),
            vec![4, 7, 10, 13, 16, 19, 22]
        );
        assert_eq!(ARange::new(-9, 3, 4).to_vec(), vec![-9, -5, -1, 3]);
    }

    #[test]
    fn sum_test() {
        assert_eq!(ARange::new(7, 11, 2).sum(), 27);
        assert_eq!(ARange::new(-3, 5, 2).sum(), 5);
    }
}

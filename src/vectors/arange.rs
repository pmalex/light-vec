use std::ops::{Add, Mul};

use crate::{Int, LightVec};

/// Вектор, значения которого являются арифметической прогрессией.
pub struct Arange {
    /// Начальный индекс (включительно).
    pub start: Int,

    /// Конечный индекс (включительно)
    pub end: Int,

    /// Шаг
    pub step: usize,
}

impl Arange {
    pub fn new(start: Int, end: Int, step: usize) -> Self {
        assert!(start < end);
        assert!(step > 0);

        assert!((end - start) % (step as Int) == 0);

        Self { start, end, step }
    }
}

impl LightVec for Arange {
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

    fn product(&self) -> Int {
        todo!()
    }
}

impl Add<Arange> for Arange {
    type Output = Arange;

    fn add(self, rhs: Arange) -> Arange {
        assert_eq!(self.size(), rhs.size());

        Arange {
            start: self.start + rhs.start,
            end: self.end + rhs.end,
            step: self.step + rhs.step,
        }
    }
}

impl Add<Int> for Arange {
    type Output = Arange;

    fn add(self, value: Int) -> Arange {
        Arange {
            start: self.start + value,
            end: self.end + value,
            step: self.step,
        }
    }
}

impl Mul<Int> for Arange {
    type Output = Arange;

    fn mul(self, value: Int) -> Arange {
        assert!(value > 0);

        Arange {
            start: self.start * value,
            end: self.end * value,
            step: self.step * value as usize,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::LightVec;

    use super::Arange;

    #[test]
    fn size_test() {
        assert_eq!(Arange::new(-3, 3, 2).size(), 4);
        assert_eq!(Arange::new(7, 11, 2).size(), 3);
        assert_eq!(Arange::new(4, 22, 3).size(), 7);
    }

    #[test]
    fn to_vec_test() {
        assert_eq!(Arange::new(-3, 3, 1).to_heavy(), vec![-3, -2, -1, 0, 1, 2, 3]);
        assert_eq!(
            Arange::new(4, 22, 3).to_heavy(),
            vec![4, 7, 10, 13, 16, 19, 22]
        );
        assert_eq!(Arange::new(-9, 3, 4).to_heavy(), vec![-9, -5, -1, 3]);
    }

    #[test]
    fn sum_test() {
        assert_eq!(Arange::new(7, 11, 2).sum(), 27);
        assert_eq!(Arange::new(-3, 5, 2).sum(), 5);
    }
}

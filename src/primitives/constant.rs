use std::ops::Add;

use crate::{Int, LightVec};

#[derive(Debug)]
/// Вектор, содержащий фиксированное значение.
pub struct Constant {
    value: Int,

    /// Длина вектора
    size: usize,
}

impl Constant {
    pub fn new(value: Int, size: usize) -> Self {
        assert!(size > 0);

        Self { value, size }
    }

    /// Разделяет вектор на две части по переданному индексу.
    ///
    /// # Паникует
    /// если размер вектора равен 1 или же если индекс выходит за пределы размера вектора.
    pub fn split_at(self, index: usize) -> (Self, Self) {
        assert!(self.size > 1, "Impossible to split vector of size 1");
        assert!(index > 0 && index < self.size);

        let first = Self::new(self.value, index);
        let second = Self::new(self.value, self.size - index);

        (first, second)
    }
}

impl LightVec for Constant {
    fn size(&self) -> usize {
        self.size
    }

    fn get(&self, index: usize) -> Int {
        assert!(index < self.size);

        self.value
    }

    fn to_vec(self) -> Vec<Int> {
        vec![self.value; self.size]
    }

    fn from_vec(vec: Vec<Int>) -> Self {
        assert!(!vec.is_empty());

        let value = vec[0];
        let size = vec.len();

        assert!(
            vec.into_iter().all(|v| v == value),
            "Вектор содержит разные значения и не может быть преобразован в константный"
        );

        Self::new(value, size)
    }
}

impl Add for Constant {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        assert_eq!(self.size, rhs.size);

        Self {
            value: self.value + rhs.value,
            size: self.size,
        }
    }
}

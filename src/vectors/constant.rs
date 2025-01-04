use std::ops::Add;

use crate::{Int, LightVec};

#[derive(Debug)]
/// Вектор, все элементы которого равны.
pub struct Constant {
    pub value: Int,

    /// Длина вектора
    size: usize,
}

impl Constant {
    /// Создаёт константный вектор указанного размера.
    pub fn new(value: Int, size: usize) -> Self {
        assert!(size > 0);

        Self { value, size }
    }

    /// Создаёт нулевой вектор указанного размера.
    pub fn zeros(size: usize) -> Self {
        Self::new(0, size)
    }

    /// Создаёт единичный вектор указанного размера.
    pub fn ones(size: usize) -> Self {
        Self::new(1, size)
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

    fn to_vec(&self) -> Vec<Int> {
        vec![self.value; self.size]
    }

    fn sum(&self) -> Int {
        self.value * (self.size as Int)
    }

    fn product(&self) -> Int {
        self.value.pow(self.size as u32)
    }
}

impl Add<Constant> for Constant {
    type Output = Constant;

    fn add(self, rhs: Constant) -> Constant {
        assert_eq!(self.size, rhs.size);

        Self {
            value: self.value + rhs.value,
            size: self.size,
        }
    }
}

impl Add<Int> for Constant {
    type Output = Constant;

    fn add(self, rhs: Int) -> Self::Output {
        Constant {
            value: self.value + rhs,
            size: self.size,
        }
    }
}

impl From<Vec<Int>> for Constant {
    fn from(vec: Vec<Int>) -> Self {
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

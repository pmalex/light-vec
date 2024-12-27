//! Библиотека для работы с целочисленными конечными виртуальными (лёгкими) векторами.

/// Тип данных, который имеют значения векторов.
pub type Int = i32;

pub mod operations;
pub mod primitives;

/// Описывает метода, общие для всех виртуальных векторов.
pub trait LightVec {
    /// Возвращает размер вектора.
    fn size(&self) -> usize;

    /// Возвращает элемент вектора, находящийся по переданному индексу.
    fn get(&self, index: usize) -> Int;

    /// Возвращает сумму элементов вектора.
    fn sum(&self) -> Int;

    /// Возвращает произведение элементов вектора.
    fn product(&self) -> Int;

    /// Преобразовывает лёгкий вектор в тяжёлый.
    fn to_vec(self) -> Vec<Int>
    where
        Self: Sized,
    {
        (0..self.size()).map(|index| self.get(index)).collect()
    }
}

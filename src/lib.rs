//! Библиотека для работы с целочисленными конечными виртуальными (лёгкими) векторами.

/// Тип данных, который имеют элементы вектора.
pub type Int = i32;

pub mod operations;
pub mod vectors;

/// Методы, общие для всех виртуальных векторов.
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
    fn to_heavy(&self) -> vectors::Heavy
    where
        Self: Sized,
    {
        (0..self.size()).map(|index| self.get(index)).collect()
    }
}

use crate::Int;

/// Вектор, значения которого являются арифметической прогрессией.
pub struct Range {
    pub start: Int,
    pub end: Int,

    /// Шаг. Может быть отрицательным.
    pub step: Int,
}

impl Range {
    pub fn new(start: Int, end: Int, step: Int) -> Self {
        assert!(start < end);
        assert!(step != 0);

        Self { start, end, step }
    }
}

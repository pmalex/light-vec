use crate::LightVec;

/// Разжижает вектор.
///
/// Пример для `dilation = 2`:
/// ```
/// [1, 2, 3] -> [1, 0, 0, 2, 0, 0, 3]
/// ```
pub struct Dilation<V: LightVec> {
    vec: V,
    dilation: usize,
}

impl<V: LightVec> Dilation<V> {
    pub fn new(vec: V, dilation: usize) -> Self {
        assert!(dilation > 0);

        Self { vec, dilation }
    }
}

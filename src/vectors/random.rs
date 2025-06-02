use crate::Int;
use rand::Rng;

#[derive(Clone)]
/// Дискретная случайная величина из равномерного распределения
/// на интервале от min до max (включительно).
pub struct RandomValue {
    min: Int,
    max: Int,
}

impl RandomValue {
    pub fn new(min: Int, max: Int) -> Self {
        Self { min, max }
    }

    /// Вскрывает значение случайной величины, уничтожая её.
    pub fn collapse(self) -> Int {
        rand::rng().random_range(self.min..=self.max)
    }
}

#[derive(Clone)]
/// Дискретный вектор равномерно распределённых
/// на интервале случайных величин.
pub struct RandomVec {
    value: RandomValue,
    size: usize,
}

impl RandomVec {
    pub fn new(min: Int, max: Int, size: usize) -> Self {
        assert!(size > 0);
        assert!(min < max);

        Self {
            value: RandomValue::new(min, max),
            size,
        }
    }

    pub fn get(&self) -> RandomValue {
        self.value.clone()
    }

    pub fn sum(&self) -> RandomValue {
        RandomValue {
            min: self.value.min * self.size as Int,
            max: self.value.max * self.size as Int,
        }
    }

    /// Среднее значение случайного вектора.
    pub fn mean(&self) -> f32 {
        (self.value.min as f32 + self.value.max as f32) / 2.0
    }

    /// Дисперсия случайного вектора.
    pub fn var(&self) -> f32 {
        let RandomValue { min, max } = self.value;

        ((max + 1) as f32 - min as f32) * ((max + 1) as f32 - min as f32) / 12.0
    }

    /// Стандартное отклонение случайного вектора.
    pub fn std(&self) -> f32 {
        self.var().sqrt()
    }
}

impl Into<Vec<Int>> for RandomVec {
    fn into(self) -> Vec<Int> {
        (0..self.size)
            .map(|_| self.value.clone().collapse())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::RandomVec;
    use crate::Int;

    #[test]
    fn sum_test() {
        let light_vec = RandomVec::new(-2, 9, 10);
        let light_vec_sum = light_vec.sum();

        let ys: Vec<Int> = (0..10_000)
            .map(|_| light_vec_sum.clone().collapse())
            .collect();
        let ys_mean = mean(&ys);
        dbg!(ys_mean);

        // Считаем фактическую сумму.
        let vec = (0..1_000)
            .map(|_| {
                let heavy_vec: Vec<Int> = light_vec.clone().into();

                heavy_vec.iter().sum::<Int>()
            })
            .collect::<Vec<Int>>();

        let vec_mean = mean(&vec);
        dbg!(vec_mean);

        assert!((ys_mean - vec_mean).abs() < 0.9);
    }

    #[test]
    fn mean_std_test() {
        let light_vec = RandomVec::new(-2, 9, 2_000_000);
        let heavy_vec: Vec<Int> = light_vec.clone().into();

        let light_vec_mean = light_vec.mean();
        let light_vec_std = light_vec.std();

        dbg!(light_vec_mean);
        dbg!(light_vec_std);

        let heavy_vec_mean = mean(&heavy_vec);
        let heavy_vec_std = std(&heavy_vec);

        dbg!(heavy_vec_mean);
        dbg!(heavy_vec_std);

        assert!((heavy_vec_mean - light_vec_mean).abs() < 1e-2);
        assert!((heavy_vec_std - light_vec_std).abs() < 4e-2);
    }

    /// Подсчитывает среднее по вектору.
    fn mean(vec: &[Int]) -> f32 {
        vec.iter().sum::<Int>() as f32 / vec.len() as f32
    }

    /// Считает стандартное отклонение вектора.
    fn std(vec: &[Int]) -> f32 {
        let mean = mean(vec);

        (vec.iter()
            .map(|&v| (v as f32 - mean) * (v as f32 - mean))
            .sum::<f32>()
            / vec.len() as f32)
            .sqrt()
    }
}

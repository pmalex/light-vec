//! Простейший пример использования библиотеки

use light_vec::{
    operations::{Concat, Inverse},
    primitives::{Arange, Constant},
    LightVec,
};

fn main() {
    let vec_1 = Constant::new(2, 5);
    let vec_2 = Arange::new(1, 9, 2);
    let vec_3 = vec![8, 7, 1];

    dbg!(Concat(Inverse(vec_2 + vec_1), vec_3).to_vec());
}

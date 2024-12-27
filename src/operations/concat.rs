use crate::LightVec;

pub struct Concat<V1: LightVec, V2: LightVec>(pub V1, pub V2);

//! Операции над векторами.

pub use concat::Concat;
pub use dilation::Dilation;
pub use inverse::Inverse;
pub use repeat::Repeat;

pub mod concat;
pub mod dilation;
pub mod inverse;
pub mod padding;
pub mod repeat;
pub mod slice;

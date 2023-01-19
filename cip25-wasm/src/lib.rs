#[cfg(not(feature="wasm"))]
pub use core::*;

#[cfg(feature="wasm")]
mod inner;

#[cfg(feature="wasm")]
pub use inner::*;

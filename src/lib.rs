//! Rust Vec which can contain arbitrary types inrernally casts inputs to ``Box<UnsafeAny>``.

extern crate unsafe_any as uany;

mod anyhashmap;
mod anyvec;
mod cast;

pub use anyhashmap::AnyHashMap;
pub use anyvec::AnyVec;
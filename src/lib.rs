#![cfg_attr(target_os = "windows", doc=include_str!("..\\README.md"))]
#![cfg_attr(not(target_os = "windows"), doc=include_str!("../README.md"))]
// #![warn(missing_docs)] // uncomment when writing docs

pub mod error;
mod primitive_impls;
mod private;
mod traits;
pub mod types;

pub use traits::{Read, ReadContext, ReadVersioned, Write};

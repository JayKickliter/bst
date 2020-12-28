#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(debug_assertions, allow(dead_code))]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod ptree;
mod ltree;

pub use ptree::*;
pub use ltree::*;

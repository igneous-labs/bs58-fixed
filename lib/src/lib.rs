#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

mod borrowed;
mod owned;

//pub use borrowed::*;
pub use owned::*;

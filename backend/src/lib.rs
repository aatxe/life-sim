#![feature(custom_derive, plugin)]
#![plugin(rand_macros)]

extern crate rand;
extern crate rustc_serialize;

pub mod brain;
pub mod chem;
pub mod genome;

pub use brain::*;
pub use chem::*;
pub use genome::*;

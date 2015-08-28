#![no_std]

#![feature(alloc)]
#![feature(no_std)]
#![feature(collections)]
#![feature(vec_push_all)]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate collections;

// for tests
#[cfg(test)]
#[macro_use]
extern crate std;

mod http;
mod router;
mod parser;

pub use http::*;
pub use router::*;
pub use parser::*;

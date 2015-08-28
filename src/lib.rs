#![no_std]

#![feature(core)]
#![feature(alloc)]
#![feature(no_std)]
#![feature(collections)]
#![feature(vec_push_all)]
#![feature(fixed_size_array)]

#[macro_use]
extern crate alloc;
#[macro_use]
extern crate collections;

use core::prelude::*;
use core::hash::Hasher;
use core::hash::SipHasher;
use core::array::FixedSizeArray;
use core::fmt::{Formatter};

use collections::vec::*;
use collections::String;
use collections::string::ToString;
use core::str::from_utf8;


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

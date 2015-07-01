#![no_std]

#![feature(core, alloc, no_std, macro_reexport, unboxed_closures, collections, convert, hash, step_by)]

#[macro_use]
extern crate core;
#[macro_use]
extern crate alloc;
#[macro_use]
extern crate collections;


//#[macro_use]
//extern crate nom;

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

/*
use std::str::from_utf8;
use std::collections::HashMap;
*/

mod http;
mod router;
mod parser;

pub use http::*;
pub use router::*;
pub use parser::*;
// #![feature(custom_attribute)]

#[macro_use]
extern crate type_level_values;
#[macro_use]
extern crate derive_type_level;
extern crate core_extensions;

use core_extensions::SelfOps;

use std::fmt::Debug;
use std::marker::PhantomData;
use std::mem;

use type_level_values::prelude::*;


pub mod derived_state;


/////////////////////////////////////////////////////

fn main() {
}

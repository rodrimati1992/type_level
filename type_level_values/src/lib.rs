/*!

Crate for using type level values and functions.

# Introduction 

[For the introduction to this library go here.](./docs/guide/introduction/index.html)

# Guide

This guide will guide any user towards defining type-level-values and  using them,
starting with simple examples,then getting gradually more complex.

The guide is [here](./docs/guide/index.html),
and it starts [here](./docs/guide/introduction/index.html),


# Reference

Derive macros from `derive_type_level`:
    
- [TypeLevel derive macro.](./docs/attribute_typelevel/index.html) 

- [MutConstValue derive macro.](./docs/attribute_mut_const_value/index.html)


Miscelaneous things:

- [Control Flow: 
    On how to do control flow on the type-level.
](./docs/appendix_control_flow/index.html)

- [Privacy: 
    Details on how TypeLevel deals with privacy.
](./docs/reference_privacy/index.html)

- [reading error messages: 
    How to read error messages by the compiler.
  ](./docs/appendix_error_messages/index.html)

- [Patterns: Programming patterns in this library.](./docs/appendix_patterns/index.html)

- [type-level-functions](./docs/appendix_functions/index.html)

# Documentation

For documentation outside of API docs go [here](./docs/index.html).

# Minimum supported Rust version

This package support rust back to 1.20 .
Using a build script to enable features after Rust 1.20.

# no-std support

To use this crate in no_std contexts disable the default-feature.

Disabling the std feature disables these things:

- the MutConstParam methods taking Box/Rc/Arc.


# Cargo Features

"std":Enables standard library support,otherwise uses the core library.Enabled by default.

"serde":Enables serde support.Enabled by default.

"large_tlist":to enable fixed-size impls for type-lists of 
up to 32 elements instead of 16 elements,

*/

#![recursion_limit = "96"]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

// this is pub because it is used by the derive macros to access the standard/core library.
#[doc(hidden)]
#[cfg(feature = "std")]
pub extern crate std as std_;

// this is pub because it is used by the derive macros to access the standard/core library.
#[doc(hidden)]
#[cfg(not(feature = "std"))]
pub extern crate core as std_;

pub extern crate typenum;

#[macro_use]
pub extern crate core_extensions;

#[macro_use]
extern crate derive_type_level;

#[cfg(feature = "serde")]
extern crate serde;

extern crate num_traits;



include!( "./macros/tlist.rs");
include!( "./macros/type_fn.rs");
include!( "./macros/mutator_fn.rs");



#[macro_use]
pub mod type_fn;
pub mod std_ops;
pub mod fn_adaptors;


#[macro_use]
pub mod ops;


pub mod discriminant;
#[macro_use]
pub mod field_traits;
pub mod const_wrapper;
pub mod collection_ops;
pub mod extern_types;
pub mod new_types;
pub mod runtime_value;
pub mod std_types;
pub mod user_traits;
pub mod util_types;
pub mod initialization;

#[cfg(feature="priv_doctests")]
pub mod derive_tests;

#[cfg(test)]
pub(crate) mod testing;

#[path = "../docs/mod.rs"]
pub mod docs;

pub mod prelude;

pub(crate) mod type_level_values {
    pub(crate) use super::*;
}

#[doc(hidden)]
pub mod reexports ;

// emulating Rust 2018 edition's crate:: prefix.
// Used instead of it so as to stay compatible with Rust pre-1.30 .
mod crate_ {
    pub(crate) use super::*;
}





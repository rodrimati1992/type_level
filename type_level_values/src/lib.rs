/*!

Crate for using type level values and functions.

# Documentation

For documentation outside of API docs go [here](./docs/index.html).

# Minimum supported Rust version

This package support rust back to 1.20 .
Using a build script to enable features after Rust 1.20.

# no-std support

To use this crate in no_std contexts disable the default-feature.

# Cargo Features

"std":Enables standard library support.Enabled by default.

"serde":Enables serde support.Enabled by default.

*/

#![recursion_limit = "160"]
#![cfg_attr(not(feature = "std"), no_std)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]

#[cfg(feature = "std")]
pub extern crate std as std_;

#[cfg(not(feature = "std"))]
pub extern crate core as std_;

pub extern crate typenum;

#[macro_use]
pub extern crate core_extensions;

#[macro_use]
pub extern crate derive_type_level;

#[cfg(feature = "serde")]
extern crate serde;

extern crate num_traits;

#[macro_use]
pub mod type_fn;
pub mod fn_types;
pub mod fn_adaptors;


#[macro_use]
pub mod ops;
#[macro_use]
pub mod macros;


pub mod discriminant;
#[macro_use]
pub mod field_traits;
pub mod const_wrapper;
pub mod extern_types;
pub mod new_types;
pub mod runtime_value;
pub mod std_types;
pub mod user_traits;
pub mod util_types;
pub mod initialization;

#[path = "../docs/mod.rs"]
pub mod docs;

pub mod prelude;

pub(crate) mod type_level_values {
    pub(crate) use super::*;
}

#[doc(hidden)]
pub mod reexports ;

// emulating Rust 2018 edition's crate:: prefix.
mod crate_ {
    pub(crate) use super::*;
}

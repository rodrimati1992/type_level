/*! 
This crate contains many examples of defining and using type-level values and functions.

You are free to copy any of the examples here,improvements to this examples should be (preferably) an issue or a pull request.

This is a library purely so that `docs.rs` can show documentation for it,do not use this crate as a dependency.

To run the examples do `cargo run -- ` and then add one of the arguments from the help message.

*/

#![recursion_limit = "160"]

#[macro_use]
extern crate type_level_values;

#[macro_use]
extern crate derive_type_level;
extern crate derive_type_level_lib;

extern crate core_extensions;
extern crate num_traits;
extern crate take_mut;

extern crate serde;
extern crate serde_json;

pub mod _01_deserialize_uninitialized;
pub mod _02_mut_wrapper;
pub mod _03_vis_wrapper;
pub mod _04_typesafe_builder;
pub mod _05_capabilities;
pub mod _06_channel;
pub mod _07_split_mut;
pub mod _08_ranged_int;
pub mod _09_type_hof;
#[cfg(rust_1_26)]
pub mod _10_state_machine;
pub mod playground_01;
pub mod playground_02;
pub mod syntax_01_construct;

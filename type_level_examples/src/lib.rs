/*! 
This crate contains many examples of defining and using type-level values and functions.

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
pub mod _10_state_machine;
pub mod playground_01;
pub mod playground_02;
pub mod syntax_01_construct;
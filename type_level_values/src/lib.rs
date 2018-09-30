//!
//! This crate contains types/traits/macros related to Compiletime types and their users.
//!
//! # Documentation
//!
//! For documentation outside of the API docs of type_level_values go [here](./docs/index.html).
//!
//! # Minimum supported Rust version
//!
//! This package support rust back to 1.20 .
//! Using a build script to enable features after Rust 1.20.
//!
//! # no-std support
//!
//! To use this crate in no_std contexts disable the default-feature.
//!
//! # Cargo Features
//!
//! "std":Enables standard library support.Enabled by default.
//!
//! "serde":Enables serde support.Disabled by default.
//!
//! # Documentation
//!
//! The documentation of this crate is in
//!
//!
//!

#![recursion_limit = "128"]
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
pub mod ops;
#[macro_use]
pub mod macros;

pub mod enum_stuff;
#[macro_use]
pub mod field_traits;
pub mod const_wrapper;
pub mod extern_types;
pub mod new_types;
pub mod runtime_value;
pub mod std_types;
pub mod user_traits;
pub mod util_types;

#[path = "../docs/mod.rs"]
pub mod docs;


pub mod prelude;

pub(crate) mod type_level_values {
    pub(crate) use super::*;
}

#[doc(hidden)]
pub mod reexports {
    pub use std_;

    pub use std_::marker::PhantomData;

    pub use core_extensions::type_level_bool;

    pub use core_extensions::prelude::*;
    pub use core_extensions::{MarkerType, TypePanic, VariantPhantom, Void};

    pub mod _constraints {
        pub use core_extensions::type_level_bool::Boolean;
        pub use std_types::cmp_ordering::OrderingTrait;
    }

    pub use enum_stuff::{Discriminant, DiscriminantFor, GetDiscriminant};
    pub use ops::fn_adaptors::IgnoreFirst;
    pub use ops::{
        AsTList, AsTList_, ConstEq as __CEq, ConstEq_, ConstOrd as __COrd, ConstOrd_, TypeFn,
        VariantAsTList, VariantAsTList_,
    };

    pub use user_traits;

    pub use const_wrapper::{AsConstWrapper, PhantomWrapper};
    #[cfg(rust_1_22)]
    pub use runtime_value::IntoConstant;
    pub use runtime_value::{
        AssertConstType, ConstType, ConstTypeOf, ConstTypeOf_, ConstValue, DerivedTraits,
        FromRuntime, IntoConstType_, IntoRuntime,
    };

    pub use field_traits::{
        Field_, GetFieldRuntime_, GetField_, InitializationValues, SetField_,
    };

    pub use new_types::{TList, TNil};

    pub mod typenum_reexports {
        pub use typenum::consts::{
            U0, U1, U10, U11, U12, U13, U14, U15, U16, U17, U18, U19, U2, U20, U21, U22, U23, U24,
            U25, U26, U27, U28, U29, U3, U30, U31, U32, U33, U34, U35, U36, U37, U38, U39, U4, U40,
            U41, U42, U43, U44, U45, U46, U47, U48, U49, U5, U50, U51, U52, U53, U54, U55, U56,
            U57, U58, U59, U6, U60, U61, U62, U63, U64, U7, U8, U9,
        };

    }

}

// emulating Rust 2018 edition's crate:: prefix.
mod crate_ {
    pub(crate) use super::*;
}

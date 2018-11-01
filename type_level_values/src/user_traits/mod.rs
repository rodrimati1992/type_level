/*! 
Traits for regular types which have a Const-parameter.

The most important items here are:

- [MutConstParam](./mut_const_param/trait.MutConstParam.html):
provides methods to mutate the Const-parameter of Self.

- [MCPBounds](./mut_const_param/trait.MCPBounds.html):
trait for the bounds of every `MutConstParam` method.


# Glosary

ConstConstructor:is a marker type which,when provided a Const-parameter,
outputs a type with that Const-parameter.
Eg:RectangleCC in type_level_examples::_03_vis_wrapper.


ConstMethod:refers to a marker type which is used to implement an operation
on a type with a Const-parameter.
Eg:MakeInaccessible/Reset for Rectangle\<I> in type_level_examples::_03_vis_wrapper.

Op:is another name for a ConstMethod.

This:is the common way to refer to the type that a ConstMethod is implemented for.
It always has a Const-parameter.
Eg:Rectangle\<I> in type_level_examples::_03_vis_wrapper.




*/

pub mod allowed_conversions_type;
pub mod builtin_constmethods;
pub mod const_methods;
pub mod const_traits;
pub mod example_const_user;
pub mod mut_const_param;

pub use self::mut_const_param::{MCPBounds, MutConstParam};

pub use self::const_traits::ConstConstructor;

pub use self::allowed_conversions_type::allowed_conversions;

use crate_::fn_adaptors::*;
use crate_::ops::*;
use prelude::*;

use crate_::field_traits::GetField_;

use crate_::new_types::type_list::{TList, TNil};

use core_extensions::marker_traits::MarkerType;
use core_extensions::type_level_bool::{False, True};
use core_extensions::utils::transmute_ignore_size;

mod sealed {
    pub trait Sealed {}
}
use self::sealed::Sealed;

/*! 
Traits for regular types which have a ConstValue-parameter.

The most important items here are:

- [MutConstParam](./mut_const_param/trait.MutConstParam.html):
provides methods to mutate the ConstValue-parameter of Self.

- [MCPBounds](./mut_const_param/trait.MCPBounds.html):
trait for the bounds of every `MutConstParam` method.


# Glosary

Mutator Function:
    refers to a TypeFn_  allowed to mutate the ConstValue of a type with 
    `impl AllowMutatorFn<Func> for SomeType`.
Eg:MakeInaccessible/Reset for Rectangle\<I> in type_level_examples::_03_vis_wrapper.

Op:is another name for a Mutator Function.

This:
    is the common way to refer to a type with a ConstValue-parameter in this module.
Eg:Rectangle\<I> in type_level_examples::_03_vis_wrapper.




*/

pub mod const_traits;
pub mod example_const_user;
pub mod functions;
pub mod mut_const_param;
pub mod self_constructors_type;

pub use self::mut_const_param::{MCPBounds, MutConstParam};

pub use self::self_constructors_type::allowed_self_constructors;

pub use self::const_traits::{AllowMutatorFn, MutatorFnAttrs};

pub use self::functions::{AdaptFn, AdaptUnary, ReplaceWithParamFn};

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

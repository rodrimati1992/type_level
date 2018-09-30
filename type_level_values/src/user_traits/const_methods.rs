//! Contains all the traits related to Const-methods.
//!
//! Const-methods are constrained ways in which a type allows mutating its ConstValue-parameter,
//! declared using the `const_method` macro,
//! implementing the ConstMethod marker trait.
//!
//!

use super::*;

use user_traits::allowed_conversions_type::AllowedConversionsTrait;

use user_traits::const_traits::{AllowOp, AllowedOps};

#[derive(TypeLevel)]
pub enum ExtensionMethodKind {
    #[typelevel(
        doc = "\
               Used in ConstMethod::MethodKind,\
               for a built-in ConstMethod,defined in type_level_values.\
               "
    )]
    Builtin,

    #[typelevel(
        doc = "\
               Used in ConstMethod::MethodKind,\
               for a regular extension ConstMethod,defined anywhere.\
               "
    )]
    RegularExt,
}

/// Marker trait for Const-methods.
pub trait ConstMethod {}

/// Trait for extension Const-methods
pub trait ExtensionConstMethod: ConstMethod {
    /// Describes what kind of extension method this is,
    /// whether it is a built-in or a regular extension method.
    ///
    ///
    ///
    type MethodKind: type_level_ExtensionMethodKind::ExtensionMethodKindTrait;
}

///////////////////////////////////////////////////////////////////////////////////

/** 
Marker trait for extension Const-methods.

This is automatically implemented for Const-methods in the const_fn macro
by writing `extension_method=true;`.

# Example

```
# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;

use type_level_values::user_traits::example_const_user::ConstUserExtMeth;


const_method!{
    type ConstConstructor[]=( T )
    type AllowedConversions=( allowed_conversions::All )

    extension_method=true;

    /// Converts a constant to a pair of the same constant
    pub fn PairUp[I](I,()){ (I,I) }
}

# fn main(){

let value=ConstUserExtMeth::<()>::new();

let value:ConstUserExtMeth<((),())>=value.mutparam(PairUp,Default::default());
    

# }


```
*/
pub trait ConstMethod_RegularExt:
    ExtensionConstMethod<MethodKind = type_level_ExtensionMethodKind::RegularExt>
{
}

///////////////////////////////////////////////////////////////////////////////////

/**
Marker trait for extension Const-methods defined in the type_level_values crate
which can be called on all types which contain a Const-parameter.

This is automatically implemented for Const-methods in the const_fn macro
by writing `extension_method=internal_blanket_impl;`.

Builtin extension Const-methods are generally unsafe to instantiate 
due to being able to violate the invariants of the type they change the Const-parameter of.
This means that they should generally be reserved to private uses ,
as in where the type was defined.

This trait is Sealed and cannot be implemented outside the type_level_values crate.

# Example

```
# #[macro_use]
# extern crate type_level_values;

# #[macro_use]
# extern crate derive_type_level;

# use type_level_values::prelude::*;

use type_level_values::user_traits::builtin_constmethods::Ext_SetConstParam;

#[derive(ConstConstructor)]
#[cconstructor(
    Type = "ConstUser",
    ConstParam = "C",
    extension_methods="false",
)]
pub struct ConstUserInner<C>{
    const_:PhantomWrapper<C>,
}

# fn main(){

let value:ConstUser<()>=ConstUser{ const_:Default::default() };

let value:ConstUser<u32>=unsafe{
    value.mutparam(Ext_SetConstParam::new(), u32::T )
};

# }


```
*/
pub trait ConstMethod_BuiltinExt:
    Sealed + ExtensionConstMethod<MethodKind = type_level_ExtensionMethodKind::Builtin>
{
}

////////////////////////////////////////////////////////////////////////////////////

impl<Op, ThisCC> AllowOp<Op> for ThisCC
where
    ThisCC: ConstConstructor,
    Op: ExtensionConstMethod,
    AllowHelper: TypeFn_<(ThisCC, Op, Op::MethodKind)>,
{}

type_fn!{
    pub fn
    AllowHelper[ThisCC,Op](ThisCC,Op,type_level_ExtensionMethodKind::Builtin)
    where [
        Op:ConstMethod_BuiltinExt,
    ]{ () }

    AllowHelper[ThisCC,Op](ThisCC,Op,type_level_ExtensionMethodKind::RegularExt)
    where [
        Op:ConstMethod_RegularExt,
        ThisCC:AllowedOps<ExtensionMethods=True>
    ]{ () }
}

///////////////////////////////////////////////////////////////////////////////////

/// The attributes for a ConstMethod.
///
/// The impl for this trait should only be constrained by the implied bounds
/// (as in the ones in the declaration of the type).
///
/// For more information please refer to the [module-level documentation](./index.html).
pub trait OpAttrs: ConstMethod {
    /// The types of conversions allowed from This to MutConstThis<This,Param>.
    ///
    /// The 3 types of conversions possible are:
    ///
    /// - by reference:eg:
    /// & This to & MutConstThis\<This,Param> or
    /// Rc<This> to Rc\<MutConstThis<This,Param>> or
    /// Arc<This> to Arc\<MutConstThis<This,Param>> .
    ///
    /// - by mutable reference:eg: &mut This to &mut MutConstThis\<This,Param>
    ///
    /// - by value:eg:
    ///     This to MutConstThis<This,Param> or
    ///     Box<This> to Box<MutConstThis<This,Param>>
    ///
    ///
    /// Default Type:`allowed_conversions::All`
    type Conversions: AllowedConversionsTrait;
}

///////////////////////////////////////////////////////////////////////////////////

/// Implemented by ConstMethods to calculate the next value of the constant,
pub trait ComputeConstParam_<Const, Msg>: Sized + OpAttrs + ConstMethod {
    /// A mutated version of the Const-parameter.
    type Output;
}

pub type ComputeConstParam<Op, Const, Msg> = <Op as ComputeConstParam_<Const, Msg>>::Output;

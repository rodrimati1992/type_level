//! Traits implemented by the MutConstValue derive macro.
//!

use super::*;

use user_traits::self_constructors_type::AllowedConstructorsTrait;

/// The ConstValue-parameter associated to Self.
pub trait GetConstParam_ {
    type Const;
}

/// The ConstConstructor for this type.
#[doc(hidden)]
pub trait GetConstConstructor_: GetConstParam_ {
    /// the ConstConstructor for this type.
    type Constructor: ConstConstructor;
}

/// Marker trait for ConstConstructors.
///
/// ConstConstructors are types which,when provided a ConstValue-parameter,
/// output another type with that ConstValue-parameter.
///
#[doc(hidden)]
pub trait ConstConstructor: Sized {}

/// Applies a ConstValue-parameter to a ConstConstructor.
///
/// Returning a type with that ConstValue-parameter.
///
/// # Safety
///
/// The Applied type parameter is not guaranteed to be memory-layout compatible among any
/// applications of the ConstValue-parameter.
///
/// To check memory-layout compatiblity please use the ConstLayoutIndependent trait.
///
///
///
#[doc(hidden)]
pub trait ApplyConstParam_<Param>: ConstConstructor {
    type Applied: GetConstParam_<Const = Param> + GetConstConstructor_<Constructor = Self>;
}

/// Gets the ConstValue-parameter to `This`.
pub type GetConstParam<This> = <This as GetConstParam_>::Const;

/// Gets the ConstConstructor for `This`.
#[doc(hidden)]
pub type GetConstConstructor<This> = <This as GetConstConstructor_>::Constructor;

/// Applies the ConstValue-parameter to the ConstConstructor ,
/// returning a type containing the ConstValue-parameter.
#[doc(hidden)]
pub type ApplyConstParam<Constructor, Const> = <Constructor as ApplyConstParam_<Const>>::Applied;

///////////////////////////////////////////////////////////////////////////////////


/**
Marker trait for types whose memory layout does not change when the ConstValue-parameter does.

# Safety

Implementors of this trait must ensure that the ConstValue-parameter is not used to
determine the layout of the type.

To ensure that the ConstValue-parameter does not affect the layout:

- Use the `MutConstValue` derive macro which automatically implements this trait.

- Or implement this trait manually.

# Manual implementors

Manual implementors of this trait must constrain every field which
mentions the ConstValue-parameter implements ConstLayoutIndependent< NewFieldType >,
and optionally SameConstConstructor< NewFieldType >
(if one wants the ConstConstructor to stay the same) .<br>
NewFieldType is the type of the same field in `Other`.

# Incompatible memory layouts 

Some memory layouts are incompatible,
this is caused by having a field that mentions the ConstValue-parameter
and does not implement ConstLayoutIndependent.

### Example

```compile_fail

# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;

use type_level_values::user_traits::example_const_user::{
    StoredInside,
    ChangeParam,
};

fn main(){
    let wrapper=StoredInside::new(100,());
    let wrapper=MutConstParam::mutparam(wrapper,ChangeParam::NEW , String::T );
}

```

### Example

```compile_fail

# #[macro_use]
# extern crate type_level_values;

# use type_level_values::prelude::*;

# use type_level_values::user_traits::example_const_user::{
#     StoredInside,
#     ChangeParam,
# };

# fn main(){

let wrapper=StoredInside::new(100,"hello");
MutConstParam::mutparam(wrapper,ChangeParam::NEW , String::T );

# }

```

### Example


```compile_fail

# #[macro_use]
# extern crate derive_type_level;

# #[macro_use]
# extern crate type_level_values;

use type_level_values::prelude::*;
use type_level_values::user_traits::ReplaceWithParamFn;

#[derive(MutConstValue)]
#[mcv(
    derive(Debug,Copy,Clone,Default),
    Type= "ValueWrapper", ConstValue = "I"
)]
pub struct ValueWrapperInner<T,I> {
    pub value:T,
    pub marker: I,
}


impl AllowMutatorFn<ReplaceWithParamFn> for SideEffectful_T {
    type AllowedSelf=allowed_constructors::All;
}


/*

// This is the type alias that MutConstValue generates.

pub type ValueWrapper<I>=ValueWrapper_Ty<ConstWrapper<I>>;

*/

# fn main(){


let wrapper_1=ValueWrapper_Ty{ value:100, marker:String::new() };
let wrapper_2=MutConstParam::mutparam( wrapper_1 , ReplaceWithParamFn::NEW , u32::T );

# }

```

*/
pub unsafe trait ConstLayoutIndependent<Other: ?Sized> {}

unsafe impl<This:?Sized, Other:?Sized> 
    ConstLayoutIndependent<PhantomData<Other>> for PhantomData<This>
{}

unsafe impl<This:?Sized, Other:?Sized> 
    ConstLayoutIndependent<ConstWrapper<Other>> for ConstWrapper<This>
{}

///////////////////////////////////////////////////////////////////////////////////

/// Mutates the Const-value associated with a regular type.
///
/// # Safety
///
/// The Output type is not guaranteed to be memory-layout compatible with Self.
///
/// To check memory-layout compatiblity please use the ConstLayoutIndependent trait.
///
///
pub trait SetConstParam_<Value> {
    /// This is Self with the ConstValue-parameter replaced with `Value`
    type Output: GetConstParam_< Const = Value>;
}

impl<This, Value> SetConstParam_<Value> for This
where
    This: GetConstConstructor_,
    This::Constructor: ApplyConstParam_<Value>,
    Self: ConstLayoutIndependent<ApplyConstParam<This::Constructor, Value>>,
{
    type Output = ApplyConstParam<This::Constructor, Value>;
}

/// Type alias for mutating the ConstValue-parameter of `This` to `Value`.
pub type SetConstParam<This, Value> = <This as SetConstParam_<Value>>::Output;

///////////////////////////////////////////////////////////////////////////////////

/// Asserts that the memory layout of the `Field` field is the same as the one in  `Other`.
///
#[doc(hidden)]
pub trait SameFieldLayout<Field, Other: ?Sized> {}

impl<Field, This: ?Sized, Other: ?Sized, ThisField: ?Sized, OtherField: ?Sized>
    SameFieldLayout<Field, Other> for This
where
    // Always wrap the type of the field in a PhantomData to be able to use a ?Sized type,
    // eventually replace it with `struct MaybeSized<T:?Sized>(PhantomData<T>);`.
    Self: GetField_<Field, Output = PhantomData<ThisField>>,
    Other: GetField_<Field, Output = PhantomData<OtherField>>,
    ThisField: ConstLayoutIndependent<OtherField>,
{}

/// Asserts that the ConstConstructor of Self is the same as the one of Other.
pub trait SameConstConstructor<Other: ?Sized> {}

impl<This: ?Sized, Other: ?Sized> SameConstConstructor<Other> for This
where
    This: GetConstConstructor_,
    Other: GetConstConstructor_<Constructor = GetConstConstructor<This>>,
{}

////////////////////////////////////////////////////////////////////////////////////

/**
The attributes for a mutator function.

It is necessary to implement this directly on the function because otherwise Rust 
doesn't know the value of SelfConstructors in generic methods.
*/
pub trait MutatorFnAttrs{
    /**
    The classes of type containing self allowed in MutConstParam methods.
    
    The 3 classes (with 2*2*2 possible combinations) are:
    
    - by reference:eg:
    & This to & MutConstThis\<This,NewConstValue> or
    Rc<This> to Rc\<MutConstThis<This,NewConstValue>> or
    Arc<This> to Arc\<MutConstThis<This,NewConstValue>> .
    
    - by mutable reference:eg: &mut This to &mut MutConstThis\<This,NewConstValue>
    
    - by value:eg:
        This to MutConstThis<This,NewConstValue> or
        Box<This> to Box<MutConstThis<This,NewConstValue>>
    
    The values of this associated type can be:

    - allowed_self_constructors::All

    - allowed_self_constructors::ByRef

    - allowed_self_constructors::ByMut

    - allowed_self_constructors::ByVal

    */
    type AllowedSelf: AllowedConstructorsTrait;
}



/**
Allows the `Func` TypeFn_ to mutate the ConstValue parameter of Self
*/
pub trait AllowMutatorFn<Func>{}


type_fn!{
    pub fn GetAllowedSelfOp[Func](Func)
    where[ Func:MutatorFnAttrs ]
    { Func::AllowedSelf }
}

////////////////////////////////////////////////////////////////////////////////////


// /**
// Marker trait for unit structs which represent a generic type
// (where the generic parameters don't matter).
// */
// pub trait TypeMarker{}

// /// Gets the TypeMarker which represent Self.
// pub trait TypeMarkerOf_{
//     type Marker:TypeMarker;
// }

// /// Gets the TypeMarker which represent Self.
// pub type TypeMarkerOf<This>=
//     <This as TypeMarkerOf_>::Marker;

// type_fn!{
//     pub fn TypeMarkerOp[This](This)
//     where[ This:TypeMarkerOf_ ]
//     { This::Marker }
// }
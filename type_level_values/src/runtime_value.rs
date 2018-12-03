/*!
Traits for converting between ConstValues and constants/runtime values.


# Rust versions

The IntoConstant trait requires Rust 1.22 to allow constructing Drop/not Copy types.


*/

use core_extensions::MarkerType;
use prelude::*;
use std_::ops::BitAnd;

/// Represents the "type" of a ConstValue,
/// eg:SignedInteger/BooleanType/OptionType/ResultType.
pub trait ConstType {}

/// A compile-time value.Eg:True,False,U0,U1,ConstRange<U0,U100>.
pub trait ConstValue: MarkerType + ConstTypeOf_ {}

impl<This> ConstValue for This where This: MarkerType + ConstTypeOf_ {}

/// The ConstType of this ConstValue.
pub trait ConstTypeOf_ {
    ///
    type Type: ConstType;
}

/// The ConstType of `Type`.
pub type ConstTypeOf<This> = <This as ConstTypeOf_>::Type;

/// The ConstType equivalent of Self.
pub trait IntoConstType_ {
    type ToConst: ConstType;
}

/// The ConstType equivalent of `This`.
pub type FromRuntime<This> = <This as IntoConstType_>::ToConst;

/// Converts a ConstValue into a runtime value
pub trait IntoRuntime<To> {
    /// Gets the runtime equivalent of this ConstValue.
    fn to_runtime() -> To;

    /// Gets the runtime equivalent of this ConstValue.
    #[inline(always)]
    fn to_runtime_ty(_: VariantPhantom<To>) -> To {
        Self::to_runtime()
    }

    /// Gets the runtime equivalent of this ConstValue.
    #[inline(always)]
    fn into_runtime(&self) -> To {
        Self::to_runtime()
    }

    /// Gets the runtime equivalent of this ConstValue.
    #[inline(always)]
    fn into_runtime_ty(&self, _: VariantPhantom<To>) -> To {
        Self::to_runtime()
    }
}

#[cfg(rust_1_22)]
/// Converts a ConstValue into a constant
///
pub trait IntoConstant<To> {
    const VALUE: To;
}

/////////////////////////////////////////////////////////////////////////////

/// Trait alias,for ConstValues created by the TypeLevel derive macro.
pub trait NoGetDiscriminant:
    Copy + Clone + Send + Sync + Sized + Default + MarkerType + ConstValue + ConstTypeOf_
{
}

impl<This> NoGetDiscriminant for This where
    This: Copy + Clone + Send + Sync + Sized + Default + MarkerType + ConstValue + ConstTypeOf_
{}

/// Trait alias,for ConstValues created by the TypeLevel derive macro,where no impls are disabled.
///
pub trait DerivedTraits: NoGetDiscriminant + GetDiscriminant {}

impl<This> DerivedTraits for This where This: NoGetDiscriminant + GetDiscriminant {}

/////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////

type_fn!{
    /// The ConstType of this ConstValue.
    alias ConstTypeOfOp[This]::Type =ConstTypeOf_
}

type_fn!{
    /// The ConstType equivalent of Self.
    alias IntoConstTypeOp[This]::ToConst =IntoConstType_
}

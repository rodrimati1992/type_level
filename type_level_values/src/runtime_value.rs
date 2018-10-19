/*!
Traits for converting between type-level-values and constants/runtime values.


# Rust versions

The IntoConstant trait requires Rust 1.22 to allow constructing Drop/not Copy types.


*/

use core_extensions::MarkerType;
use prelude::*;
use std_::ops::BitAnd;

/// Represents a compile-time type,like SignedInteger/BooleanType/OptionType/ResultType.
pub trait ConstType {}

/// A compile-time value.
pub trait ConstValue: MarkerType + ConstTypeOf_ {}

impl<This> ConstValue for This where This: MarkerType + ConstTypeOf_ {}

/// The ConstType of this Const-value.
pub trait ConstTypeOf_ {
    ///
    type Type: ConstType;
}

/// The ConstType of `Type`.
pub type ConstTypeOf<This> = <This as ConstTypeOf_>::Type;

type_fn!{alias ConstTypeOfOp[This]::Type =ConstTypeOf_ }

/// The ConstType equivalent of Self.
pub trait IntoConstType_ {
    type ToConst: ConstType;
}

type_fn!{alias IntoConstTypeOp[This]::ToConst =IntoConstType_ }

/// The ConstType equivalent of `This`.
pub type FromRuntime<This> = <This as IntoConstType_>::ToConst;

/// Converts a compile-time value into a runtime value
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
/// Converts a compile-time value into a runtime value
pub trait IntoConstant<To, From = Self> {
    const VALUE: To;
}

/////////////////////////////////////////////////////////////////////////////

/// Trait alias for the variants/the Const\<DerivingType> created by the TypeLevel macro.
///
pub trait DerivedTraits:
    Copy + Clone + 
    Send + Sync + 
    Sized + Default + 
    MarkerType + ConstValue + 
    ConstTypeOf_ + GetDiscriminant
{
}

impl<This> DerivedTraits for This where
    This: 
        Copy + Clone + 
        Send + Sync + 
        Sized + Default + 
        MarkerType + ConstValue + 
        ConstTypeOf_ + GetDiscriminant
{}

/////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////

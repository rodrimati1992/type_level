use core_extensions::MarkerType;
use prelude::*;
use std_::ops::BitAnd;

type_fn!{
    pub fn AssertConstType[T](T)where[ T:ConstType ]{ () }
}

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

/// The ConstType equivalent of Self.
pub trait IntoConstType_<From = Self> {
    type ToConst: ConstType;
}

/// The ConstType equivalent of `This`.
pub type FromRuntime<This> = <This as IntoConstType_>::ToConst;

/// Converts a compile-time value into a runtime value
pub trait IntoRuntime<To, From = Self> {
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

impl<CType, From, To> IntoRuntime<To, From> for CType
where
    From: ConstTypeOf_<Type = CType> + IntoRuntime<To>,
    CType: ConstType,
{
    #[inline(always)]
    fn to_runtime() -> To {
        From::to_runtime()
    }
}

#[cfg(rust_1_22)]
/// Converts a compile-time value into a runtime value
pub trait IntoConstant<To, From = Self> {
    const VALUE: To;
}

//////////////////////////////////////////////////////////////

/// The identity conversion module,
/// use it by using the #[typelevel(delegate(runtime_conv="ConvIdentity"))] attribute.
///
pub struct ConvIdentity;

impl<To> IntoRuntime<To, To> for ConvIdentity
where
    To: ConstValue,
{
    fn to_runtime() -> To {
        To::MTVAL
    }
}

/////////////////////////////////////////////////////////////////////////////

/// Traits derived in the TypeLevel macro.
pub trait DerivedTraits:
    Copy + Clone + MarkerType + ConstValue + ConstTypeOf_ + GetDiscriminant
{
}

impl<This> DerivedTraits for This where
    This: Copy + Clone + MarkerType + ConstValue + ConstTypeOf_ + GetDiscriminant
{}

/////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////

/////////////////////////////////////////////////////////////////////////////

//! Traits implemented by the ConstConstructor derive macro.
//!

use super::*;

use user_traits::allowed_conversions_type::AllowedConversionsTrait;

/// The Const-parameter associated to Self.
pub trait GetConstParam_ {
    type Const;
}

/// The ConstConstructor for this type.
pub trait GetConstConstructor_: GetConstParam_ {
    /// the ConstConstructor for this type.
    type Constructor: ConstConstructor;
}

/// Marker trait for ConstConstructors.
///
/// ConstConstructors are types which,when provided a Const-parameter,
/// output another type with that Const-parameter.
///
pub trait ConstConstructor: Sized {}

/// Applies a Const-parameter to a ConstConstructor.
///
/// Returning a type with that Const-parameter.
///
/// # Safety
///
/// The Applied type parameter is not guaranteed to be memory-layout compatible among any
/// applications of the Const-parameter.
///
/// To check memory-layout compatiblity please use the ConstLayoutIndependent trait.
///
///
///
pub trait ApplyConstParam_<Param>: ConstConstructor {
    type Applied: GetConstParam_<Const = Param> + GetConstConstructor_<Constructor = Self>;
}

/// Gets the Const-parameter to `This`.
pub type GetConstParam<This> = <This as GetConstParam_>::Const;

/// Gets the ConstConstructor for `This`.
pub type GetConstConstructor<This> = <This as GetConstConstructor_>::Constructor;

/// Applies the Const-parameter to the ConstConstructor ,
/// returning a type containing the Const-parameter.
pub type ApplyConstParam<Constructor, Const> = <Constructor as ApplyConstParam_<Const>>::Applied;

///////////////////////////////////////////////////////////////////////////////////

/// Marker trait for types whose memory layout does not change when the Const-parameter does.
///
/// # Safety
///
/// Implementors of this trait must ensure that the Const-parameter is not used to
/// determine the layout of the type.
///
/// To ensure that the Const-parameter does not affect the layout:
///
/// - Use the ConstConstructor derive macro which automatically implements this trait.
///
/// - Or implement this trait manually.
///
/// # Manual implementors
///
/// Manual implementors of this trait must constrain every field which
/// mentions the Const-parameter implements ConstLayoutIndependent< NewFieldType >,
/// and optionally SameConstConstructor< NewFieldType >
/// (if one wants the ConstConstructor to stay the same) .<br>
/// NewFieldType is the type of the same field in `Other`.
///
///
pub unsafe trait ConstLayoutIndependent<Other: ?Sized> {}

/// All MarkerType are interchangeable when it comes to memory layout.
unsafe impl<This, Other> ConstLayoutIndependent<Other> for This
where
    This: MarkerType,
    Other: MarkerType,
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
pub trait SetConstParam_<Value>: GetConstConstructor_ {
    /// This is Self with the ConstValue-parameter replaced with `Value`
    type Output: GetConstConstructor_<Constructor = Self::Constructor, Const = Value>;
}

impl<This, Value> SetConstParam_<Value> for This
where
    This: GetConstConstructor_,
    This::Constructor: ApplyConstParam_<Value>,
    Self: ConstLayoutIndependent<ApplyConstParam<This::Constructor, Value>>,
{
    type Output = ApplyConstParam<This::Constructor, Value>;
}

/// Type alias for mutating the Const-parameter of `This` to `Value`.
pub type SetConstParam<This, Value> = <This as SetConstParam_<Value>>::Output;

///////////////////////////////////////////////////////////////////////////////////

/// Asserts that the memory layout of the `Field` field is the same as the one in  `Other`.
///
pub trait SameFieldLayout<Field, Other: ?Sized> {}

impl<Field, This: ?Sized, Other: ?Sized, ThisField: ?Sized, OtherField: ?Sized>
    SameFieldLayout<Field, Other> for This
where
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

///////////////////////////////////////////////////////////////////////////////////

///////////////////////////////////////////////////////////////////////////////////

/// This trait declares that `Op` (a ConstMethod) is allowed to mutate the Const-parameter
/// of the type Self is a ConstConstructor of.
///
/// There are 3 ways a type implements this trait:
///
/// - Self implements AllowOp<SomeOp>,this is done in the const_method
///     macro for regular ConstMethods.
///
/// - An Op implements ConstBuiltinMethod,only implementable inside type_level_values,
/// defining a ConstMethod for all types.
///
/// - An Op implements ConstExtensionMethod,
/// defining a ConstMethod for all types which allow extension Const-methods.
///
///
pub trait AllowOp<Op>: ConstConstructor {}

///////////////////////////////////////////////////////////////////////////////////

/// Whether extension ConstMethods are allowed to mutate the Const-parameter
/// of the type Self is a ConstConstructor of.
///
/// The associated type here is either True/False.
pub trait AllowedOps: ConstConstructor {
    type ExtensionMethods;
}

///////////////////////////////////////////////////////////////////////////////////

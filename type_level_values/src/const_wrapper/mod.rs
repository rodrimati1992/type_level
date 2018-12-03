/*! 
Contains ConstWrapper,
for wrapping ConstValues so that they implement many std library traits.

To instantiate a ConstWrapper use either `Value::CW` or `value.to_cw()`.

*/

use prelude::*;

use crate_::discriminant::Discriminant;
use crate_::field_traits::{
    GetField, GetFieldRuntime, GetFieldRuntime_, GetField_, SetField, SetField_,
};
use crate_::initialization::InitializationValues;
use crate_::runtime_value::DerivedTraits;
use crate_::user_traits::const_traits;

use std_types::cmp_ordering::OrderingTrait;

use core_extensions::MarkerType;
use core_extensions::{CallInto, CallMut, CallRef};

use std_::ops::{Deref, Index};
use std_::{cmp, fmt, hash};

//////////////////////////////////////////////////////////////////////////////////////

mod sealed {
    pub trait Sealed {}
}
use self::sealed::Sealed;

/// Creates a ConstWrapper from a type implementing WrapperTrait.
pub type ConstWrapperFromTrait<ConstParam> = ConstWrapper<UnwrapConst<ConstParam>>;

//////////////////////////////////////////////////////////////////////////////////////

/// Const-type of all ConstWrapper values.
#[derive(Debug, Copy, Clone, Default)]
pub struct ConstWrapperType;

impl ConstType for ConstWrapperType {}

//////////////////////////////////////////////////////////////////////////////////////

/// Trait used to access the type parameters of ConstWrapper in a generic context.
pub trait WrapperTrait: Sealed + DerivedTraits<Type = ConstWrapperType> {
    type ConstValue: ?Sized;
}

impl<T: ?Sized> WrapperTrait for ConstWrapper<T> {
    type ConstValue = T;
}

impl<T: ?Sized> GetDiscriminant for ConstWrapper<T> {
    type Discriminant = Discriminant<ConstWrapperType, ConstWrapperType, U0>;
    type UIntDiscr = U0;
    type Variant = ConstWrapperType;
}

impl<T: ?Sized> Sealed for ConstWrapper<T> {}

/// Gets the `ConstValue` associated type from a WrapperTrait.
pub type UnwrapConst<ConstWrapper> = <ConstWrapper as WrapperTrait>::ConstValue;

//////////////////////////////////////////////////////////////////////////////////////

/// Wrapper type for ConstValues,
/// always implements standard library traits,
/// delegating many trait (from type_level_values) impls to the wrapped ConstValue.
///
pub struct ConstWrapper<Compiletime: ?Sized>(VariantPhantom<Compiletime>);

unsafe impl<T: ?Sized> MarkerType for ConstWrapper<T> {}

unsafe impl<Compiletime: ?Sized> Send for ConstWrapper<Compiletime> {}
unsafe impl<Compiletime: ?Sized> Sync for ConstWrapper<Compiletime> {}

impl<T: ?Sized> Default for ConstWrapper<T> {
    #[inline(always)]
    fn default() -> Self {
        ConstWrapper(PhantomData)
    }
}

impl<T: ?Sized> Copy for ConstWrapper<T> {}

impl<T: ?Sized> Clone for ConstWrapper<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

//////////////////////////////////////////////////////////////////////////////////////

/// Used to wrap a type in a ConstWrapper.
pub trait AsConstWrapper {
    /// Use this if you want to initialize a field of a ConstValue.
    const CW: ConstWrapper<Self> = ConstWrapper::NEW;
    /// Use this if you want to construct a
    /// ConstWrapper<ConstValueParam> field on a struct deriving MutConstValue.
    const CW2: ConstWrapper<ConstWrapper<Self>> = ConstWrapper::NEW;
    const CW3: ConstWrapper<ConstWrapper<ConstWrapper<Self>>> = ConstWrapper::NEW;

    #[inline(always)]
    /// Creates a ConstWrapper<Self>.
    fn to_cw(&self) -> ConstWrapper<Self> {
        ConstWrapper::NEW
    }
}

impl<This: ?Sized> AsConstWrapper for This {}

type_fn!{
    /// Constructs a ConstWrapper<v> (on the type-level).
    pub fn NewConstWrapper[v:?Sized](v){ ConstWrapper<v> }
}

//////////////////////////////////////////////////////////////////////////////////////

impl<T: ?Sized> fmt::Display for ConstWrapper<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&"ConstWrapper(T)", f)
    }
}

impl<T: ?Sized> fmt::Debug for ConstWrapper<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&"ConstWrapper(T)", f)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////

impl<T: ?Sized> Eq for ConstWrapper<T> {}

impl<T: ?Sized> PartialEq for ConstWrapper<T> {
    #[inline(always)]
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
impl<T: ?Sized> Ord for ConstWrapper<T> {
    #[inline(always)]
    fn cmp(&self, _: &Self) -> cmp::Ordering {
        cmp::Ordering::Equal
    }
}
impl<T: ?Sized> PartialOrd for ConstWrapper<T> {
    #[inline(always)]
    fn partial_cmp(&self, _: &Self) -> Option<cmp::Ordering> {
        Some(cmp::Ordering::Equal)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

impl<T: ?Sized> hash::Hash for ConstWrapper<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        ().hash(state)
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<'de, T: ?Sized> Deserialize<'de> for ConstWrapper<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            <()>::deserialize(deserializer).map(|_| ConstWrapper::NEW)
        }
    }
    impl<T: ?Sized> Serialize for ConstWrapper<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            ().serialize(serializer)
        }
    }
}

impl<T> From<T> for ConstWrapper<T> {
    #[inline(always)]
    fn from(_: T) -> Self {
        ConstWrapper::NEW
    }
}
impl<T: ?Sized> Into<VariantPhantom<T>> for ConstWrapper<T> {
    #[inline(always)]
    fn into(self) -> VariantPhantom<T> {
        PhantomData
    }
}

impl<T: ?Sized, R> IntoRuntime<R> for ConstWrapper<T>
where
    T: IntoRuntime<R>,
{
    #[inline(always)]
    fn to_runtime() -> R {
        T::to_runtime()
    }
}
#[cfg(rust_1_22)]
impl<T: ?Sized, R> IntoConstant<R> for ConstWrapper<T>
where
    T: IntoConstant<R>,
{
    const VALUE: R = T::VALUE;
}

impl<T: ?Sized> InitializationValues for ConstWrapper<T>
where
    T: InitializationValues,
{
    type Uninitialized = ConstWrapper<T::Uninitialized>;
    type Initialized = ConstWrapper<T::Initialized>;
}

impl<T: ?Sized> ConstWrapper<T> {
    pub const NEW: Self = MarkerType::MTVAL;

    #[inline(always)]
    pub fn from_ty(_: VariantPhantom<T>) -> Self {
        Self::NEW
    }

    #[inline(always)]
    pub fn from_phantomdata(_: PhantomData<T>) -> Self {
        Self::NEW
    }

    #[inline(always)]
    pub fn get(self) -> T
    where
        T: MarkerType,
    {
        T::MTVAL
    }
    pub fn get_runt<Runtime>(self) -> Runtime
    where
        T: IntoRuntime<Runtime>,
    {
        T::to_runtime()
    }
    pub fn get_as<Runtime>(self, _: VariantPhantom<Runtime>) -> Runtime
    where
        T: IntoRuntime<Runtime>,
    {
        T::to_runtime()
    }

    #[inline(always)]
    /// Returns the value of the field:
    pub fn field<Field>(self, _: Field) -> GetField<T, Field>
    where
        T: GetField_<Field>,
        GetField<T, Field>: MarkerType,
    {
        MarkerType::MTVAL
    }

    #[inline(always)]
    /// Sets the `Field` field with the `Value` value.
    pub fn set_field<Field, Value>(self) -> SetField<Self, Field, Value>
    where
        T: SetField_<Field, Value>,
        SetField<Self, Field, Value>: WrapperTrait,
    {
        MarkerType::MTVAL
    }

    #[inline(always)]
    /// Sets the `Field` field with the `Value` value.
    pub fn set_field_val<Field, Value>(self, _: Field, _: Value) -> SetField<Self, Field, Value>
    where
        T: SetField_<Field, Value>,
        SetField<Self, Field, Value>: WrapperTrait,
    {
        MarkerType::MTVAL
    }

    #[inline(always)]
    /// Changes the ConstValue being wrapped.
    pub fn set<T2: ?Sized>(self) -> ConstWrapper<T2> {
        ConstWrapper::NEW
    }

    #[inline(always)]
    /// Changes the ConstValue being wrapped.
    pub fn set_val<T2>(self, _: T2) -> ConstWrapper<T2> {
        ConstWrapper::NEW
    }

    #[inline(always)]
    pub fn identity_(self, rhs: T) -> T
    where
        T: Sized,
    {
        rhs
    }
}

macro_rules! impl_map_methods {
    (
        $(#[$map_attr:meta])*
        map($map:ident constraint=[$($map_c:tt)*])

        $(#[$map_all_to_attr:meta])*
        map_all_to($map_all_to:ident constraint=[$($map_all_to_c:tt)*])

        $(#[$map_all_attr:meta])*
        map_all($map_all:ident constraint=[$($map_all_c:tt)*])
    ) => {
        impl<T> ConstWrapper<T>{

            #[inline(always)]
            $(#[$map_attr])*
            pub fn $map<Field,_Value,F>(self,_:Field,_:F)->SetField<Self,Field,_Value>
            where
                F:$($map_c)*,
                T:GetField_<Field>,
                T:SetField_<Field,_Value>,
                SetField<Self,Field,_Value>:WrapperTrait,
            {
                MarkerType::MTVAL
            }

            #[inline(always)]
            $(#[$map_all_to_attr])*
            pub fn $map_all_to<Field,_Value,F>(self,_:Field,_:F)->SetField<Self,Field,_Value>
            where
                F:$($map_all_to_c)*,
                Self:SetField_<Field,_Value>,
                SetField<Self,Field,_Value>:WrapperTrait,
            {
                MarkerType::MTVAL
            }

            #[inline(always)]
            $(#[$map_all_attr])*
            pub fn $map_all<T2,F>(self,_:F)->ConstWrapper<T2>
            where
                F:$($map_all_c)*,
                ConstWrapper<T2>:WrapperTrait,
            {
                MarkerType::MTVAL
            }
        }
    }
}

impl_map_methods!{
    /// Maps the `Field` field using the closure `F`.
    map       (map_field_fn        constraint=[FnOnce(GetField<T,Field>)->_Value])
    /// Maps the `Field` field using the closure `F`,which takes the entire value.
    map_all_to(map_to_fn constraint=[FnOnce(T)->_Value])
    /// Maps the entire value using the closure `F`.
    map_all   (map_fn    constraint=[FnOnce(T)->T2])
}
impl_map_methods!{
    /// Maps the `Field` field using the CallInto `F`.
    ///
    /// CallInto is implemented by ConstWrapper<impl TypeFn_>,which allows using any unary TypeFn_ in this function.
    map       (map_field    constraint=[CallInto<GetField<T,Field>,Returns=_Value>])
    /// Maps the `Field` field using the CallInto `F`,which takes the entire value.
    ///
    /// CallInto is implemented by ConstWrapper<impl TypeFn_>,which allows using any unary TypeFn_ in this function.
    map_all_to(map_to constraint=[CallInto<T,Returns=_Value>])
    /// Maps the entire value using the CallInto `F`.
    ///
    /// CallInto is implemented by ConstWrapper<impl TypeFn_>,which allows using any unary TypeFn_ in this function.
    map_all   (map    constraint=[CallInto<T,Returns=T2>])
}

impl<T: ?Sized> ConstTypeOf_ for ConstWrapper<T> {
    type Type = ConstWrapperType;
}
impl<T: ?Sized> IntoConstType_ for ConstWrapper<T> {
    type ToConst = ConstWrapperType;
}

impl<T: ?Sized, Field> GetField_<Field> for ConstWrapper<T>
where
    T: GetField_<Field>,
{
    type Output = T::Output;
}

impl<T: ?Sized, Runtime, Field> GetFieldRuntime_<Field, Runtime> for ConstWrapper<T>
where
    T: GetFieldRuntime_<Field, Runtime>,
{
    type Runtime = T::Runtime;
}

impl<T: ?Sized, Field, Value> SetField_<Field, Value> for ConstWrapper<T>
where
    T: SetField_<Field, Value>,
{
    type Output = ConstWrapper<T::Output>;
}

/////////////////////////////////////////////////////////////////////////////////////

impl<T: ?Sized, Params> TypeFn_<Params> for ConstWrapper<T>
where
    T: TypeFn_<Params>,
{
    type Output = T::Output;
}

impl<F, Params> CallRef<Params> for ConstWrapper<F>
where
    F: TypeFn_<Params>,
    F::Output: ConstValue,
{
    #[inline(always)]
    fn call_ref(&self, _: Params) -> F::Output {
        MarkerType::MTVAL
    }
}

impl<F, Params> CallMut<Params> for ConstWrapper<F>
where
    F: TypeFn_<Params>,
    F::Output: ConstValue,
{
    #[inline(always)]
    fn call_mut(&mut self, _: Params) -> F::Output {
        MarkerType::MTVAL
    }
}

impl<F, Params> CallInto<Params> for ConstWrapper<F>
where
    F: TypeFn_<Params>,
    F::Output: ConstValue,
{
    type Returns = F::Output;

    #[inline(always)]
    fn call_into(self, _: Params) -> F::Output {
        MarkerType::MTVAL
    }
}

/////////////////////////////////////////////////////////////////////////////////////

impl<T: ?Sized> Deref for ConstWrapper<T>
where
    T: MarkerType,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        T::markertype_ref()
    }
}

impl<T: ?Sized, Field> Index<Field> for ConstWrapper<T>
where
    T: GetField_<Field>,
{
    type Output = ConstWrapper<GetField<T, Field>>;

    #[inline(always)]
    fn index(&self, _: Field) -> &Self::Output {
        MarkerType::markertype_ref()
    }
}

/////////////////////////////////////////////////////////////////////////////////////

#[cfg(all(test, feature = "passed_tests"))]
mod tests;

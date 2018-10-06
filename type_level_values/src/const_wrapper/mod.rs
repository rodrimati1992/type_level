/*! 
Contains ConstWrapper,
for wrapping ConstValues so that they implement many std library traits.

To instantiate a ConstWrapper use either `Value::CW` or `value.to_cw()`.

*/

use prelude::*;

use crate_::user_traits::const_traits;
use crate_::discriminant::{
    Discriminant,
};
use crate_::runtime_value::{
    DerivedTraits,
};
use crate_::field_traits::{
    GetField, GetFieldRuntime, GetFieldRuntime_, GetField_, SetField, SetField_,
};
use crate_::initialization::InitializationValues;

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
pub type ConstWrapperFromTrait<ConstParam> =
    ConstWrapper<UnwrapConst<ConstParam>>;

//////////////////////////////////////////////////////////////////////////////////////

/// Const-type of all ConstWrapper values.
#[derive(Debug, Copy, Clone, Default)]
pub struct ConstWrapperType;

impl ConstType for ConstWrapperType {}

//////////////////////////////////////////////////////////////////////////////////////

/// Trait used to access the type parameters of ConstWrapper in a generic context.
pub trait WrapperTrait: Sealed + DerivedTraits<Type = ConstWrapperType>
{
    type ConstValue;
}

impl<T> WrapperTrait for ConstWrapper<T> {
    type ConstValue = T;
}

impl<T> GetDiscriminant for ConstWrapper<T>{
    type Discriminant=Discriminant<ConstWrapperType,ConstWrapperType, U0>;

    type Variant=ConstWrapperType;
}

impl<T> Sealed for ConstWrapper<T> {}

/// Gets the `ConstValue` associated type from a WrapperTrait implementor.
pub type UnwrapConst<ConstWrapper> = <ConstWrapper as WrapperTrait>::ConstValue;

//////////////////////////////////////////////////////////////////////////////////////

/// ConstWrapper type for compiletime-values,which as like a PhantomData, 
/// and delegates many trait impls to the wrapped Constant.
///
pub struct ConstWrapper<Compiletime>(VariantPhantom<Compiletime>);

unsafe impl<T> MarkerType for ConstWrapper<T> {}

unsafe impl<Compiletime> Send for ConstWrapper<Compiletime> {}
unsafe impl<Compiletime> Sync for ConstWrapper<Compiletime> {}

impl<T> Default for ConstWrapper<T> {
    #[inline(always)]
    fn default() -> Self {
        ConstWrapper(PhantomData)
    }
}

impl<T> Copy for ConstWrapper<T> {}

impl<T> Clone for ConstWrapper<T> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

//////////////////////////////////////////////////////////////////////////////////////

pub trait AsConstWrapper: Sized {
    const CW: ConstWrapper<Self> = ConstWrapper::NEW;

    #[inline(always)]
    fn to_cw(&self) -> ConstWrapper<Self> {
        ConstWrapper::NEW
    }

    #[inline(always)]
    fn pw_(&self) -> ConstWrapper<Self> {
        ConstWrapper::NEW
    }
}

impl<This> AsConstWrapper for This {}

//////////////////////////////////////////////////////////////////////////////////////

impl<T> fmt::Display for ConstWrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&"ConstWrapper(T)", f)
    }
}

impl<T> fmt::Debug for ConstWrapper<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&"ConstWrapper(T)", f)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////

impl<T> Eq for ConstWrapper<T> {}

impl<T> PartialEq for ConstWrapper<T> {
    #[inline(always)]
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
impl<T> Ord for ConstWrapper<T> {
    #[inline(always)]
    fn cmp(&self, _: &Self) -> cmp::Ordering {
        cmp::Ordering::Equal
    }
}
impl<T> PartialOrd for ConstWrapper<T> {
    #[inline(always)]
    fn partial_cmp(&self, _: &Self) -> Option<cmp::Ordering> {
        Some(cmp::Ordering::Equal)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> hash::Hash for ConstWrapper<T> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        ().hash(state)
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<'de, T> Deserialize<'de> for ConstWrapper<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            <()>::deserialize(deserializer).map(|_| Default::default())
        }
    }
    impl<T> Serialize for ConstWrapper<T> {
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
impl<T> Into<VariantPhantom<T>> for ConstWrapper<T> {
    #[inline(always)]
    fn into(self) -> VariantPhantom<T> {
        PhantomData
    }
}

impl<T, R> IntoRuntime<R> for ConstWrapper<T>
where
    T: IntoRuntime<R>,
{
    #[inline(always)]
    fn to_runtime() -> R {
        T::to_runtime()
    }
}
#[cfg(rust_1_20)]
impl<T, R> IntoConstant<R> for ConstWrapper<T>
where
    T: IntoConstant<R>,
{
    const VALUE: R = T::VALUE;
}

impl<T> InitializationValues for ConstWrapper<T>
where T:InitializationValues
{
    type Uninitialized = ConstWrapper<T::Uninitialized>;
    type Initialized = ConstWrapper<T::Initialized>;
}


impl<T> ConstWrapper<T> {
    #[inline(always)]
    pub fn to_runtime<R>(self) -> R
    where
        T: IntoRuntime<R>,
    {
        T::to_runtime()
    }
}


impl<T> ConstWrapper<T> {
    pub fn get_runt<Runtime>(self) -> Runtime
    where
        T: IntoRuntime<Runtime>,
    {
        T::to_runtime()
    }
}

impl<T> ConstWrapper<T> {
    pub const NEW: Self = MarkerType::MTVAL;

    #[inline(always)]
    pub fn get(self) -> T
    where
        T: MarkerType,
    {
        T::MTVAL
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

    /// Returns the runtime value of the field.
    pub fn field_runt<Field, R>(self, _: Field) -> GetFieldRuntime<Self, Field, R>
    where
        Self: GetFieldRuntime_<Field, R>,
        GetField<Self, Field>: IntoRuntime<GetFieldRuntime<Self, Field, R>>,
    {
        Self::get_val()
    }

    /// Returns the runtime value of the field.
    pub fn field_as<Field, Runtime>(
        self,
        _: Field,
        _: VariantPhantom<Runtime>,
    ) -> GetFieldRuntime<T, Field, Runtime>
    where
        T: GetFieldRuntime_<Field, Runtime>,
        GetField<T, Field>: IntoRuntime<GetFieldRuntime<T, Field, Runtime>>,
    {
        T::get_val()
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
    /// Changes the compile-time value being wrapped.
    pub fn set<T2>(self) -> ConstWrapper<T2> {
        ConstWrapper::NEW
    }

    #[inline(always)]
    /// Changes the compile-time value being wrapped.
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

impl<T> ConstTypeOf_ for ConstWrapper<T> {
    type Type = ConstWrapperType;
}
impl<T> IntoConstType_ for ConstWrapper<T> {
    type ToConst = ConstWrapperType;
}

impl<T, Field> GetField_<Field> for ConstWrapper<T>
where
    T: GetField_<Field>,
{
    type Output = T::Output;
}

impl<T, Runtime, Field> GetFieldRuntime_<Field, Runtime> for ConstWrapper<T>
where
    T: GetFieldRuntime_<Field, Runtime>,
{
    type Runtime = T::Runtime;
}

impl<T, Field, Value> SetField_<Field, Value> for ConstWrapper<T>
where
    T: SetField_<Field, Value>,
{
    type Output = ConstWrapper<T::Output>;
}

/////////////////////////////////////////////////////////////////////////////////////

impl<T, Params> TypeFn_<Params> for ConstWrapper<T>
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

impl<T> Deref for ConstWrapper<T>
where
    T: MarkerType,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        T::markertype_ref()
    }
}

impl<T, Field> Index<Field> for ConstWrapper<T>
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

/// The ConstConstructor for a ConstWrapper.
pub struct ConstWrapperCC;

impl<T> const_traits::GetConstParam_ for ConstWrapper<T> {
    type Const = T;
}
impl<T> const_traits::GetConstConstructor_ for ConstWrapper<T> {
    type Constructor = ConstWrapperCC;
}

impl const_traits::ConstConstructor for ConstWrapperCC {}

impl<T> const_traits::ApplyConstParam_<T> for ConstWrapperCC {
    type Applied = ConstWrapper<T>;
}

/////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests;

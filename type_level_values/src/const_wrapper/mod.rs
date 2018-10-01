use prelude::*;

use crate_::user_traits::const_traits;
use crate_::enum_stuff::{
    Discriminant,
};
use crate_::runtime_value::{
    DerivedTraits,
};
use crate_::field_traits::{
    GetField, GetFieldRuntime, GetFieldRuntime_, GetField_, SetField, SetField_,
};
use crate_::field_traits::initialization::InitializationValues;

use crate_::ops::TypeFn_;
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

/// Type-level enum which represents which kind of ConstWrapper [ConstWrapper] is.
pub trait WrapperKindType: Sealed {}

impl<R> Sealed for RuntimeKind<R> {}
impl<R> WrapperKindType for RuntimeKind<R> {}

impl Sealed for PhantomKind {}
impl WrapperKindType for PhantomKind {}

/// Requires that ConstWrapper uses the runtime value of the type whenever possible.
pub struct RuntimeKind<R>(R);

/// Requires that ConstWrapper act as a PhantomData.
pub struct PhantomKind;

/// A ConstWrapper alias,used for compile-time values, which implements many traits.
/// <br>
/// Some impls for this are on [ConstWrapper<Compiletime,PhantomKind>](./struct.ConstWrapper.html).
///
/// # Construction
///
/// When constructing a PhantomWrapper inside a function
/// that returns it prefer using PhantomWrapper::NEW.
///
/// When constructing a PhantomWrapper whose type can't be inferred use Type::PW or <Type>::PW.
///
/// When there is a value that needs to be converted to a PhantomWrapper
/// either use value.to_pw() or
/// value.into() (if the type is Copy and the target type is PhantomWrapper<_>),
///
///
///
pub type PhantomWrapper<Compiletime> = ConstWrapper<Compiletime, PhantomKind>;

/// A ConstWrapper type for compile-time values which implements many traits
/// using the runtime type.
pub type AsRuntime<Compiletime, Runtime> = ConstWrapper<Compiletime, RuntimeKind<Runtime>>;

/// Creates a ConstWrapper from a generic type which implements WrapperTrait.
pub type ConstWrapperFromTrait<ConstParam> =
    ConstWrapper<GetConstValue<ConstParam>, GetWrapperKind<ConstParam>>;

//////////////////////////////////////////////////////////////////////////////////////

/// Const-type of all ConstWrapper values.
#[derive(Debug, Copy, Clone, Default)]
pub struct WrapperType;

impl ConstType for WrapperType {}

type_fn!{
    pub fn AssertValidKind(PhantomKind){()}
           AssertValidKind[R](RuntimeKind<R>){()}
}

//////////////////////////////////////////////////////////////////////////////////////

/// Trait used to access the type parameters of ConstWrapper in a generic context.
pub trait WrapperTrait: Sealed + DerivedTraits<Type = WrapperType>
{
    type ConstValue;
    type Kind;
}

impl<T, Kind> WrapperTrait for ConstWrapper<T, Kind> {
    type ConstValue = T;
    type Kind = Kind;
}

impl<T,K> GetDiscriminant for ConstWrapper<T,K>{
    type Discriminant=Discriminant<names::ConstWrapper_Type,WrapperType, U0>;

    type Variant=names::ConstWrapper_Type;
}

mod names{
    pub struct ConstWrapper_Type;
}


impl<T, Kind> Sealed for ConstWrapper<T, Kind> {}

/// Gets the `ConstValue` associated type from a WrapperTrait implementor.
pub type GetConstValue<ConstWrapper> = <ConstWrapper as WrapperTrait>::ConstValue;

/// Gets the `Kind` associated type from a WrapperTrait implementor.
pub type GetWrapperKind<ConstWrapper> = <ConstWrapper as WrapperTrait>::Kind;

//////////////////////////////////////////////////////////////////////////////////////

/// ConstWrapper type for compiletime-values.
///
/// This Type has 2 flavours:
///
/// -[PhantomWrapper]:Which as like a PhantomData
///
/// -[AsRuntime]:Which acts like the runtime version of the value.
///
///
pub struct ConstWrapper<Compiletime, WrapperKind>(VariantPhantom<(Compiletime, WrapperKind)>);

unsafe impl<T, K> MarkerType for ConstWrapper<T, K> {}

unsafe impl<Compiletime, WrapperKind> Send for ConstWrapper<Compiletime, WrapperKind> {}
unsafe impl<Compiletime, WrapperKind> Sync for ConstWrapper<Compiletime, WrapperKind> {}

impl<T, Kind> Default for ConstWrapper<T, Kind> {
    #[inline(always)]
    fn default() -> Self {
        ConstWrapper(PhantomData)
    }
}

impl<T, Kind> Copy for ConstWrapper<T, Kind> {}

impl<T, Kind> Clone for ConstWrapper<T, Kind> {
    #[inline(always)]
    fn clone(&self) -> Self {
        *self
    }
}

//////////////////////////////////////////////////////////////////////////////////////

pub trait AsConstWrapper: Sized {
    const PW: PhantomWrapper<Self> = ConstWrapper::NEW;

    #[inline(always)]
    fn to_pw(&self) -> PhantomWrapper<Self> {
        ConstWrapper::NEW
    }

    #[inline(always)]
    fn to_ar<R>(&self) -> AsRuntime<Self, R> {
        ConstWrapper::NEW
    }

    #[inline(always)]
    fn pw_(&self) -> PhantomWrapper<Self> {
        ConstWrapper::NEW
    }

    #[inline(always)]
    fn ar_<R>(&self) -> AsRuntime<Self, R> {
        ConstWrapper::NEW
    }
}

impl<This> AsConstWrapper for This {}

//////////////////////////////////////////////////////////////////////////////////////

impl<T, R> fmt::Display for ConstWrapper<T, RuntimeKind<R>>
where
    T: IntoRuntime<R>,
    R: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&T::to_runtime(), f)
    }
}

impl<T> fmt::Display for ConstWrapper<T, PhantomKind> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&"ConstWrapper(T,R,PhantomKind)", f)
    }
}

impl<T, R> fmt::Debug for ConstWrapper<T, RuntimeKind<R>>
where
    T: IntoRuntime<R>,
    R: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(&T::to_runtime(), f)
    }
}

impl<T> fmt::Debug for ConstWrapper<T, PhantomKind> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&"ConstWrapper(T,R,PhantomKind)", f)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////

impl<T> Eq for PhantomWrapper<T> {}

impl<T> PartialEq for PhantomWrapper<T> {
    #[inline(always)]
    fn eq(&self, _: &Self) -> bool {
        true
    }
}
impl<T> Ord for PhantomWrapper<T> {
    #[inline(always)]
    fn cmp(&self, _: &Self) -> cmp::Ordering {
        cmp::Ordering::Equal
    }
}
impl<T> PartialOrd for PhantomWrapper<T> {
    #[inline(always)]
    fn partial_cmp(&self, _: &Self) -> Option<cmp::Ordering> {
        Some(cmp::Ordering::Equal)
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////

impl<T, R> Eq for AsRuntime<T, R> where Self: PartialEq<Self> {}

impl<T1, T2, R1, R2> PartialEq<AsRuntime<T2, R2>> for AsRuntime<T1, R1>
where
    T1: IntoRuntime<R1>,
    T2: IntoRuntime<R2>,
    R1: PartialEq<R2>,
{
    fn eq(&self, _: &AsRuntime<T2, R2>) -> bool {
        T1::to_runtime().eq(&T2::to_runtime())
    }
}
impl<T, R> Ord for AsRuntime<T, R>
where
    T: IntoRuntime<R>,
    R: Ord,
{
    #[inline(always)]
    fn cmp(&self, _: &Self) -> cmp::Ordering {
        cmp::Ordering::Equal
    }
}
impl<T1, T2, R1, R2> PartialOrd<AsRuntime<T2, R2>> for AsRuntime<T1, R1>
where
    T1: IntoRuntime<R1>,
    T2: IntoRuntime<R2>,
    R1: PartialOrd<R2>,
{
    fn partial_cmp(&self, _: &AsRuntime<T2, R2>) -> Option<cmp::Ordering> {
        T1::to_runtime().partial_cmp(&T2::to_runtime())
    }
}

/////////////////////////////////////////////////////////////////////////////////////////////////

impl<T> hash::Hash for ConstWrapper<T, PhantomKind> {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        ().hash(state)
    }
}

impl<T, R> hash::Hash for ConstWrapper<T, RuntimeKind<R>>
where
    T: IntoRuntime<R>,
    R: hash::Hash,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        let v = T::to_runtime();
        v.hash(state)
    }
}

#[cfg(feature = "serde")]
mod serde_impl {
    use super::*;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl<'de, T, R> Deserialize<'de> for ConstWrapper<T, RuntimeKind<R>>
    where
        R: Deserialize<'de>,
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let _ = R::deserialize(deserializer)?;
            Ok(Default::default())
        }
    }
    impl<T, R> Serialize for ConstWrapper<T, RuntimeKind<R>>
    where
        T: IntoRuntime<R>,
        R: Serialize,
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            T::to_runtime().serialize(serializer)
        }
    }

    impl<'de, T> Deserialize<'de> for PhantomWrapper<T> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            <()>::deserialize(deserializer).map(|_| Default::default())
        }
    }
    impl<T> Serialize for PhantomWrapper<T> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            ().serialize(serializer)
        }
    }
}

impl<T> From<T> for ConstWrapper<T, PhantomKind> {
    #[inline(always)]
    fn from(_: T) -> Self {
        ConstWrapper::NEW
    }
}
impl<T, R> From<T> for ConstWrapper<T, RuntimeKind<R>> {
    #[inline(always)]
    fn from(_: T) -> Self {
        ConstWrapper::NEW
    }
}
impl<T, R> From<ConstWrapper<T, RuntimeKind<R>>> for ConstWrapper<T, PhantomKind> {
    #[inline(always)]
    fn from(_: ConstWrapper<T, RuntimeKind<R>>) -> Self {
        ConstWrapper::NEW
    }
}
impl<T, R> From<ConstWrapper<T, PhantomKind>> for ConstWrapper<T, RuntimeKind<R>> {
    #[inline(always)]
    fn from(_: ConstWrapper<T, PhantomKind>) -> Self {
        ConstWrapper::NEW
    }
}

impl<T, K> Into<VariantPhantom<T>> for ConstWrapper<T, K> {
    #[inline(always)]
    fn into(self) -> VariantPhantom<T> {
        PhantomData
    }
}

impl<T, R> IntoRuntime<R> for PhantomWrapper<T>
where
    T: IntoRuntime<R>,
{
    #[inline(always)]
    fn to_runtime() -> R {
        T::to_runtime()
    }
}

impl<T, R> IntoRuntime<R> for AsRuntime<T, R>
where
    T: IntoRuntime<R>,
{
    #[inline(always)]
    fn to_runtime() -> R {
        T::to_runtime()
    }
}

#[cfg(rust_1_20)]
impl<T, R> IntoConstant<R> for PhantomWrapper<T>
where
    T: IntoConstant<R>,
{
    const VALUE: R = T::VALUE;
}

#[cfg(rust_1_20)]
impl<T, R> IntoConstant<R> for AsRuntime<T, R>
where
    T: IntoConstant<R>,
{
    const VALUE: R = T::VALUE;
}


impl<T,K> InitializationValues for ConstWrapper<T,K>
where T:InitializationValues
{
    type Uninitialized = ConstWrapper<T::Uninitialized,K>;
    type Initialized = ConstWrapper<T::Initialized,K>;
}


impl<T> ConstWrapper<T, PhantomKind> {
    #[inline(always)]
    pub fn to_runtime<R>(self) -> R
    where
        T: IntoRuntime<R>,
    {
        T::to_runtime()
    }
}

impl<T, R> AsRuntime<T, R> {
    pub fn get_runt(self) -> R
    where
        T: IntoRuntime<R>,
    {
        T::to_runtime()
    }
}

impl<T> PhantomWrapper<T> {
    pub fn get_runt<Runtime>(self) -> Runtime
    where
        T: IntoRuntime<Runtime>,
    {
        T::to_runtime()
    }
}

impl<T, Kind> ConstWrapper<T, Kind> {
    pub const NEW: Self = MarkerType::MTVAL;

    #[inline(always)]
    pub fn as_runtime<R>(self) -> AsRuntime<T, R> {
        AsRuntime::NEW
    }

    #[inline(always)]
    pub fn as_phantom(self) -> ConstWrapper<T, PhantomKind> {
        PhantomWrapper::NEW
    }

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
    pub fn set<T2>(self) -> ConstWrapper<T2, Kind> {
        ConstWrapper::NEW
    }

    #[inline(always)]
    /// Changes the compile-time value being wrapped.
    pub fn set_val<T2>(self, _: T2) -> ConstWrapper<T2, Kind> {
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
        impl<T,Kind> ConstWrapper<T,Kind>{

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
            pub fn $map_all<T2,F>(self,_:F)->ConstWrapper<T2,Kind>
            where
                F:$($map_all_c)*,
                ConstWrapper<T2,Kind>:WrapperTrait,
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
    /// CallInto is implemented by PhantomWrapper<impl TypeFn_>,which allows using any unary TypeFn_ in this function.
    map       (map_field    constraint=[CallInto<GetField<T,Field>,Returns=_Value>])
    /// Maps the `Field` field using the CallInto `F`,which takes the entire value.
    ///
    /// CallInto is implemented by PhantomWrapper<impl TypeFn_>,which allows using any unary TypeFn_ in this function.
    map_all_to(map_to constraint=[CallInto<T,Returns=_Value>])
    /// Maps the entire value using the CallInto `F`.
    ///
    /// CallInto is implemented by PhantomWrapper<impl TypeFn_>,which allows using any unary TypeFn_ in this function.
    map_all   (map    constraint=[CallInto<T,Returns=T2>])
}

impl<T, Kind> ConstTypeOf_ for ConstWrapper<T, Kind> {
    type Type = WrapperType;
}
impl<T, Kind> IntoConstType_ for ConstWrapper<T, Kind> {
    type ToConst = WrapperType;
}

impl<T, Kind, Field> GetField_<Field> for ConstWrapper<T, Kind>
where
    T: GetField_<Field>,
{
    type Output = T::Output;
}

impl<T, Runtime, Field> GetFieldRuntime_<Field, Runtime> for AsRuntime<T, Runtime>
where
    T: GetFieldRuntime_<Field, Runtime>,
{
    type Runtime = T::Runtime;
}

impl<T, Runtime, Field> GetFieldRuntime_<Field, Runtime> for PhantomWrapper<T>
where
    T: GetFieldRuntime_<Field, Runtime>,
{
    type Runtime = T::Runtime;
}

impl<T, Kind, Field, Value> SetField_<Field, Value> for ConstWrapper<T, Kind>
where
    T: SetField_<Field, Value>,
{
    type Output = ConstWrapper<T::Output, Kind>;
}

/////////////////////////////////////////////////////////////////////////////////////

impl<T, Kind, Params> TypeFn_<Params> for ConstWrapper<T, Kind>
where
    T: TypeFn_<Params>,
{
    type Output = T::Output;
}

impl<F, Params> CallRef<Params> for PhantomWrapper<F>
where
    F: TypeFn_<Params>,
    F::Output: ConstValue,
{
    #[inline(always)]
    fn call_ref(&self, _: Params) -> F::Output {
        MarkerType::MTVAL
    }
}

impl<F, Params> CallMut<Params> for PhantomWrapper<F>
where
    F: TypeFn_<Params>,
    F::Output: ConstValue,
{
    #[inline(always)]
    fn call_mut(&mut self, _: Params) -> F::Output {
        MarkerType::MTVAL
    }
}

impl<F, Params> CallInto<Params> for PhantomWrapper<F>
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

impl<F, FR, Params> CallRef<Params> for AsRuntime<F, FR>
where
    F: IntoRuntime<FR>,
    FR: CallRef<Params>,
{
    fn call_ref(&self, params: Params) -> FR::Returns {
        F::to_runtime().call_ref(params)
    }
}

impl<F, FR, Params> CallMut<Params> for AsRuntime<F, FR>
where
    F: IntoRuntime<FR>,
    FR: CallMut<Params>,
{
    fn call_mut(&mut self, params: Params) -> FR::Returns {
        F::to_runtime().call_mut(params)
    }
}

impl<F, FR, Params> CallInto<Params> for AsRuntime<F, FR>
where
    F: IntoRuntime<FR>,
    FR: CallInto<Params>,
{
    type Returns = FR::Returns;
    fn call_into(self, params: Params) -> FR::Returns {
        F::to_runtime().call_into(params)
    }
}


/////////////////////////////////////////////////////////////////////////////////////

impl<T, Kind> Deref for ConstWrapper<T, Kind>
where
    T: MarkerType,
{
    type Target = T;

    #[inline(always)]
    fn deref(&self) -> &T {
        T::markertype_ref()
    }
}

impl<T, Field> Index<Field> for PhantomWrapper<T>
where
    T: GetField_<Field>,
{
    type Output = PhantomWrapper<GetField<T, Field>>;

    #[inline(always)]
    fn index(&self, _: Field) -> &Self::Output {
        MarkerType::markertype_ref()
    }
}

impl<T, R, Field> Index<Field> for AsRuntime<T, R>
where
    T: GetField_<Field>,
    T: GetFieldRuntime_<Field, R>,
{
    type Output = AsRuntime<GetField<T, Field>, GetFieldRuntime<T, Field, R>>;

    #[inline(always)]
    fn index(&self, _: Field) -> &Self::Output {
        MarkerType::markertype_ref()
    }
}

/////////////////////////////////////////////////////////////////////////////////////

/// The ConstConstructor for a ConstWrapper.
pub struct ConstWrapperCC<K>(VariantPhantom<K>);

impl<T, K> const_traits::GetConstParam_ for ConstWrapper<T, K> {
    type Const = T;
}
impl<T, K> const_traits::GetConstConstructor_ for ConstWrapper<T, K> {
    type Constructor = ConstWrapperCC<K>;
}

impl<K> const_traits::ConstConstructor for ConstWrapperCC<K> {}

impl<T, K> const_traits::ApplyConstParam_<T> for ConstWrapperCC<K> {
    type Applied = ConstWrapper<T, K>;
}

/////////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests;
